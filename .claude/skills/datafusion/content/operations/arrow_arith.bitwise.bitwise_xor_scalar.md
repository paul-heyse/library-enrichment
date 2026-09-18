# `arrow_arith::bitwise::bitwise_xor_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_xor_scalar.json).

<a id="op-be2cb70cff8706fb6b45cde0"></a>
## bitwise_xor_scalar

`function` · `arrow_arith::bitwise::bitwise_xor_scalar` · arrow-arith 59.3.0

```rust
fn bitwise_xor_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitXor<Output = T::Native>
```

Source: `src/bitwise.rs:163`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `xor` every value in an array with the scalar. If any value in the array is null then the
result is also null.
