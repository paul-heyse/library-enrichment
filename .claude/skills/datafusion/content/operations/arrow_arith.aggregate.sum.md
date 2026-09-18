# `arrow_arith::aggregate::sum`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.sum.json).

<a id="op-9ca11115de8e6ca5f8995ed6"></a>
## sum

`function` · `arrow_arith::aggregate::sum` · arrow-arith 59.3.0

```rust
fn sum<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

Source: `src/aggregate.rs:943`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the sum of values in the primitive array.

Returns `None` if the array is empty or only contains null values.

This doesn't detect overflow in release mode by default. Once overflowing, the result will
wrap around. For an overflow-checking variant, use [`sum_checked`](../operations/arrow_arith.aggregate.sum_checked.md#op-ec941a70051abf9e17bb01c3) instead.
