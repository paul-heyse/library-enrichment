# `arrow_arith::bitwise::bitwise_or_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_or_scalar.json).

<a id="op-14aab992ba96e5b1eb330c3f"></a>
## bitwise_or_scalar

`function` · `arrow_arith::bitwise::bitwise_or_scalar` · arrow-arith 59.3.0

```rust
fn bitwise_or_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitOr<Output = T::Native>
```

Source: `src/bitwise.rs:150`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `or` every value in an array with the scalar. If any value in the array is null then the
result is also null.
