## Rust Compiler notes
This page contains notes about the Rust compiler `rustc`.

The compiler can be found in a subdirectory named `rustc` of the rust-lang rust
repository.

### rustc command line options
```console
$ rustc --print crate-name src/bin/gpio_async.rs 
gpio_async
```
Getting the same information with cargo:
```console
$ cargo rustc --bin gpio_async -- --print crate-name

Show all Codegen options:
```console
$ rustc -C help
```
```console
$ cargo rustc --bin gpio_async --  -C help
```

Show all nightly features:
```console
$ rustc -Z help
```

### rustc walkthrough
Some of this walk through might seem like it going through unnecessary details
but this is intentional as I'm fairly new to Rust and I believe that there are
design patterns etc that I can pick up from going through the code base.

The entry point to rustc is in main.rs there are some step related to `jemalloc`
and the setup of signal handlers, after which `rustc_driver::main()` is called
which is path dependency of the rustc crate:
```toml
[dependencies]                                                                  
rustc_driver = { path = "../rustc_driver" }
```

```rust
fn main() {
     ...
     rustc_driver::set_sigpipe_handler();                                        
     rustc_driver::main() 
}
```

`rustc_driver` can be found in `compiler/rustc_driver/src/lib.rs`
```rust
pub fn main() => ! {
  ...
  init_rustc_env_logger();
  signal_handler::install();  // setups up signal handling for the process
}

pub fn init_rustc_env_logger() {
    if let Err(error) = rustc_log::init_rustc_env_logger() {
        early_error(ErrorOutputType::default(), &error.to_string());
    }
}
```
rustc_log is a separate crate and my understanding is that this is to allow
other compiler crated to depend on it without having to depend on rustc_driver
which would increase the compile time during development.
```rust
pub fn init_rustc_env_logger() -> Result<(), Error> {
    init_env_logger("RUSTC_LOG")
}
```
`signal_handler::install()` will set up signal handling for the process and
print the stacktrace if a signal is delivered.
Next we have:
```rust
     let mut callbacks = TimePassesCallbacks::default();
     install_ice_hook();
```
What is an `ice_hook`?  ICE stands for Internal Compiler Error. 
`install_ice_hook()` can be found rustc_driver/src/lib.rs:
```rust
pub fn install_ice_hook() {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    SyncLazy::force(&DEFAULT_HOOK);
}
```
Now, SyncLazy has been replaced with LazyCell in newer verisons of Rust but
the underlying functionality is bascially the same. The DEFAULT_HOOK will
replace the current panic hook and set a "custom" panic hook that
prints the backtrace and then the BUG_REPORT_URL. 
The `callbacks` is an implementation of Callbacks which is a trait which
has functions with the following default implementations:
```rust
fn config(&mut self, config: &mut interface::Config) {}
fn after_parsing<'tcx>(&mut self, compiler: &interface::Compiler,
                       queries: &'tcx Queries<'tcx>) -> Compilation {
    Compilation::Continue
}
fn after_expansion<'tcx>(&mut self, compiler: &interface::Compiler,
                       queries: &'tcx Queries<'tcx>) -> Compilation {
    Compilation::Continue
}
fn after_analysis<'tcx>(&mut self, compiler: &interface::Compiler,
                       queries: &'tcx Queries<'tcx>) -> Compilation {
    Compilation::Continue
}
```
The `interface` is a module, from the rustc_interface crate.
Compiler represents a compiler session.

After this we have the following:
```rust
    let exit_code = catch_with_exit_code(|| {
        let args = env::args_os()
            .enumerate()
            .map(|(i, arg)| {
                arg.into_string().unwrap_or_else(|arg| {
                    early_error(
                        ErrorOutputType::default(),
                        &format!("argument {} is not valid Unicode: {:?}", i, arg),
                    )
                })
            })
            .collect::<Vec<_>>();
        RunCompiler::new(&args, &mut callbacks).run()
    });
```
The callbacks 
```rust
    pub fn run(self) -> interface::Result<()> {
        run_compiler(
            self.at_args,
            self.callbacks,
            self.file_loader,
            self.emitter,
            self.make_codegen_backend,
        )
    }
```

