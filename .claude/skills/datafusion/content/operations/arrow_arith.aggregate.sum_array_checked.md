# `arrow_arith::aggregate::sum_array_checked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.sum_array_checked.json).

<a id="op-0d246d691ec5f625c5d4772c"></a>
## sum_array_checked

`function` · `arrow_arith::aggregate::sum_array_checked` · arrow-arith 59.3.0

```rust
fn sum_array_checked<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Result<Option<T::Native>, ArrowError>
```

Source: `src/aggregate.rs:614`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the sum of values in the array.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant,
use [`sum_array`](../operations/arrow_arith.aggregate.sum_array.md#op-fcf21a10e9cca1c4b273e2a1) instead.
Additionally returns an `Err` on run-end-encoded arrays with a provided
values type parameter that is incorrect.
