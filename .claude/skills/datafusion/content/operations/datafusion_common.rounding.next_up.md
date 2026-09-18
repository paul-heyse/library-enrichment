# `datafusion_common::rounding::next_up`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.rounding.next_up.json).

<a id="op-a6d1a0f749d41cf674481ef2"></a>
## next_up

`function` · `datafusion_common::rounding::next_up` · datafusion-common 55.1.0

```rust
fn next_up<F: FloatBits + Copy>(float: F) -> F
```

Source: `src/rounding.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the next representable floating-point value greater than the input value.

This function takes a floating-point value that implements the FloatBits trait,
calculates the next representable value greater than the input, and returns it.

If the input value is NaN or positive infinity, the function returns the input value.

# Examples

```
use datafusion_common::rounding::next_up;

let f: f32 = 1.0;
let next_f = next_up(f);
assert_eq!(next_f, 1.0000001);
```
