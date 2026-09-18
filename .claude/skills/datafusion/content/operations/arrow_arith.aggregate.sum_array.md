# `arrow_arith::aggregate::sum_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.sum_array.json).

<a id="op-fcf21a10e9cca1c4b273e2a1"></a>
## sum_array

`function` · `arrow_arith::aggregate::sum_array` · arrow-arith 59.3.0

```rust
fn sum_array<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Option<T::Native>
```

Source: `src/aggregate.rs:574`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the sum of values in the array.

This doesn't detect overflow. Once overflowing, the result will wrap around.
For an overflow-checking variant, use [`sum_array_checked`](../operations/arrow_arith.aggregate.sum_array_checked.md#op-0d246d691ec5f625c5d4772c) instead.
