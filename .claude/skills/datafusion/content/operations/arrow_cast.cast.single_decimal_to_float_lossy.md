# `arrow_cast::cast::single_decimal_to_float_lossy`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.single_decimal_to_float_lossy.json).

<a id="op-d0424e9d70c901c308632c28"></a>
## single_decimal_to_float_lossy

`function` · `arrow_cast::cast::single_decimal_to_float_lossy` · arrow-cast 59.3.0

```rust
fn single_decimal_to_float_lossy<D, F>(f: &F, x: D::Native, scale: i32) -> f64 where D: DecimalType, F: Fn(D::Native) -> f64
```

Source: `src/cast/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Lossy conversion from decimal to float.

Conversion is lossy and follows standard floating point semantics. Values
that exceed the representable range become `INFINITY` or `-INFINITY` without
returning an error.
