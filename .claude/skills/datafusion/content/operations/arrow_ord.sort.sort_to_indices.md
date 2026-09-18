# `arrow_ord::sort::sort_to_indices`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.sort_to_indices.json).

<a id="op-103cd36daa9d40320d711c7e"></a>
## sort_to_indices

`function` · `arrow_ord::sort::sort_to_indices` · arrow-ord 59.3.0

```rust
fn sort_to_indices(array: &dyn Array, options: Option<SortOptions>, limit: Option<usize>) -> Result<UInt32Array, arrow_schema::ArrowError>
```

Source: `src/sort.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Sort elements from `ArrayRef` into an unsigned integer (`UInt32Array`) of indices.
Floats are sorted using IEEE 754 totalOrder.  `limit` is an option for [partial_sort](../operations/arrow_ord.sort.partial_sort.md#op-2737bc1fa18cec7d1dc587ba).
