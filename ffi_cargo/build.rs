fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=cfunctions.c");
    println!("cargo:rustc-link-lib=cfunctions");
    println!("cargo:rustc-link-search=.");
    //rustc -L. -lcfunctions something.rs -C link-arg='-Wl,-rpath,${PWD}'
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("cfunctions.c")
        .compile("cfunction");
}
