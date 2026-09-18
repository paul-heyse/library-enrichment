# `arrow_arith::bitwise::bitwise_and_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_and_scalar.json).

<a id="op-c346bd522edaa0706edc0cce"></a>
## bitwise_and_scalar

`function` · `arrow_arith::bitwise::bitwise_and_scalar` · arrow-arith 59.3.0

```rust
fn bitwise_and_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native>
```

Source: `src/bitwise.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `and` every value in an array with the scalar. If any value in the array is null then the
result is also null.
