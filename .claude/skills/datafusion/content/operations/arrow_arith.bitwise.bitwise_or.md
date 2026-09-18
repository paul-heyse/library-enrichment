# `arrow_arith::bitwise::bitwise_or`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_or.json).

<a id="op-7b9ee1273e23539e6c92703c"></a>
## bitwise_or

`function` · `arrow_arith::bitwise::bitwise_or` · arrow-arith 59.3.0

```rust
fn bitwise_or<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitOr<Output = T::Native>
```

Source: `src/bitwise.rs:55`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `left | right` operation on two arrays. If either left or right value is null
then the result is also null.
