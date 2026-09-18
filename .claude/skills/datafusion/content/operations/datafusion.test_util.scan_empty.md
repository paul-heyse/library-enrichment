# `datafusion::test_util::scan_empty`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.scan_empty.json).

<a id="op-35f0f3e58302b7db59bdd304"></a>
## scan_empty

`function` · `datafusion::test_util::scan_empty` · datafusion 55.1.0

```rust
fn scan_empty(name: Option<&str>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> error::Result<logical_expr::LogicalPlanBuilder>
```

Source: `src/test_util/mod.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Scan an empty data source, mainly used in tests
