# `datafusion_expr_common::type_coercion::binary::binary_to_string_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.binary_to_string_coercion.json).

<a id="op-1d5ee2f9d849ce3c0424de3f"></a>
## binary_to_string_coercion

`function` · `datafusion_expr_common::type_coercion::binary::binary_to_string_coercion` · datafusion-expr-common 55.1.0

```rust
fn binary_to_string_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:1815`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coercion rules for binary (Binary/LargeBinary) to string (Utf8/LargeUtf8):
If one argument is binary and the other is a string then coerce to string
(e.g. for `like`)
