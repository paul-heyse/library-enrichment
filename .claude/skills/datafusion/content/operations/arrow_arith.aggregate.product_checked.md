# `arrow_arith::aggregate::product_checked`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.product_checked.json).

<a id="op-d5af67299b94b11fd2dd26ca"></a>
## product_checked

`function` · `arrow_arith::aggregate::product_checked` · arrow-arith 59.3.0

```rust
fn product_checked<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Result<Option<T::Native>, ArrowError>
```

Source: `src/aggregate.rs:963`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the product of values in the primitive array.

Returns `Ok(None)` if the array is empty or only contains null values.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant,
use [`product`](../operations/arrow_arith.aggregate.product.md#op-fa068b1b44ebf2b93af0cf97) instead.
