# `arrow_arith::bitwise::bitwise_xor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_xor.json).

<a id="op-61b8ea2d49479f8f472ce234"></a>
## bitwise_xor

`function` · `arrow_arith::bitwise::bitwise_xor` · arrow-arith 59.3.0

```rust
fn bitwise_xor<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitXor<Output = T::Native>
```

Source: `src/bitwise.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `left ^ right` operation on two arrays. If either left or right value is null
then the result is also null.
