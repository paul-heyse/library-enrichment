# `datafusion_datasource_parquet::file_format::fetch_statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.file_format.fetch_statistics.json).

<a id="op-7fbefad3c995e7a6753557ca"></a>
## fetch_statistics

`function` · `datafusion_datasource_parquet::file_format::fetch_statistics` · datafusion-datasource-parquet 55.1.0

```rust
async fn fetch_statistics(store: &dyn ObjectStore, table_schema: arrow::datatypes::SchemaRef, file: &object_store::ObjectMeta, metadata_size_hint: Option<usize>, decryption_properties: Option<&datafusion_common::encryption::FileDecryptionProperties>, file_metadata_cache: Option<std::sync::Arc<datafusion_execution::cache::cache_manager::FileMetadataCache>>) -> datafusion_common::Result<datafusion_common::Statistics>
```

Source: `src/file_format.rs:663`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Read and parse the statistics of the Parquet file at location `path`

See [`statistics_from_parquet_meta_calc`](../operations/datafusion_datasource_parquet.file_format.statistics_from_parquet_meta_calc.md#op-620d68e4dacb14f283ffd979) for more details
