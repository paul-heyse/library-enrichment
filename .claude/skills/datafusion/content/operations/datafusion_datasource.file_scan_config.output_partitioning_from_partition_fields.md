# `datafusion_datasource::file_scan_config::output_partitioning_from_partition_fields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_scan_config.output_partitioning_from_partition_fields.json).

<a id="op-2b2fe51d7b7270423fe8925a"></a>
## output_partitioning_from_partition_fields

`function` · `datafusion_datasource::file_scan_config::output_partitioning_from_partition_fields` · datafusion-datasource 55.1.0

```rust
fn output_partitioning_from_partition_fields(schema: &arrow::datatypes::Schema, partition_cols: &arrow::datatypes::Fields, partition_count: usize) -> Option<datafusion_physical_expr::Partitioning>
```

Source: `src/file_scan_config/mod.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Builds output partitioning over `partition_cols` (resolved to their indices in
`schema`) with `partition_count` partitions. Returns `None` when there are no
partition columns. Callers use this to declare the output partitioning of a scan
whose file groups are organized by partition column values.
