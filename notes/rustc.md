## Rust Compiler notes
This page contains notes about the Rust compiler `rustc`.

The compiler can be found in a subdirectory named `rustc` of the rust-lang rust
repository.

### Building rustc
```console
$ ./x.py build
````

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
Getting the same information with cargo:
```console
$ cargo rustc --bin gpio_async --  -C help
```

Show all nightly features:
```console
$ rustc -Z help
```

### rustc walkthrough
Some of this walk through might seem like it's going through unnecessary details
but this is intentional as I'm fairly new to Rust and I believe that there are
design patterns etc that I can pick up from going through the code base.

The entry point to rustc is in `compiler/rustc/src/main.rs` there are some step
related to `jemalloc` and the setup of signal handlers first, after which
 `rustc_driver::main()` is called which is path dependency of the rustc crate:
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

Lets start a debug session using an empty main function and a locally built
`rustc` compiler with `debug = true` set in config.toml and then rebuild the
compiler:
```console
[rust]
debug = true
debug-logging = true
...
debuginfo-level = 2
```

```console
$ ./x.py build library
```
After that we can start a debugging session using:
```rust
$ rust-gdb --args ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc main.rs
Reading symbols from ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc...
(gdb) br rustc_main::main 
Breakpoint 1 at 0x1131: file compiler/rustc/src/main.rs, line 61.
(gdb) r
```

`rustc_driver` can be found in `compiler/rustc_driver/src/lib.rs`
```rust
pub fn main() => ! {
  ...
  let start_rss = get_resident_set_size();
  init_rustc_env_logger();
  signal_handler::install();  // setups up signal handling for the process
}
```
`get_resident_set_size` can be found in `compiler/rustc_datastructures/src/profiling.rs:
```rust
pub fn get_resident_set_size() -> Option<usize> {
   let field = 1;
   let contents = fs::read("/proc/self/statm").ok()?;
   let contents = String::from_utf8(contents).ok()?;
   let s = contents.split_whitespace().nth(field)?;
   let npages = s.parse::<usize>().ok()?;
   Some(npages * 4096)
}
```
The above is a actually using a conditional compilation attribute of
`#[cfg(unix)]` and reading the `statm` file form the filesystem and accessing
the `resident field (size, resident, shared, text, lib, data, dt). The resident
set size is the a measure of how much memory the process is consuming of the
physical RAM.

Next we have logging setup:
```rust
pub fn init_rustc_env_logger() {
    if let Err(error) = rustc_log::init_rustc_env_logger() {
        early_error(ErrorOutputType::default(), &error.to_string());
    }
}
```
rustc_log is a separate crate and my understanding is that this is to allow
other compiler crates to depend on it without having to depend on rustc_driver
which would increase the compile time during development.
```rust
pub fn init_rustc_env_logger() -> Result<(), Error> {
    init_env_logger("RUSTC_LOG")
}
```
`signal_handler::install()` will set up signal handling for the process and
print the stacktrace if a signal is delivered.

Next in rustc_driver main we we have:
```rust
     let mut callbacks = TimePassesCallbacks::default();
     install_ice_hook();
```
What is an `ice_hook`?  ICE stands for `Internal Compiler Error`. 
`install_ice_hook()` can be found `rustc_driver/src/lib.rs`:
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

After this, still in rustc_driver main, we have the following:
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
`catch_with_exit_code` looks like this:
```rust
pub fn catch_with_exit_code(f: impl FnOnce() -> interface::Result<()>) -> i32 {
    let result = catch_fatal_errors(f).and_then(|result| result);
    match result {
        Ok(()) => EXIT_SUCCESS,
        Err(_) => EXIT_FAILURE,
    }
}
````
An if we look at `catch_fatal_errors`:
```rust
pub fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, ErrorGuaranteed> {
    catch_unwind(panic::AssertUnwindSafe(f)).map_err(|value| {
        if value.is::<rustc_errors::FatalErrorMarker>() {
            ErrorGuaranteed::unchecked_claim_error_was_emitted()
        } else {
            panic::resume_unwind(value);
        }
    })
}
```
TODO: add link to catch_unwind notes.

So the first thing that happens in the closure passed to catch_with_exit_code is
setting up the args to be passed as the first argument to `RunCompiler::new`:
```rust
   RunCompiler::new(&args, &mut callbacks).run()
```

`RunCompiler::run`:
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
I'm not sure about the naming of the `at_args` field as this point but we know
that these the arguments from the command line.

