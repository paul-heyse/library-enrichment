# `arrow_arith::bitwise::bitwise_shift_right_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_shift_right_scalar.json).

<a id="op-5df510e9efe51e093e8daee7"></a>
## bitwise_shift_right_scalar

`function` · `arrow_arith::bitwise::bitwise_shift_right_scalar` · arrow-arith 59.3.0

```rust
fn bitwise_shift_right_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShr<Output = T::Native>
```

Source: `src/bitwise.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `left >> right` every value in an array with the scalar. If any value in the array is null then the
result is also null.
