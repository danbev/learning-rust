use sha2::{Sha256, Digest};
use hkdf::{Hkdf, HkdfExtract};

fn digest_example() {
    let mut hasher = Sha256::new();
    hasher.update(b"bajja");
    let hash = hasher.finalize();
    println!("{:x}", hash);

    // Digest will do the above for us, that is create the hasher, call update
    // and then finalize.
    println!("{:x}", sha2::Sha256::digest(b"bajja"));
}

fn hkdf_extract_example() -> Hkdf<Sha256> {
    let mut extract_ctx = HkdfExtract::<Sha256>::new(Some(b"mysalt"));
    extract_ctx.input_ikm(b"hello");
    extract_ctx.input_ikm(b" world");
    let (streamed_res, hkdf) = extract_ctx.finalize();
    println!("Expanded: {:x}", streamed_res);

    let (oneshot_res, _) = Hkdf::<Sha256>::extract(Some(b"mysalt"), b"hello world");
    assert_eq!(streamed_res, oneshot_res);
    hkdf

}

fn hkdf_expand_example(hkdf: Hkdf<Sha256>) {
    // Our initial key matrial which we want to expand
    //let ikm = hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
    //let salt = hex::decode("00000000000000000000000000").unwrap();
    let info = hex::decode("f0f1f2f3f4f5f6f7f8f9").unwrap();

    //let h = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
    let mut okm = [0u8; 42];
    hkdf.expand(&info, &mut okm).unwrap();
    println!("Output Key Material: {}", hex::encode(&okm[..]));
}

fn main() {
    digest_example();
    let hkdf = hkdf_extract_example();
    hkdf_expand_example(hkdf);
}
