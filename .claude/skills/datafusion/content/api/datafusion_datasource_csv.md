# `datafusion_datasource_csv`

Crate `datafusion-datasource-csv` · 1 public items · structured records in [`model/datafusion_datasource_csv.json`](../model/datafusion_datasource_csv.json)

## partitioned_csv_config

`function` · `datafusion_datasource_csv::partitioned_csv_config`

```rust
fn partitioned_csv_config(file_groups: Vec<datafusion_datasource::file_groups::FileGroup>, file_source: std::sync::Arc<dyn FileSource>) -> datafusion_common::Result<datafusion_datasource::file_scan_config::FileScanConfig>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_csv.partitioned_csv_config.md).


Returns a [`FileScanConfig`] for given `file_groups`

---
