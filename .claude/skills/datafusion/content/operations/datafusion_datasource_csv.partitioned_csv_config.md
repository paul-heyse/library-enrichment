# `datafusion_datasource_csv::partitioned_csv_config`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.partitioned_csv_config.json).

<a id="op-48d688e003d7d651c702da30"></a>
## partitioned_csv_config

`function` · `datafusion_datasource_csv::partitioned_csv_config` · datafusion-datasource-csv 55.1.0

```rust
fn partitioned_csv_config(file_groups: Vec<datafusion_datasource::file_groups::FileGroup>, file_source: std::sync::Arc<dyn FileSource>) -> datafusion_common::Result<datafusion_datasource::file_scan_config::FileScanConfig>
```

Source: `src/mod.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Returns a [`FileScanConfig`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-d7aa66cd63851cc944f55f57) for given `file_groups`
