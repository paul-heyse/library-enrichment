# `datafusion_physical_expr::utils::collect_columns`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.collect_columns.json).

<a id="op-d936802da5653acf0cff3545"></a>
## collect_columns

`function` · `datafusion_physical_expr::utils::collect_columns` · datafusion-physical-expr 55.1.0

```rust
fn collect_columns(expr: &std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::HashSet<expressions::Column>
```

Source: `src/utils/mod.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Recursively extract referenced [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28)s within a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7).
