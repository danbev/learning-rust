#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_interface;

use indexmap;
use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface::Compiler, Config, Queries};
use std::fs;
use std::path::PathBuf;
use wasm_compose::composer::ComponentComposer;
use wasm_compose::config::Config as ComposerConfig;
use wasm_compose::config::{Instantiation, InstantiationArg};
use wit_component::ComponentEncoder;

struct Seedwing;

impl Callbacks for Seedwing {
    fn config(&mut self, _config: &mut Config) {}

    fn after_parsing<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        Compilation::Continue
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        Compilation::Continue
    }

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        Compilation::Continue
    }
}

fn sys_root() -> Vec<String> {
    let home = option_env!("RUSTUP_HOME");
    let toolchain = option_env!("RUSTUP_TOOLCHAIN");
    let sysroot = format!("{}/toolchains/{}", home.unwrap(), toolchain.unwrap());
    vec!["--sysroot".into(), sysroot]
}

fn main() {
    let _ = rustc_driver::catch_fatal_errors(|| {
        let args: Vec<_> = vec!["--", "--target=wasm32-wasi", "src/example.rs"];
        let args = args
            .iter()
            .map(|s| (*s).to_string())
            .chain(sys_root().into_iter())
            .collect::<Vec<_>>();

        RunCompiler::new(&args, &mut Seedwing).run()
    })
    .map_err(|e| println!("{:?}", e));
    let module_path = PathBuf::from("example.wasm");

    // Make a component out for the core webassembly module
    let module = fs::read(module_path).unwrap();
    let adapter = fs::read("wasi_preview1_component_adapter.command.wasm").unwrap();
    let component_encoder = ComponentEncoder::default()
        .module(module.as_slice())
        .unwrap()
        .adapter("wasi_snapshot_preview1", adapter.as_slice())
        .unwrap();
    let component = component_encoder.encode().unwrap();
    let component_path = PathBuf::from("component.wasm");
    let _ = fs::write(&component_path, component);

    // Next compose component with another component
    let mut inst_args = indexmap::IndexMap::new();
    inst_args.insert(
        "compose::example/engine".to_string(),
        InstantiationArg {
            instance: "example-component".to_string(),
            export: None,
        },
    );
    let instantiation = Instantiation {
        arguments: inst_args,
        ..Default::default()
    };
    let mut instantiations = indexmap::IndexMap::new();
    instantiations.insert("input".to_string(), instantiation);

    let composer_config = ComposerConfig {
        search_paths: vec![PathBuf::from(".")],
        instantiations,
        ..Default::default()
    };
    let composer = ComponentComposer::new(&component_path, &composer_config);
    let _composed = composer.compose();
}
