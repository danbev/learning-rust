fn main() {
    println!("cargo:rerun-if-changed=cfunctions.c");
    println!("cargo:rustc-link-search=.");
    cc::Build::new()
        .file("cfunctions.c")
        .shared_flag(true)
        .pic(true)
        .compile("cfunction.so");
}
