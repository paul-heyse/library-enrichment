# `datafusion::test_util::scan_empty_with_partitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.scan_empty_with_partitions.json).

<a id="op-d28326cec04ed3d32c6e5fa6"></a>
## scan_empty_with_partitions

`function` · `datafusion::test_util::scan_empty_with_partitions` · datafusion 55.1.0

```rust
fn scan_empty_with_partitions(name: Option<&str>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, partitions: usize) -> error::Result<logical_expr::LogicalPlanBuilder>
```

Source: `src/test_util/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Scan an empty data source with configured partition, mainly used in tests.
