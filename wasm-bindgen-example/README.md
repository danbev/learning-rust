## wasm-bindgen example
This example is intended to help me understand how wasm-bindgen works
and also act as quick reference to try things out in a simple and familiar
setting/application.

### Prerequisites

The wasm tool chain is required:
```console
$ rustup target add wasm32-unknown-unknown
```

Install `webpack`:
```console
$ npm install --save-dev webpack webpack-cli
```

add `wasm-bindgen` crate
```console
$ cargo add wasm-bindgen
```

### Building
Compile:
```console
$ make build
```

### Running
```console
$ python3 -m http.server
```
Open http://0.0.0.0:8000/ in a web browser and you should see the following
output in the console (CTRL+SHIFT+J):
```
message: bajja
```
