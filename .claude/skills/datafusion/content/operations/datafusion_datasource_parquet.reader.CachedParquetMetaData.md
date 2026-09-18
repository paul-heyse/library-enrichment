# `datafusion_datasource_parquet::reader::CachedParquetMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.reader.CachedParquetMetaData.json).

<a id="op-3e2d0157ba6f5f8e1719a195"></a>
## CachedParquetMetaData

`struct` · `datafusion_datasource_parquet::reader::CachedParquetMetaData` · datafusion-datasource-parquet 55.1.0

```rust
struct CachedParquetMetaData
```

Source: `src/reader.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Wrapper to implement [`FileMetadata`](../operations/datafusion_execution.cache.cache_manager.FileMetadata.md#op-2f003a3649d7b4b496dab583) for [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b).

<a id="op-f1b420a8552076afe4afaec2"></a>
## as_any

`function` · `datafusion_datasource_parquet::reader::CachedParquetMetaData::as_any` · datafusion-datasource-parquet 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [344, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::cache_manager::FileMetadata", "path": "FileMetadata"}, "trait_path": "datafusion_execution::cache::cache_manager::FileMetadata"}`

Source: `src/reader.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b6bd3b6797c311dddc36c1d"></a>
## extra_info

`function` · `datafusion_datasource_parquet::reader::CachedParquetMetaData::extra_info` · datafusion-datasource-parquet 55.1.0

```rust
fn extra_info(&self) -> HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [344, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::cache_manager::FileMetadata", "path": "FileMetadata"}, "trait_path": "datafusion_execution::cache::cache_manager::FileMetadata"}`

Source: `src/reader.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cb7f648bfe975525926dccf"></a>
## memory_size

`function` · `datafusion_datasource_parquet::reader::CachedParquetMetaData::memory_size` · datafusion-datasource-parquet 55.1.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [344, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::cache_manager::FileMetadata", "path": "FileMetadata"}, "trait_path": "datafusion_execution::cache::cache_manager::FileMetadata"}`

Source: `src/reader.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71182f18dd1ba74e6984b825"></a>
## new

`function` · `datafusion_datasource_parquet::reader::CachedParquetMetaData::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(metadata: Arc<ParquetMetaData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [328, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e990d5c02499cf3126f6b250"></a>
## parquet_metadata

`function` · `datafusion_datasource_parquet::reader::CachedParquetMetaData::parquet_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn parquet_metadata(&self) -> &Arc<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [328, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
