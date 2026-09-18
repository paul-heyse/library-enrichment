# `datafusion_common::utils::linear_search`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.linear_search.json).

<a id="op-a6bd845788116bfb0bfa8347"></a>
## linear_search

`function` · `datafusion_common::utils::linear_search` · datafusion-common 55.1.0

```rust
fn linear_search<const SIDE: bool>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> Result<usize>
```

Source: `src/utils/mod.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function searches for a tuple of given values (`target`) among the given
rows (`item_columns`) via a linear scan. It assumes that `item_columns` is sorted
according to `sort_options` and returns the insertion index of `target`.
Template argument `SIDE` being `true`/`false` means left/right insertion.
