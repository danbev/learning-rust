### Trunk
Trunk is similar to wasm-pack.
```
Trunk is designed for creating progressive, single-page web applications,
written in Rust, compiled to WebAssembly, without any JS (though today JS is
still needed for loading WASM modules). 
```

An an example can be found in [trunk-example](../trunk-example).

### Overview
Trunk uses a source html file to drive all the assets and building and bundling.
The [index.html](./index.html) file is just the structure of a single
applications page (SPA):
```html
<html>
  <head>
    <link data-trunk rel="scss" href="index.scss"/>
    <!-- This following is not really required but just to show how assets 
	specified
    -->
    <link data-trunk rel="rust" href="./Cargo.toml"/>
  </head>
</html>
```
Notice that the `link` elements contain `data-trunk` attributes. More about this
later.

After trunk build has been run the `dist` directory will contain the actual
html page that will get served:
```html
<html>
<head>
  <link rel="stylesheet" href="/index-597d90855083f957.css">
  
  <link rel="preload"
    href="/trunk-example-61548d88d76db941_bg.wasm"
    as="fetch"
    type="application/wasm"
    crossorigin="">

   <link rel="modulepreload" href="/trunk-example-61548d88d76db941.js">
</head>

<body>

<script type="module">
  import init from '/trunk-example-61548d88d76db941.js';
  init('/trunk-example-61548d88d76db941_bg.wasm');
</script>

<script>(function () {
    var protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    var url = protocol + '//' + window.location.host + '/_trunk/ws';
    var poll_interval = 5000;
    var reload_upon_connect = () => {
        window.setTimeout(
            () => {
                // when we successfully reconnect, we'll force a
                // reload (since we presumably lost connection to
                // trunk due to it being killed, so it will have
                // rebuilt on restart)
                var ws = new WebSocket(url);
                ws.onopen = () => window.location.reload();
                ws.onclose = reload_upon_connect;
            },
            poll_interval);
    };

    var ws = new WebSocket(url);
    ws.onmessage = (ev) => {
        const msg = JSON.parse(ev.data);
        if (msg.reload) {
            window.location.reload();
        }
    };
    ws.onclose = reload_upon_connect;
})()
</script></body></html>
```
Notice the `link`s in the head element.

### link preload
```
The preload value of the <link> element's rel attribute lets you declare fetch
requests in the HTML's <head>, specifying resources that your page will need
very soon, which you want to start loading early in the page lifecycle, before
browsers' main rendering machinery kicks in.
```

### link modulepreload
```
The modulepreload keyword for the rel attribute of the <link> element provides
a declarative way to preemptively fetch a module script and its dependencies,
and store them in the document's module map for later evaluation.
```

### HtmlPipeline
When we run `trunk build` this call [build.rs] which will log the first
line in the console output:
```console
$ trunk build
2023-04-24T06:23:54.480734Z  INFO 📦 starting build
```
build will call `do_build` which will call:
```rust
        // Spawn the source HTML pipeline. This will spawn all other pipelines derived from
        // the source HTML, and will ultimately generate and write the final HTML.
        self.html_pipeline
            .clone()
            .spawn()
            .await
            .context("error joining HTML pipeline")?
            .context("error from HTML pipeline")?;
```
[HtmlPipeline]'s [spawn] will call [run] which will open the html file and
process it by iterating over the `link data-trunk`. 
```html
    <link data-trunk rel="scss" href="index.scss"/>
    <link data-trunk rel="rust" href="./Cargo.toml"/>
```

