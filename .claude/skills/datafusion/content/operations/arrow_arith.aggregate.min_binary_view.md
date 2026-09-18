# `arrow_arith::aggregate::min_binary_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_binary_view.json).

<a id="op-eae114e013a1a3230da5dde7"></a>
## min_binary_view

`function` · `arrow_arith::aggregate::min_binary_view` · arrow-arith 59.3.0

```rust
fn min_binary_view(array: &BinaryViewArray) -> Option<&[u8]>
```

Source: `src/aggregate.rs:541`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the minimum value in the binary view array, according to the natural order.
