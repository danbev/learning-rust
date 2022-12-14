extern crate derp;
extern crate untrusted;

use base64::decode;
use derp::Tag;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_ASN1_SIGNING};
use untrusted::Input;

// Content is taken from a EC private key:
// -----BEGIN EC PRIVATE KEY-----
// MHcCAQEEIFjdJCw6Lkx1VPYtdnbihRKoLFb36zzAc0XgCJ1B5/oPoAoGCCqGSM49
// AwEHoUQDQgAEY62fGO3T7D69Hmu58+/QcDAXB30Wzh84kXRBNviAkNyUf5hVVXcH
// /FwFtJ6S7P7snrB9BrhLWRIG7X6POF2CJw==
// -----END EC PRIVATE KEY-----
const EC_PRIVATE_KEY_FORMAT: &'static str = "MHcCAQEEIFjdJCw6Lkx1VPYtdnbihRKoLFb36zzAc0XgCJ1B5/oPoAoGCCqGSM49AwEHoUQDQgAEY62fGO3T7D69Hmu58+/QcDAXB30Wzh84kXRBNviAkNyUf5hVVXcH/FwFtJ6S7P7snrB9BrhLWRIG7X6POF2CJw==";

const EC_PUBLIC_KEY_FORMAT: &'static str = "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEY62fGO3T7D69Hmu58+/QcDAXB30Wzh84kXRBNviAkNyUf5hVVXcH/FwFtJ6S7P7snrB9BrhLWRIG7X6POF2CJw==";

/*
$ openssl ec -pubin -in temp.pem -noout -text
read EC key
Public-Key: (256 bit)
pub:
    04:63:ad:9f:18:ed:d3:ec:3e:bd:1e:6b:b9:f3:ef:
    d0:70:30:17:07:7d:16:ce:1f:38:91:74:41:36:f8:
    80:90:dc:94:7f:98:55:55:77:07:fc:5c:05:b4:9e:
    92:ec:fe:ec:9e:b0:7d:06:b8:4b:59:12:06:ed:7e:
    8f:38:5d:82:27
ASN1 OID: prime256v1
NIST CURVE: P-256
*/
const PUBLIC_KEY_BYTES: &'static [u8] = &[
    0x04, 0x63, 0xad, 0x9f, 0x18, 0xed, 0xd3, 0xec, 0x3e, 0xbd, 0x1e, 0x6b, 0xb9, 0xf3, 0xef, 0xd0,
    0x70, 0x30, 0x17, 0x07, 0x7d, 0x16, 0xce, 0x1f, 0x38, 0x91, 0x74, 0x41, 0x36, 0xf8, 0x80, 0x90,
    0xdc, 0x94, 0x7f, 0x98, 0x55, 0x55, 0x77, 0x07, 0xfc, 0x5c, 0x05, 0xb4, 0x9e, 0x92, 0xec, 0xfe,
    0xec, 0x9e, 0xb0, 0x7d, 0x06, 0xb8, 0x4b, 0x59, 0x12, 0x06, 0xed, 0x7e, 0x8f, 0x38, 0x5d, 0x82,
    0x27,
];

const PRIME_256_V1: &'static [u8] = &[0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];

fn read_private_key(bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let input = Input::from(bytes);
    let (octet_string, curve_oid) = input
        .read_all(derp::Error::Read, |input| {
            let (octet_string, curve_oid) = derp::nested(input, Tag::Sequence, |input| {
                let version = derp::expect_tag_and_get_value(input, Tag::Integer)?;
                println!("version: {:?}", version.as_slice_less_safe());
                let octet_string = derp::expect_tag_and_get_value(input, Tag::OctetString)?;
                println!("octet_string : {:02x?}", octet_string.as_slice_less_safe());

                let param = derp::read_tag_and_get_value(input)?;
                println!("param: {:?}", param);
                let curve_oid = param.1.as_slice_less_safe();
                let _param = derp::read_tag_and_get_value(input)?;
                println!("param: {:?}", param);
                Ok((octet_string.as_slice_less_safe(), curve_oid))
            })?;
            Ok((octet_string, curve_oid))
        })
        .unwrap();
    (octet_string.to_vec(), curve_oid.to_vec())
}

fn read_public_key(bytes: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let input = Input::from(bytes);
    let (bit_string, ecc_oid, curve_oid) = input
        .read_all(derp::Error::Read, |input| {
            let (bit_string, ecc_oid, curve_oid) = derp::nested(input, Tag::Sequence, |input| {
                let (ecc_oid, curve_oid) = derp::nested(input, Tag::Sequence, |input| {
                    let ecc_oid = derp::expect_tag_and_get_value(input, Tag::Oid)?;
                    println!("ecc_oid value: {:?}", ecc_oid);
                    let curve_oid = derp::expect_tag_and_get_value(input, Tag::Oid)?;
                    println!("curve_oid value: {:?}", curve_oid);
                    Ok((ecc_oid, curve_oid))
                })?;
                println!("input: {:?}", input);
                let bit_string = derp::bit_string_with_no_unused_bits(input)?;
                println!("bit_string: {:?}", bit_string);
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

fn main() {
    // Parse the private key pem.
    println!("base64 private_key: {:?}", &EC_PRIVATE_KEY_FORMAT);
    let decoded = decode(&EC_PRIVATE_KEY_FORMAT).unwrap();
    let (octet_string, curve_oid) = read_private_key(&decoded);
    println!("priv_key: {:02x?}", octet_string);
    if curve_oid == PRIME_256_V1 {
        println!("curve_oid: {:02x?} {}", curve_oid, "prime256v1");
    }

    // Parse the public key pem.
    println!("public_pem: {:?}", &EC_PUBLIC_KEY_FORMAT);
    let decoded = decode(&EC_PUBLIC_KEY_FORMAT).unwrap();
    println!("decoded public_key: {:?}", &decoded);
    let (public_key, _ecc_oid, _curve_oid) = read_public_key(&decoded);
    println!("public_key: {:?}", public_key);

    // Generate the keypair.
    let key_pair: EcdsaKeyPair = EcdsaKeyPair::from_private_key_and_public_key(
        &ECDSA_P256_SHA256_ASN1_SIGNING,
        &octet_string,
        //&PUBLIC_KEY_BYTES,
        &public_key,
    )
    .unwrap();
    println!("keypair: {:02x?}", &key_pair);
}
