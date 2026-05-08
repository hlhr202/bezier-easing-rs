# Bezier Easing for Rust

[![Crates.io](https://img.shields.io/crates/v/bezier_easing.svg)](https://crates.io/crates/bezier_easing)
[![Documentation](https://docs.rs/bezier_easing/badge.svg)](https://docs.rs/bezier_easing)

This is a Rust port of [gre/bezier-easing](https://github.com/gre/bezier-easing).

Bezier easing provides a way to create custom easing functions (ease-in, ease-out, ease-in-out...) for use in animations.

By providing the coordinates of the bezier curve's control points, you can create your own easing functions that follow the curve you've defined.

## Workspace

- `crates/bezier-easing`: the published Rust library.
- `examples/wasm-demo`: an HTML canvas demo powered by the Rust library compiled to WebAssembly.

## Installation

```toml
[dependencies]
bezier_easing = "0.3"
```

## Usage

```rust
use bezier_easing::bezier_easing;

let ease = bezier_easing(0.0_f64, 0.0, 1.0, 0.5).unwrap();
assert_eq!(ease.sample(0.0), 0.0);
assert!((ease.sample(0.5) - 0.3125).abs() < 0.000001);
assert_eq!(ease.sample(1.0), 1.0);
```

The default floating point type is `f64`. Use suffixed arguments to create an `f32` easing function:

```rust
use bezier_easing::bezier_easing;

let ease = bezier_easing(0.0_f32, 0.0, 1.0, 0.5).unwrap();
assert!((ease.sample(0.5) - 0.3125).abs() < 0.000001);
```

## WASM Example

Online demo: <https://hlhr202.github.io/bezier-easing-rs/>

```sh
wasm-pack build --target web --out-dir pkg examples/wasm-demo
npx serve examples/wasm-demo
```

Open the local URL printed by `serve` to view the canvas animation.

## Publishing

Run the release checks before publishing the library crate:

```sh
cargo fmt --check
cargo clippy -p bezier_easing --all-targets --all-features -- -D warnings
cargo test -p bezier_easing --all-features
cargo llvm-cov -p bezier_easing --all-features --fail-under-lines 90
cargo package -p bezier_easing
cargo publish -p bezier_easing --dry-run
```

Publish only after the dry run succeeds:

```sh
cargo publish -p bezier_easing
```

## License
MIT

## Acknowledgements

- [gre/bezier-easing](https://github.com/gre/bezier-easing)
- [implementations](https://greweb.me/2012/02/bezier-curve-based-easing-functions-from-concept-to-implementation)
