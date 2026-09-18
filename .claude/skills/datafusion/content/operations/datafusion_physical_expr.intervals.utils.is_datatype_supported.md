# `datafusion_physical_expr::intervals::utils::is_datatype_supported`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.utils.is_datatype_supported.json).

<a id="op-119dedbf41d245290f605fba"></a>
## is_datatype_supported

`function` · `datafusion_physical_expr::intervals::utils::is_datatype_supported` · datafusion-physical-expr 55.1.0

```rust
fn is_datatype_supported(data_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/intervals/utils.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indicates whether interval arithmetic is supported for the given data type.
