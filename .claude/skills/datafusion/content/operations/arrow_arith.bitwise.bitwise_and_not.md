# `arrow_arith::bitwise::bitwise_and_not`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_and_not.json).

<a id="op-0765547f25d8658689664994"></a>
## bitwise_and_not

`function` · `arrow_arith::bitwise::bitwise_and_not` · arrow-arith 59.3.0

```rust
fn bitwise_and_not<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native> + Not<Output = T::Native>
```

Source: `src/bitwise.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `left & !right` operation on two arrays. If either left or right value is null
then the result is also null.
