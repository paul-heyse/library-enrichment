# `arrow_arith::aggregate::bit_xor`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.bit_xor.json).

<a id="op-c7a152e3de4717f6a2c2dcaf"></a>
## bit_xor

`function` · `arrow_arith::aggregate::bit_xor` · arrow-arith 59.3.0

```rust
fn bit_xor<T>(array: &PrimitiveArray<T>) -> Option<T::Native> where T: ArrowNumericType, T::Native: BitXor<Output = T::Native> + ArrowNativeTypeOp
```

Source: `src/aggregate.rs:869`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the bitwise xor of all non-null input values.

Returns `None` if the array is empty or only contains null values.
