# `datafusion_expr_common::type_coercion::binary::type_union_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.type_union_coercion.json).

<a id="op-9303c7820aa0bbaf350e183e"></a>
## type_union_coercion

`function` · `datafusion_expr_common::type_coercion::binary::type_union_coercion` · datafusion-expr-common 55.1.0

```rust
fn type_union_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:915`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coerce `lhs_type` and `rhs_type` to a common type for type unification
contexts — where two values must be brought to a common type but are not
being compared. Examples: UNION, CASE THEN/ELSE branches, NVL2. For other
contexts, [`comparison_coercion`](../operations/datafusion_expr_common.type_coercion.binary.comparison_coercion.md#op-360302c9badc4c090a1d63b0) should typically be used instead.

The intuition is that we try to find the "widest" type that can represent
all values from both sides. When one side is a string and the other is
numeric, this prefers strings because every number has a textual
representation but not every string can be parsed as a number (e.g., `SELECT
1 UNION SELECT 'a'` coerces both sides to a string). This is in contrast to
[`comparison_coercion`](../operations/datafusion_expr_common.type_coercion.binary.comparison_coercion.md#op-360302c9badc4c090a1d63b0), which prefers numeric types so that ordering and
equality follow numeric semantics.
