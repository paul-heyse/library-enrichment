# `datafusion_common::utils::search_in_slice`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.search_in_slice.json).

<a id="op-950eba7e26b2e0d34c70e853"></a>
## search_in_slice

`function` · `datafusion_common::utils::search_in_slice` · datafusion-common 55.1.0

```rust
fn search_in_slice<F>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], compare_fn: F, low: usize, high: usize) -> Result<usize> where F: Fn(&[ScalarValue], &[ScalarValue]) -> Result<bool>
```

Source: `src/utils/mod.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function searches for a tuple of given values (`target`) among a slice of
the given rows (`item_columns`) via a linear scan. The slice starts at the index
`low` and ends at the index `high`. The boolean-valued function `compare_fn`
specifies the stopping criterion.
