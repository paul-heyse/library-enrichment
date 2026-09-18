# `datafusion_expr_common::casts::is_supported_type`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.casts.is_supported_type.json).

<a id="op-1dbd128c6b1037fc996756a2"></a>
## is_supported_type

`function` · `datafusion_expr_common::casts::is_supported_type` · datafusion-expr-common 55.1.0

```rust
fn is_supported_type(data_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/casts.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns true if unwrap_cast_in_comparison supports this data type
