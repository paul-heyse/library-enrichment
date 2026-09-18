# `arrow_arith::bitwise::bitwise_shift_left_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_shift_left_scalar.json).

<a id="op-4761de7efe860e9d822692ac"></a>
## bitwise_shift_left_scalar

`function` · `arrow_arith::bitwise::bitwise_shift_left_scalar` · arrow-arith 59.3.0

```rust
fn bitwise_shift_left_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShl<Output = T::Native>
```

Source: `src/bitwise.rs:176`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `left << right` every value in an array with the scalar. If any value in the array is null then the
result is also null.
