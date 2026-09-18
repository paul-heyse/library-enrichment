# `datafusion_common::utils::extract_row_at_idx_to_buf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.extract_row_at_idx_to_buf.json).

<a id="op-7816a1e419e5da23954fdee6"></a>
## extract_row_at_idx_to_buf

`function` · `datafusion_common::utils::extract_row_at_idx_to_buf` · datafusion-common 55.1.0

```rust
fn extract_row_at_idx_to_buf(columns: &[arrow::array::ArrayRef], idx: usize, buf: &mut Vec<ScalarValue>) -> Result<()>
```

Source: `src/utils/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Extracts a row at the specified index from a set of columns and stores it in the provided buffer.
