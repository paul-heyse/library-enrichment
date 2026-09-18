# `datafusion_common::utils::get_row_at_idx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.get_row_at_idx.json).

<a id="op-d962a973777c122f7f50ba6f"></a>
## get_row_at_idx

`function` · `datafusion_common::utils::get_row_at_idx` · datafusion-common 55.1.0

```rust
fn get_row_at_idx(columns: &[arrow::array::ArrayRef], idx: usize) -> Result<Vec<ScalarValue>>
```

Source: `src/utils/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Given column vectors, returns row at `idx`.
