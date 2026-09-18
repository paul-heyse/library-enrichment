# `arrow_arith::bitwise::bitwise_and`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_and.json).

<a id="op-be3081b1c58f9f4ba1b80d5c"></a>
## bitwise_and

`function` · `arrow_arith::bitwise::bitwise_and` · arrow-arith 59.3.0

```rust
fn bitwise_and<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native>
```

Source: `src/bitwise.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `left & right` operation on two arrays. If either left or right value is null
then the result is also null.
