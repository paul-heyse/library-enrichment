# `datafusion_common::utils::compare_rows`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.compare_rows.json).

<a id="op-8ef2c60bae315cd9ce657880"></a>
## compare_rows

`function` · `datafusion_common::utils::compare_rows` · datafusion-common 55.1.0

```rust
fn compare_rows(x: &[ScalarValue], y: &[ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> Result<std::cmp::Ordering>
```

Source: `src/utils/mod.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function compares two tuples depending on the given sort options.
