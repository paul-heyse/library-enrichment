# `datafusion_expr::logical_plan::builder::table_scan_with_filter_and_fetch`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.table_scan_with_filter_and_fetch.json).

<a id="op-daeb4e8f8bfe149b8612cdf0"></a>
## table_scan_with_filter_and_fetch

`function` · `datafusion_expr::logical_plan::builder::table_scan_with_filter_and_fetch` · datafusion-expr 55.1.0

```rust
fn table_scan_with_filter_and_fetch(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> datafusion_common::Result<LogicalPlanBuilder>
```

Source: `src/logical_plan/builder.rs:2101`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema,
filters, and inlined fetch.
This is mostly used for testing and documentation.
