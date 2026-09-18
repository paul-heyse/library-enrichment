# `datafusion_expr::logical_plan::builder::table_scan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.table_scan.json).

<a id="op-a054f8221bf10ec1f61bb378"></a>
## table_scan

`function` · `datafusion_expr::logical_plan::builder::table_scan` · datafusion-expr 55.1.0

```rust
fn table_scan(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> datafusion_common::Result<LogicalPlanBuilder>
```

Source: `src/logical_plan/builder.rs:2074`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema.
This is mostly used for testing and documentation.
