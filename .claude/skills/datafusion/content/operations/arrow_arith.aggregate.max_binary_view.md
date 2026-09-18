# `arrow_arith::aggregate::max_binary_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_binary_view.json).

<a id="op-b1e02dc8fc610a73e03fdf34"></a>
## max_binary_view

`function` · `arrow_arith::aggregate::max_binary_view` · arrow-arith 59.3.0

```rust
fn max_binary_view(array: &BinaryViewArray) -> Option<&[u8]>
```

Source: `src/aggregate.rs:526`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the maximum value in the binary view array, according to the natural order.
