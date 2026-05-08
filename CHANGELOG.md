# Changelog

## 0.2.0

### Breaking Changes

- Changed the default easing float type to `f64`.
- Added generic float support for both `f32` and `f64`.
- Changed `bezier_easing` to return a boxed callable easing function.
- Replaced the previous sample/Newton/subdivision solver with the Cardano-based solver used by the upstream JavaScript implementation.

### Added

- Added integration tests ported from upstream `gre/bezier-easing`.
- Added regression coverage for degenerate `a == 0` curves matching upstream JavaScript behavior.
- Added a Cargo workspace layout.
- Added a WebAssembly HTML demo under `examples/wasm-demo`.
- Added GitHub Actions CI and GitHub Pages deployment support.

### Fixed

- Aligned invalid x-control-point error text with upstream JavaScript.
- Fixed documentation and examples for floating-point comparisons.

## 0.1.1

- Initial Rust library release.
