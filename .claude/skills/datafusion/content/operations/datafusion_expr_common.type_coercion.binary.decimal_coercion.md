# `datafusion_expr_common::type_coercion::binary::decimal_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.decimal_coercion.json).

<a id="op-250a1b6b1a3891c05a48f3c5"></a>
## decimal_coercion

`function` · `datafusion_expr_common::type_coercion::binary::decimal_coercion` · datafusion-expr-common 55.1.0

```rust
fn decimal_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Decimal coercion rules.
