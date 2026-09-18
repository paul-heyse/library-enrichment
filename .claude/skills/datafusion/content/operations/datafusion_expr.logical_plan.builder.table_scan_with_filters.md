# `datafusion_expr::logical_plan::builder::table_scan_with_filters`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.table_scan_with_filters.json).

<a id="op-915cefd2d2963ffd76943d98"></a>
## table_scan_with_filters

`function` · `datafusion_expr::logical_plan::builder::table_scan_with_filters` · datafusion-expr 55.1.0

```rust
fn table_scan_with_filters(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, filters: Vec<Expr>) -> datafusion_common::Result<LogicalPlanBuilder>
```

Source: `src/logical_plan/builder.rs:2085`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema,
and inlined filters.
This is mostly used for testing and documentation.
