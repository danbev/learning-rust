use ring::{
    rand,
    signature::{self, KeyPair, ECDSA_P256_SHA256_ASN1_SIGNING},
};

fn main() {
    println!("Ecdsa example");
    let rng = rand::SystemRandom::new();
    // PKCS8 is standard syntax for storing private key inforamation.
    // Note that ECDSA_P256_SHA256_ASN1_SIGNING is a struct which looks like
    // this:
    // pub static ECDSA_P256_SHA256_ASN1_SIGNING: EcdsaSigningAlgorithm = EcdsaSigningAlgorithm {
    //   curve: &ec::suite_b::curve::P256,
    //   private_scalar_ops: &p256::PRIVATE_SCALAR_OPS,
    //   private_key_ops: &p256::PRIVATE_KEY_OPS,
    //   digest_alg: &digest::SHA256,
    //   pkcs8_template: &EC_PUBLIC_KEY_P256_PKCS8_V1_TEMPLATE,
    //   format_rs: format_rs_asn1,
    //   id: AlgorithmID::ECDSA_P256_SHA256_ASN1_SIGNING,
    // };
    let pkcs8_doc =
        // returns a serialized PKCS#8 document
        signature::EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &rng).unwrap();
    println!("PKCS#8: {:?}\n", pkcs8_doc.as_ref());

    let bytes = pkcs8_doc.as_ref();
    let key_pair =
        // Returns a EcdsaKeyPair.
        signature::EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, bytes).unwrap();

    const MSG: &[u8] = b"Fletch";
    let sig = key_pair.sign(&rand::SystemRandom::new(), MSG).unwrap();
    println!("Signature: {:?}\n", &sig.as_ref());

    let public_key = key_pair.public_key();
    println!("{:?}", public_key);

    let peer_public_key =
        signature::UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_ASN1, public_key.as_ref());

    let verified = peer_public_key.verify(MSG, sig.as_ref());
    if verified.is_ok() {
        println!("Successful verification!");
    } else {
        println!("Verification failed!");
    }
}
