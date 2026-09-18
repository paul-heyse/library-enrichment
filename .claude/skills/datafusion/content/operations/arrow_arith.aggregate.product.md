# `arrow_arith::aggregate::product`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.product.json).

<a id="op-fa068b1b44ebf2b93af0cf97"></a>
## product

`function` · `arrow_arith::aggregate::product` · arrow-arith 59.3.0

```rust
fn product<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

Source: `src/aggregate.rs:953`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the product of values in the primitive array.

Returns `None` if the array is empty or only contains null values.

This doesn't detect overflow in release mode by default. Once overflowing, the result will
wrap around. For an overflow-checking variant, use [`product_checked`](../operations/arrow_arith.aggregate.product_checked.md#op-d5af67299b94b11fd2dd26ca) instead.
