# `datafusion_common::rounding`

Crate `datafusion-common` · 4 public items · structured records in [`model/datafusion_common.rounding.json`](../model/datafusion_common.rounding.json)

## alter_fp_rounding_mode

`function` · `datafusion_common::rounding::alter_fp_rounding_mode`

```rust
fn alter_fp_rounding_mode<const UPPER: bool, F>(lhs: &ScalarValue, rhs: &ScalarValue, operation: F) -> Result<ScalarValue> where F: FnOnce(&ScalarValue, &ScalarValue) -> Result<ScalarValue>
```

---

## next_down

`function` · `datafusion_common::rounding::next_down`

```rust
fn next_down<F: FloatBits + Copy>(float: F) -> F
```

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

---

## next_up

`function` · `datafusion_common::rounding::next_up`

```rust
fn next_up<F: FloatBits + Copy>(float: F) -> F
```

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

---

## FloatBits

`trait` · `datafusion_common::rounding::FloatBits`

```rust
trait FloatBits
```

**Methods** (5)

```rust
fn float_is_nan(self) -> bool
fn from_bits(bits: Self::Item) -> Self
fn infinity() -> Self
fn neg_infinity() -> Self
fn to_bits(self) -> Self::Item
```

A trait to manipulate floating-point types with bitwise operations.
Provides functions to convert a floating-point value to/from its bitwise
representation as well as utility methods to handle special values.

---
