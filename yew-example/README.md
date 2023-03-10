## Yew example
Yew is a component based framework for writing web applications similar to
React and Elm (which does not really help me as I don't have any experience with
those frameworks) using WebAssembly (Wasm). So this enables us to write a pure
Rust application which is then compiled to wasm. Wasm and the potential to be
more performat than JavaScript.

This example is intended to help me understand how Yew works and also serve as
a quick reference to try things out in a simple and familiar
setting/application.

### Trunk
We mentioned that we have to compile our Rust program to wasm.

### Build
```console
$ make build
```

### Running
```console
$ make serve
```
Open http://localhost:8080/ in a browser and we should see the application is
deployed.





