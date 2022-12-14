extern crate derp;
extern crate untrusted;

use base64::decode;
use derp::Tag;
use untrusted::Input;

// Content is taken from a EC private key:
// -----BEGIN EC PRIVATE KEY-----
// MHcCAQEEIFjdJCw6Lkx1VPYtdnbihRKoLFb36zzAc0XgCJ1B5/oPoAoGCCqGSM49
// AwEHoUQDQgAEY62fGO3T7D69Hmu58+/QcDAXB30Wzh84kXRBNviAkNyUf5hVVXcH
// /FwFtJ6S7P7snrB9BrhLWRIG7X6POF2CJw==
// -----END EC PRIVATE KEY-----
const EC_PRIVATE_KEY_FORMAT: &'static str = "MHcCAQEEIFjdJCw6Lkx1VPYtdnbihRKoLFb36zzAc0XgCJ1B5/oPoAoGCCqGSM49AwEHoUQDQgAEY62fGO3T7D69Hmu58+/QcDAXB30Wzh84kXRBNviAkNyUf5hVVXcH/FwFtJ6S7P7snrB9BrhLWRIG7X6POF2CJw==";

const PRIME_256_V1: &'static [u8] = &[0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];

fn main() {
    println!("base64: {:?}", &EC_PRIVATE_KEY_FORMAT);
    let decoded = decode(&EC_PRIVATE_KEY_FORMAT).unwrap();
    println!("decoded: {:02x?}", decoded);
    let input = Input::from(&decoded);
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

                println!("curve_oid: {:02x?}", curve_oid);
                println!("curve_oid: {:?}", curve_oid);
                let _param = derp::read_tag_and_get_value(input)?;
                println!("param: {:?}", param);
                Ok((octet_string, curve_oid))
            })?;
            Ok((octet_string, curve_oid))
        })
        .unwrap();
    println!("-----------------------------");
    println!("priv_key: {:02x?}", octet_string.as_slice_less_safe());
    if curve_oid == PRIME_256_V1 {
        println!("curve_oid: {:02x?} {}", curve_oid, "prime256v1");
    }
}
