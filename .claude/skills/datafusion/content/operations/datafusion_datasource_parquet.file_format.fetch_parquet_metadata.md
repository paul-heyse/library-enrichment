# `datafusion_datasource_parquet::file_format::fetch_parquet_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.file_format.fetch_parquet_metadata.json).

<a id="op-ab47d0a73ae9247b2985cc4d"></a>
## fetch_parquet_metadata

`function` · `datafusion_datasource_parquet::file_format::fetch_parquet_metadata` · datafusion-datasource-parquet 55.1.0

```rust
async fn fetch_parquet_metadata(store: &dyn ObjectStore, object_meta: &object_store::ObjectMeta, size_hint: Option<usize>, decryption_properties: Option<&datafusion_common::encryption::FileDecryptionProperties>, file_metadata_cache: Option<std::sync::Arc<datafusion_execution::cache::cache_manager::FileMetadataCache>>) -> datafusion_common::Result<std::sync::Arc<parquet::file::metadata::ParquetMetaData>>
```

Source: `src/file_format.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Fetches parquet metadata from ObjectStore for given object

This component is a subject to **change** in near future and is exposed for low level integrations
through [`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: crate::ParquetFileReaderFactory
