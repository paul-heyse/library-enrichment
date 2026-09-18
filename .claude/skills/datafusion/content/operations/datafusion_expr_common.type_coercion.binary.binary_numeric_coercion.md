# `datafusion_expr_common::type_coercion::binary::binary_numeric_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.binary_numeric_coercion.json).

<a id="op-1d977f7c8d489bbd45655d33"></a>
## binary_numeric_coercion

`function` · `datafusion_expr_common::type_coercion::binary::binary_numeric_coercion` · datafusion-expr-common 55.1.0

```rust
fn binary_numeric_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:1048`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coerce `lhs_type` and `rhs_type` to a common type where both are numeric
