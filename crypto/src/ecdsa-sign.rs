use base64;
use ring::{
    rand,
    signature::{self, ECDSA_P256_SHA256_ASN1_SIGNING},
};
use sigstore::crypto::signing_key::SigStoreKeyPair;
use std::fs;

fn main() {
    // This example expects a key that was generated using cosign's
    // generate key-pair
    // The private key generated will be encrypted using scrypt and we have
    // to decrypt it first before using it.
    let bytes = fs::read("cosign.key").unwrap();
    println!("Ecdsa sign example");

    let r = SigStoreKeyPair::from_encrypted_pem(&bytes, "test".as_bytes());
    let key_store_pair: SigStoreKeyPair = r.unwrap();
    println!("Decrypted type {}", key_store_pair.to_string());
    let private_key_pem = key_store_pair.private_key_to_pem().unwrap();
    println!("private_key_pem: {:?}", private_key_pem);

    let signer = key_store_pair
        .to_sigstore_signer(&sigstore::crypto::SigningScheme::ECDSA_P256_SHA256_ASN1)
        .unwrap();

    let artifact = fs::read("artifact.txt").unwrap();
    let sig = signer.sign(&artifact).unwrap();
    println!("Signature hex: {:02x?}\n", &sig);
    println!("Signature dec: {:?}\n", &sig);
    let base64_encoded = base64::encode(sig);
    println!("Signature base64: {:?}\n", &base64_encoded);

    /*
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
    */
}
