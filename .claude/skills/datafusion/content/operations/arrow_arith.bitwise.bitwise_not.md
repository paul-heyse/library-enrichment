# `arrow_arith::bitwise::bitwise_not`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_not.json).

<a id="op-ecb38c36d08fa6e60054e12e"></a>
## bitwise_not

`function` · `arrow_arith::bitwise::bitwise_not` · arrow-arith 59.3.0

```rust
fn bitwise_not<T>(array: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: Not<Output = T::Native>
```

Source: `src/bitwise.rs:113`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `!array` operation on array. If array value is null
then the result is also null.
