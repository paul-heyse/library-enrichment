# `datafusion_expr_common::type_coercion::binary::regex_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.regex_coercion.json).

<a id="op-0c18a0497e35f5b23f77c934"></a>
## regex_coercion

`function` · `datafusion_expr_common::type_coercion::binary::regex_coercion` · datafusion-expr-common 55.1.0

```rust
fn regex_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:1898`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coercion rules for regular expression comparison operations.
This is a union of string coercion rules, dictionary coercion rules, and REE coercion rules.