For the `rust` "type" this will cause [rust.rs] to be built which is done using
cargo.
```rust
    /// Spawn a new pipeline.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn spawn(self) -> JoinHandle<Result<TrunkAssetPipelineOutput>> {
        tokio::spawn(self.build())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn build(mut self) -> Result<TrunkAssetPipelineOutput> {
        let (wasm, hashed_name) = self.cargo_build().await?;
        let output = self.wasm_bindgen_build(wasm.as_ref(), &hashed_name).await?;
        self.wasm_opt_build(&output.wasm_output).await?;
        Ok(TrunkAssetPipelineOutput::RustApp(output))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn cargo_build(&mut self) -> Result<(PathBuf, String)> {
        tracing::info!("building {}", &self.manifest.package.name);

        // Spawn the cargo build process.
        let mut args = vec![
            "build",
            "--target=wasm32-unknown-unknown",
            "--manifest-path",
            &self.manifest.manifest_path,
        ];
       ...
```
This, `self.cargo_build`, will compile the project simliar to:
```console
$ cargo b --target=wasm32-unknown-unknown
```
And after this we will find the `.wasm` module gets generated:
```console
$ find target/ -name '*.wasm'
target/wasm32-unknown-unknown/debug/deps/trunk_example-7ce50f1fb0599b0d.wasm
target/wasm32-unknown-unknown/debug/trunk-example.wasm
```
And it is followed by `wasm_bindgen_build`. trunk will download an cache the
correct version of wasm-bindgen which is stored in:
```console
$ file ~/.cache/trunk/wasm-bindgen-0.2.84/wasm-bindgen 
```
We can invoke this manually and specify a different output directory (`out`)
which to inspect the files generated by wasm-bindgen:
```
$ ~/.cache/trunk/wasm-bindgen-0.2.84/wasm-bindgen --target web --out-dir out --out-name trunk-example target/wasm32-unknown-unknown/debug/trunk-example.wasm
```
```
$ ls out/
trunk-example_bg.wasm  trunk-example_bg.wasm.d.ts  trunk-example.d.ts  trunk-example.js
```
trunk will then copy over these files to the `dist` directory.
TODO: look into WasmOpt in trunk.

### dist/index.html
```html
<html>
<head>
  <link rel="stylesheet" href="/index-597d90855083f957.css">
  
  <link rel="preload"
    href="/trunk-example-61548d88d76db941_bg.wasm"
    as="fetch"
    type="application/wasm"
    crossorigin="">

   <link rel="modulepreload" href="/trunk-example-61548d88d76db941.js">
</head>

<body>

<script type="module">
  import init from '/trunk-example-61548d88d76db941.js';
  init('/trunk-example-61548d88d76db941_bg.wasm');
</script>

<script>(function () {
    var protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    var url = protocol + '//' + window.location.host + '/_trunk/ws';
    var poll_interval = 5000;
    var reload_upon_connect = () => {
        window.setTimeout(
            () => {
                // when we successfully reconnect, we'll force a
                // reload (since we presumably lost connection to
                // trunk due to it being killed, so it will have
                // rebuilt on restart)
                var ws = new WebSocket(url);
                ws.onopen = () => window.location.reload();
                ws.onclose = reload_upon_connect;
            },
            poll_interval);
    };

    var ws = new WebSocket(url);
    ws.onmessage = (ev) => {
        const msg = JSON.parse(ev.data);
        if (msg.reload) {
            window.location.reload();
        }
    };
    ws.onclose = reload_upon_connect;
})()
</script></body></html>
```

[HtmlPipeline]: https://github.com/thedodd/trunk/blob/master/src/pipelines/html.rs
[spawn]: https://github.com/thedodd/trunk/blob/cb691cc625a8a51e93a0c52a822be1bb4f41f827/src/pipelines/html.rs#L68
[run]: https://github.com/thedodd/trunk/blob/cb691cc625a8a51e93a0c52a822be1bb4f41f827/src/pipelines/html.rs#L73
[build.rs]: https://github.com/thedodd/trunk/blob/cb691cc625a8a51e93a0c52a822be1bb4f41f827/src/build.rs#L44
[rust.rs]: https://github.com/thedodd/trunk/blob/cb691cc625a8a51e93a0c52a822be1bb4f41f827/src/pipelines/rust.rs
