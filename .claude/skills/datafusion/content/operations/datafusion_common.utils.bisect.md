# `datafusion_common::utils::bisect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.bisect.json).

<a id="op-a25da6d42d80ac75b313e925"></a>
## bisect

`function` · `datafusion_common::utils::bisect` · datafusion-common 55.1.0

```rust
fn bisect<const SIDE: bool>(item_columns: &[arrow::array::ArrayRef], target: &[ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> Result<usize>
```

Source: `src/utils/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function searches for a tuple of given values (`target`) among the given
rows (`item_columns`) using the bisection algorithm. It assumes that `item_columns`
is sorted according to `sort_options` and returns the insertion index of `target`.
Template argument `SIDE` being `true`/`false` means left/right insertion.
