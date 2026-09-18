# `datafusion::test_util::populate_csv_partitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.populate_csv_partitions.json).

<a id="op-f87c411c507284e82ffaaed0"></a>
## populate_csv_partitions

`function` · `datafusion::test_util::populate_csv_partitions` · datafusion 55.1.0

```rust
fn populate_csv_partitions(tmp_dir: &tempfile::TempDir, partition_count: usize, file_extension: &str) -> error::Result<arrow::datatypes::SchemaRef>
```

Source: `src/test_util/mod.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Generate CSV partitions within the supplied directory
