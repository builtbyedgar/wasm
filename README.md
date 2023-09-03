# Wasm experiments

A collection of experiments with WebAssembly (Wasm).

<br />

## The ecosystem

To keep things simple I use the [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) vscode extension.

To pack the Rust example into a WebAssembly module run:

```bash
wasm-pack build --target web
```
This will build the Rust code into a WebAssembly module (.wasm file) and generate the necessary JavaScript to load and interface with it.


## Examples

The examples below are based on the official documentation for Wasm, Rust, wasm-bindgen, etc.

**The example list:**

- [Hello Wasm](./examples/hello-wasm/README.md) is the mos simple example. It shows how to compile Rust to Wasm and run it in the browser.
- [Audio](./examples/audio/README.md). In this example we will try to implement a FMOscillator in Rust.
- [Timing](./examples/timing/README.md). Try to sync some noise.