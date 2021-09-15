use sha2::{Sha256, Digest};
use hkdf::{Hkdf};

fn digest_example() {
    let mut hasher = Sha256::new();
    hasher.update(b"bajja");
    let hash = hasher.finalize();
    println!("{:x}", hash);

    // Digest will do the above for us, that is create the hasher, call update
    // and then finalize.
    println!("{:x}", sha2::Sha256::digest(b"bajja"));
}

fn hkdf_extract_example<'a>(salt: &str, ikm: &str) -> (Vec<u8>, Hkdf<Sha256>) {
    let salt_hex = hex::decode(salt).unwrap();
    let ikm_hex = hex::decode(ikm).unwrap();

    let (prk, hkdf) = Hkdf::<Sha256>::extract(Some(&salt_hex), &ikm_hex);
    let s = prk.as_slice();
    println!("oneshot_res: {:?}", prk);
    (s.to_owned(), hkdf)

}

fn hkdf_expand_example(hkdf: Hkdf<Sha256>, prk: &Vec<u8>) {
    let mut okm = [0u8; 42];
    hkdf.expand(&prk, &mut okm).unwrap();
    println!("Output Key Material: {}", hex::encode(&okm[..]));
}

fn main() {
    digest_example();
    let salt = "00000000000000000000000000";
    let ikm = "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";

    let (prk, hkdf) = hkdf_extract_example(&salt, &ikm);
    println!("prk: {:?}", prk);
    hkdf_expand_example(hkdf, &prk);
}
