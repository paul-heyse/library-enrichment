# `datafusion_common::rounding::next_down`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.rounding.next_down.json).

<a id="op-2d4a8568bd1ee23e0f3a4e3d"></a>
## next_down

`function` · `datafusion_common::rounding::next_down` · datafusion-common 55.1.0

```rust
fn next_down<F: FloatBits + Copy>(float: F) -> F
```

Source: `src/rounding.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the next representable floating-point value smaller than the input value.

This function takes a floating-point value that implements the FloatBits trait,
calculates the next representable value smaller than the input, and returns it.

If the input value is NaN or negative infinity, the function returns the input value.

# Examples

```
use datafusion_common::rounding::next_down;

let f: f32 = 1.0;
let next_f = next_down(f);
assert_eq!(next_f, 0.99999994);
```
