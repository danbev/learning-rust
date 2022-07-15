#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate std;

use rustc_driver::{Callbacks, RunCompiler, Compilation};
use rustc_interface::{interface, Queries};

struct SomeCallback {}

impl Callbacks for SomeCallback {
    fn after_parsing<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        queries: &'tcx Queries,
    ) -> Compilation {
        println!("SomeCallback::after_parsing...");
        Compilation::Continue
    }
}

/*
 * $ rustc --print sysroot
 * $ env LD_LIBRARY_PATH=/home/danielbevenius/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/ ./out/rustc --help
 */
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut cb = SomeCallback{};
    let result: interface::Result<()> = RunCompiler::new(&args, &mut cb).run();
}

