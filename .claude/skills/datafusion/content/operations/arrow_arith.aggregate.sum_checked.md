# `arrow_arith::aggregate::sum_checked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.sum_checked.json).

<a id="op-ec941a70051abf9e17bb01c3"></a>
## sum_checked

`function` · `arrow_arith::aggregate::sum_checked` · arrow-arith 59.3.0

```rust
fn sum_checked<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Result<Option<T::Native>, ArrowError>
```

Source: `src/aggregate.rs:897`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the sum of values in the primitive array.

Returns `Ok(None)` if the array is empty or only contains null values.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant,
use [`sum`](../operations/arrow_arith.aggregate.sum.md#op-9ca11115de8e6ca5f8995ed6) instead.
