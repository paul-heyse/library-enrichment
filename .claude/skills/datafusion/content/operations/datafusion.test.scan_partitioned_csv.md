# `datafusion::test::scan_partitioned_csv`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.scan_partitioned_csv.json).

<a id="op-4b31beda98a51ab86d72ab3a"></a>
## scan_partitioned_csv

`function` · `datafusion::test::scan_partitioned_csv` · datafusion 55.1.0

```rust
fn scan_partitioned_csv(partitions: usize, work_dir: &std::path::Path) -> error::Result<std::sync::Arc<datafusion_datasource::source::DataSourceExec>>
```

Source: `src/test/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a [`DataSourceExec`](../operations/datafusion_datasource.source.DataSourceExec.md#op-96b0a6eef9f3c044c580d9b6) that scans "aggregate_test_100.csv" with `partitions` partitions
