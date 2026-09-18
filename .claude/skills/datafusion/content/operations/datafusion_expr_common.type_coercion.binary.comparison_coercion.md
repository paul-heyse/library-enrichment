# `datafusion_expr_common::type_coercion::binary::comparison_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.comparison_coercion.json).

<a id="op-360302c9badc4c090a1d63b0"></a>
## comparison_coercion

`function` · `datafusion_expr_common::type_coercion::binary::comparison_coercion` · datafusion-expr-common 55.1.0

```rust
fn comparison_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:953`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coerce `lhs_type` and `rhs_type` to a common type for comparison
contexts — any context where two values are compared rather than
unified. This includes binary comparison operators, IN lists,
CASE/WHEN conditions, and BETWEEN.

When the two types differ, this function determines the common type
to cast to.

# Numeric comparisons

The lower precision type is widened to the higher precision type
(e.g., `Int32` vs `Int64` → `Int64`).

# Numeric / String comparisons

Prefers the numeric type (e.g., `'2' > 1` where `1` is `Int32` coerces
`'2'` to `Int32`).

For type unification contexts (UNION, CASE THEN/ELSE), use
[`type_union_coercion`](../operations/datafusion_expr_common.type_coercion.binary.type_union_coercion.md#op-9303c7820aa0bbaf350e183e) instead.
