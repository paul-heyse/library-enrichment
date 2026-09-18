# `datafusion_expr_common::type_coercion::binary::like_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.like_coercion.json).

<a id="op-b9022981ae159e9f3132a58e"></a>
## like_coercion

`function` · `datafusion_expr_common::type_coercion::binary::like_coercion` · datafusion-expr-common 55.1.0

```rust
fn like_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:1876`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coercion rules for like operations.
This is a union of string coercion rules, dictionary coercion rules, and REE coercion rules
Note: list_coercion is intentionally NOT included here because LIKE is a string pattern
matching operation and is not supported for nested types (List, Struct, etc.)
