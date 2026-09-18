# `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.reader.CachedParquetFileReaderFactory.json).

<a id="op-1f25246e1427ba8873528de4"></a>
## CachedParquetFileReaderFactory

`struct` · `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory` · datafusion-datasource-parquet 55.1.0

```rust
struct CachedParquetFileReaderFactory
```

Source: `src/reader.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Implementation of [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916) supporting the caching of footer and page
metadata. Reads and updates the [`FileMetadataCache`](../operations/datafusion_execution.cache.cache_manager.FileMetadataCache.md#op-eae1fe1b1bd732ee9949f01d) with the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) data.

[`ParquetFileReader::get_metadata`](../operations/datafusion_datasource_parquet.reader.ParquetFileReader.md#op-10d8159e1a8262872b27f141) forwards the [`parquet::file::metadata::PageIndexPolicy`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-7dc5da782b207c53dc34d37c) from
[`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4) to [`DFParquetMetadata::fetch_metadata`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-bd8af0811fa491114afd4542), so callers such as the
parquet opener can skip page-index I/O during the initial metadata load.

<a id="op-7a859510f0b2cdde2f446927"></a>
## create_reader

`function` · `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory::create_reader` · datafusion-datasource-parquet 55.1.0

```rust
fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile, metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet) -> datafusion_common::Result<Box<dyn AsyncFileReader + Send>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory", "path": "CachedParquetFileReaderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [163, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReaderFactory", "path": "ParquetFileReaderFactory"}, "trait_path": "datafusion_datasource_parquet::reader::ParquetFileReaderFactory"}`

Source: `src/reader.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-118f511541078a1e8e060bce"></a>
## fmt

`function` · `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory", "path": "CachedParquetFileReaderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-badef1e1c1fa637a0864060f"></a>
## new

`function` · `datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(store: Arc<dyn ObjectStore>, metadata_cache: Arc<FileMetadataCache>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetFileReaderFactory", "path": "CachedParquetFileReaderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [137, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
