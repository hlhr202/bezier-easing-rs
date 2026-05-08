# WASM Demo

This example mirrors the upstream HTML canvas demo and samples easing values from the Rust implementation compiled to WebAssembly.

## Run

```sh
wasm-pack build --target web --out-dir pkg examples/wasm-demo
npx serve examples/wasm-demo
```

Open the local URL printed by `serve`.

The WebAssembly dependencies are scoped to this example crate and are not dependencies of the main `bezier_easing` library.
