# Changelog

## 0.3.0

### Breaking Changes

- Changed `BezierEasing` from a boxed callable function to an unboxed struct.
- Replaced direct calls like `ease(x)` with `ease.sample(x)`.
- Kept `bezier_easing(...)` as the free constructor, now returning the unboxed `BezierEasing` struct.

### Added

- Added `BezierEasing::new(...)` as an associated constructor.
- Added Criterion benchmarks comparing the unboxed implementation against the previous boxed implementation.
- Added bitwise regression tests to verify sampled `f32` and `f64` values match the previous boxed implementation.

## 0.2.1

### Added

- Added upstream JavaScript golden-output tests for common, overshooting, steep, and degenerate curves.
- Added boundary tests for invalid x control points, NaN/Infinity handling, endpoint preservation, unclamped inputs, and y control points outside `[0, 1]`.
- Added `proptest` coverage for endpoint invariants, deterministic outputs, identity curves, symmetric curves, and `f32`/`f64` consistency.
- Added a CI coverage gate requiring at least 90% line coverage for the library crate.

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
