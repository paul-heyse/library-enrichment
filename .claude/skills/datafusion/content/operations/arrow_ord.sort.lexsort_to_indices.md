# `arrow_ord::sort::lexsort_to_indices`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.lexsort_to_indices.json).

<a id="op-c91c43c26e3b78b4327d4c24"></a>
## lexsort_to_indices

`function` · `arrow_ord::sort::lexsort_to_indices` · arrow-ord 59.3.0

```rust
fn lexsort_to_indices(columns: &[SortColumn], limit: Option<usize>) -> Result<UInt32Array, arrow_schema::ArrowError>
```

Source: `src/sort.rs:940`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Sort elements lexicographically from a list of `ArrayRef` into an unsigned integer
(`UInt32Array`) of indices.

Note: for multi-column sorts without a limit, using the [row format](https://docs.rs/arrow-row/latest/arrow_row/)
may be significantly faster