And `run_compiler can be found in the same file:
```console
(gdb) br rustc_driver::run_compiler
```

```rust
fn run_compiler(
    at_args: &[String],
    callbacks: &mut (dyn Callbacks + Send),
    file_loader: Option<Box<dyn FileLoader + Send + Sync>>,
    emitter: Option<Box<dyn Write + Send>>,
    make_codegen_backend: Option<
        Box<dyn FnOnce(&config::Options) -> Box<dyn CodegenBackend> + Send>,
    >,
) -> interface::Result<()> {
    let args = args::arg_expand_all(at_args);
    let Some(matches) = handle_options(&args) else { return Ok(()) };
```
handle_options will handle any options like `--explain`, `help`, `C` (CodeGen
flags), `version`. If one of those options were specified then `handle_options`
will print the help/usage and return None which is handled by the above else
clause to return an empty Result.
Next session options are built which are the command line options like
`--crate-name`, `-L`, `--print` etc. All these are parsed and then an Options
is crated using the parsed fields.
Next, we have:
```rust
    let sopts = config::build_session_options(&matches);
```
This will parse other options like `L`, `sysroot`, `target`, `crate-name`,
`test`, `print`, etc.  The values of these will be collected and returned in
an `Options`.
Next, there will be a check for the `--explain` option and if there was such an
option it will be handled and then the function will return Ok(()).
Next, an `interface::Config` will be created which will be passed to:
```rust
interface::run_compiler(config, |compiler| {
        let sess = compiler.session();
        ...
}
```
`compiler/rustc_interface/src/interface.rs` we have:
```rust
pub fn run_compiler<R: Send>(config: Config, f: impl FnOnce(&Compiler) -> R + Send) -> R {
    tracing::trace!("run_compiler");
    util::run_in_thread_pool_with_globals(
        config.opts.edition,
        config.opts.unstable_opts.threads,
        || create_compiler_and_run(config, f),
    )
}
```
So, `util::run_in_thread_pool_with_globals` can be found in
compiler/rustc_interface/src/util.rs and will setup/configure the threading
properties like stack size. It will then call:
```rust
    let main_handler = move || rustc_span::create_session_globals_then(edition, f);
```
`create_session_globals_then` can be found in compiler_rustc_span/src/lib.rs:
```rust
#[inline]
pub fn create_session_globals_then<R>(edition: Edition, f: impl FnOnce() -> R) -> R {
    let session_globals = SessionGlobals::new(edition);
    SESSION_GLOBALS.set(&session_globals, f)
}
```
Lets take a closer look at `SessionGlobals`:
```rust
// Per-session global variables: this struct is stored in thread-local storage
// in such a way that it is accessible without any kind of handle to all
// threads within the compilation session, but is not accessible outside the
// session.
pub struct SessionGlobals {
    symbol_interner: symbol::Interner,
    span_interner: Lock<span_encoding::SpanInterner>,
    hygiene_data: Lock<hygiene::HygieneData>,
    source_map: Lock<Option<Lrc<SourceMap>>>,
}

scoped_tls::scoped_thread_local!(static SESSION_GLOBALS: SessionGlobals);
```
We can inspect this
```console
(gdb) whatis rustc_span::SESSION_GLOBALS
type = scoped_tls::ScopedKey<rustc_span::SessionGlobals>
(gdb) info variables SESSION_GLOBALS
All variables matching regular expression "SESSION_GLOBALS":
Non-debugging symbols:
0x00007fffee71c328  rustc_span::SESSION_GLOBALS::FOO::__getit::__KEY
0x00007ffff7f0f750  rustc_span::SESSION_GLOBALS
```
Next the function that will be called is `create_compiler_and_run(config, f)`,
```rust
pub fn run_compiler<R: Send>(config: Config, f: impl FnOnce(&Compiler) -> R + Send) -> R {
    util::run_in_thread_pool_with_globals(
        config.opts.edition,
        config.opts.unstable_opts.threads,
        || create_compiler_and_run(config, f),
    )
```

```rust
pub fn create_compiler_and_run<R>(config: Config, f: impl FnOnce(&Compiler) -> R) -> R {
    crate::callbacks::setup_callbacks();

    let registry = &config.registry;
    let (mut sess, codegen_backend) = util::create_session(
        config.opts,
        config.crate_cfg,
        config.crate_check_cfg,
        config.diagnostic_output,
        config.file_loader,
        config.input_path.clone(),
        config.lint_caps,
        config.make_codegen_backend,
        registry.clone(),
    );
```
`util::create_session` will create a `Session` which will be bound to the
`sess` variable.
```rust
    if let Some(parse_sess_created) = config.parse_sess_created {
        parse_sess_created(
            &mut Lrc::get_mut(&mut sess)
                .expect("create_session() should never share the returned session")
                .parse_sess,
        );
    }
```
If we look at `parse_sess_create` this is a declared as a callback in the struct
Config:
```rust
pub struct Config {
  ...
  /// This is a callback from the driver that is called when [`ParseSess`] is created.
  pub parse_sess_created: Option<Box<dyn FnOnce(&mut ParseSess) + Send>>,
  ...
}
```
So if `parse_sess_created` is not None it will be bound to `parse_sess_created`
in the `if let` expression. "if let destructures config.parse_sess_created into
Some(parse_sess_created) then evaluate the block" which is simliar to writing:
```rust
    match config.parse_sess_created {
        Some(parse_sess_created) => {
            parse_sess_created(
	        &mut Lrc::get_mut(&mut sess).expect("create_session() should never share the returned session").parse_sess,
            )
        },
        None => {
        }
    }
```
Arc::get_mut returns a mutable reference if there are now other Arc or Weak
pointers to the same location. `compiler/rustc_session/src/session.rs` `Session`
has the member named `parse_sess` which is then passed to the callback as the
sole argument. So the `expect` above is refering to util::create_session() if
I'm understanding this correctly. I was a little confused when I first read this
code which `create_session()` it was referring to.

