use base64::{engine::general_purpose::STANDARD as BASE64_STD_ENGINE, Engine as _};
use base64::{engine::general_purpose::STANDARD_NO_PAD as BASE64_STD_NO_PAD_ENGINE, Engine as _};
use derp::Tag;
use pem::parse;
use sha2::Digest;
use sha2::Sha256;
use ssh_key::public::{EcdsaPublicKey, KeyData};
use ssh_key::sec1::EncodedPoint;
use untrusted::Input;

fn main() {
    let public_key = r#"-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEqiLuArRcZCY1s650rgKUDpj7f+b8
9HMu3K/PDaUcR9kcyyXY8q6U+TFTkc9u84wJTsZe21wBPd/STPEzo0JrzQ==
-----END PUBLIC KEY-----"#;
    println!("public_key: {}", public_key);

    let p = parse(public_key).unwrap();
    let (bytes, _ecc_oid, _curve_oid) = read_public_key(p.contents());
    println!("public_key bytes: {:?}", &bytes[1..]);
    let keydata = KeyData::Ecdsa(EcdsaPublicKey::NistP256(
        EncodedPoint::from_bytes(&bytes).unwrap(),
    ));
    println!(
        "calculated fingerprint: {}",
        keydata.fingerprint(ssh_key::HashAlg::Sha256)
    );

    // The below is just to verify that the public key data is only the second
    // element which is base64 encoded.
    // ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBKoi7gK0XGQmNbOudK4ClA6Y+3/m/PRzLtyvzw2lHEfZHMsl2PKulPkxU5HPbvOMCU7GXttcAT3f0kzxM6NCa80=
    let key_base64 = "AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBKoi7gK0XGQmNbOudK4ClA6Y+3/m/PRzLtyvzw2lHEfZHMsl2PKulPkxU5HPbvOMCU7GXttcAT3f0kzxM6NCa80=";
    let decoded = BASE64_STD_ENGINE.decode(key_base64).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(decoded);
    let result = hasher.finalize();
    let encoded = BASE64_STD_NO_PAD_ENGINE.encode(result);
    println!("encoded fingerprint: {:?}", encoded);
}

fn read_public_key(bytes: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let input = Input::from(bytes);
    let (bit_string, ecc_oid, curve_oid) = input
        .read_all(derp::Error::Read, |input| {
            let (bit_string, ecc_oid, curve_oid) = derp::nested(input, Tag::Sequence, |input| {
                let (ecc_oid, curve_oid) = derp::nested(input, Tag::Sequence, |input| {
                    let ecc_oid = derp::expect_tag_and_get_value(input, Tag::Oid)?;
                    let curve_oid = derp::expect_tag_and_get_value(input, Tag::Oid)?;
                    Ok((ecc_oid, curve_oid))
                })?;
                let bit_string = derp::bit_string_with_no_unused_bits(input)?;
                //println!("bit_string: {:?}", bit_string);
                Ok((bit_string.as_slice_less_safe(), ecc_oid, curve_oid))
            })?;
            Ok((
                bit_string,
                ecc_oid.as_slice_less_safe(),
                curve_oid.as_slice_less_safe(),
            ))
        })
        .unwrap();
    (bit_string.to_vec(), ecc_oid.to_vec(), curve_oid.to_vec())
}
