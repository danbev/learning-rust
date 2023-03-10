## Example of using trunk

### Prerequisites
```console
$ cargo install --locked wasm-bindgen-cli
```

Add wasm-bindgen
```console
$ cargo add wasm-bindgen
```

### Building
```console
$ make build
```

### Running
```console
$ make serve
trunk serve
2023-03-10T09:24:27.356616Z  INFO 📦 starting build
2023-03-10T09:24:27.356823Z  INFO spawning asset pipelines
2023-03-10T09:24:27.658376Z  INFO building trunk-example
2023-03-10T09:24:27.658479Z  INFO compiling sass/scss path="index.scss"
2023-03-10T09:24:27.667986Z  INFO finished compiling sass/scss path="index.scss"
warning: trunk-example v0.1.0 (/home/danielbevenius/work/rust/learning-rust/trunk-example) ignoring invalid dependency `trunk` which is missing a lib target
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
2023-03-10T09:24:27.892680Z  INFO fetching cargo artifacts
2023-03-10T09:24:28.091583Z  INFO processing WASM for trunk-example
2023-03-10T09:24:28.098415Z  INFO calling wasm-bindgen for trunk-example
2023-03-10T09:24:28.144088Z  INFO copying generated wasm-bindgen artifacts
2023-03-10T09:24:28.144672Z  INFO applying new distribution
2023-03-10T09:24:28.145262Z  INFO ✅ success
2023-03-10T09:24:28.145445Z  INFO 📡 serving static assets at -> /
2023-03-10T09:24:28.145478Z  INFO 📡 server listening at http://127.0.0.1:8080
2023-03-10T09:24:30.072791Z  INFO 📦 starting build
2023-03-10T09:24:30.072920Z  INFO spawning asset pipelines
2023-03-10T09:24:30.296296Z  INFO building trunk-example
2023-03-10T09:24:30.296451Z  INFO compiling sass/scss path="index.scss"
2023-03-10T09:24:30.305654Z  INFO finished compiling sass/scss path="index.scss"
warning: trunk-example v0.1.0 (/home/danielbevenius/work/rust/learning-rust/trunk-example) ignoring invalid dependency `trunk` which is missing a lib target
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
2023-03-10T09:24:30.510948Z  INFO fetching cargo artifacts
2023-03-10T09:24:30.735079Z  INFO processing WASM for trunk-example
2023-03-10T09:24:30.741725Z  INFO calling wasm-bindgen for trunk-example
2023-03-10T09:24:30.795769Z  INFO copying generated wasm-bindgen artifacts
2023-03-10T09:24:30.796370Z  INFO applying new distribution
2023-03-10T09:24:30.796965Z  INFO ✅ success

```
Now, open http://127.0.0.1:8080 and open the console (CTRL+SHIFT+J) and you
should see the following output:
```
print mesg:  bajja
```
