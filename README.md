# Bezier Easing for Rust

This is a rust port of [gre/bezier-easing](https://github.com/gre/bezier-easing).

Bezier easing provides a way to create custom easing functions (ease-in, ease-out, ease-in-out...) for use in animations.

By providing the coordinates of the bezier curve's control points, you can create your own easing functions that follow the curve you've defined.

## Usage

```rust
use bezier_easing::bezier_easing;

let ease = bezier_easing(0.25, 0.1, 0.25, 1.0);
assert_eq!(ease(0.0), 0.0);
assert_eq!(ease(0.5), 0.3125);
assert_eq!(ease(1.0), 1.0);
```

## License
MIT

## Acknowledgements

- [gre/bezier-easing](https://github.com/gre/bezier-easing)
- [implementations](https://greweb.me/2012/02/bezier-curve-based-easing-functions-from-concept-to-implementation)