# `datafusion_physical_expr_common::datum::compare_op_for_nested`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.datum.compare_op_for_nested.json).

<a id="op-20fe4ea1f77c04f82ce6dfce"></a>
## compare_op_for_nested

`function` · `datafusion_physical_expr_common::datum::compare_op_for_nested` · datafusion-physical-expr-common 55.1.0

```rust
fn compare_op_for_nested(op: datafusion_expr_common::operator::Operator, lhs: &dyn Datum, rhs: &dyn Datum) -> datafusion_common::Result<arrow::array::BooleanArray>
```

Source: `src/datum.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Compare on nested type List, Struct, and so on
