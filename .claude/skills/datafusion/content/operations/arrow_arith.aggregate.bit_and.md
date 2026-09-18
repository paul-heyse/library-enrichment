# `arrow_arith::aggregate::bit_and`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.bit_and.json).

<a id="op-a8903bd67e811ad1a24ccf6e"></a>
## bit_and

`function` · `arrow_arith::aggregate::bit_and` · arrow-arith 59.3.0

```rust
fn bit_and<T>(array: &PrimitiveArray<T>) -> Option<T::Native> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native> + ArrowNativeTypeOp
```

Source: `src/aggregate.rs:855`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the bitwise and of all non-null input values.

Returns `None` if the array is empty or only contains null values.
