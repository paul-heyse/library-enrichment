# `datafusion_physical_expr_common::datum::apply_cmp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.datum.apply_cmp.json).

<a id="op-4b9aabedbc385a3d2b6214e3"></a>
## apply_cmp

`function` · `datafusion_physical_expr_common::datum::apply_cmp` · datafusion-physical-expr-common 55.1.0

```rust
fn apply_cmp(op: datafusion_expr_common::operator::Operator, lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue>
```

Source: `src/datum.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Applies a binary [`Datum`](../operations/arrow_array.scalar.Datum.md#op-8fd206fb21d47e60f8e9716f) comparison operator `op` to `lhs` and `rhs`
