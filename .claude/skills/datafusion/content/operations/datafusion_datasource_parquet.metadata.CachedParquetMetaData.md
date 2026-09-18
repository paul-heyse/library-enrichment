# `datafusion_datasource_parquet::metadata::CachedParquetMetaData`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.metadata.CachedParquetMetaData.json).

<a id="op-ca7b70de15dd3942dfdda7de"></a>
## CachedParquetMetaData

`struct` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData` · datafusion-datasource-parquet 55.1.0

```rust
struct CachedParquetMetaData
```

Source: `src/metadata.rs:926`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Wrapper to implement [`FileMetadata`](../operations/datafusion_execution.cache.cache_manager.FileMetadata.md#op-2f003a3649d7b4b496dab583) for [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b).

<a id="op-7810db75ad4720087658e24d"></a>
## as_any

`function` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData::as_any` · datafusion-datasource-parquet 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metadata::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [952, 2], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::cache_manager::FileMetadata", "path": "FileMetadata"}, "trait_path": "datafusion_execution::cache::cache_manager::FileMetadata"}`

Source: `src/metadata.rs:939`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1e3fd5c45e71f3e087b9108"></a>
## extra_info

`function` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData::extra_info` · datafusion-datasource-parquet 55.1.0

```rust
fn extra_info(&self) -> HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metadata::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [952, 2], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::cache_manager::FileMetadata", "path": "FileMetadata"}, "trait_path": "datafusion_execution::cache::cache_manager::FileMetadata"}`

Source: `src/metadata.rs:947`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f6bb45d39c482e122a6ba6c"></a>
## memory_size

`function` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData::memory_size` · datafusion-datasource-parquet 55.1.0

```rust
fn memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metadata::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [952, 2], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::cache_manager::FileMetadata", "path": "FileMetadata"}, "trait_path": "datafusion_execution::cache::cache_manager::FileMetadata"}`

Source: `src/metadata.rs:943`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b01616ea2743057d708b14fe"></a>
## new

`function` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(metadata: Arc<ParquetMetaData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metadata::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [928, 1], "end": [936, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:929`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-830f595d557f4033bdf3e233"></a>
## parquet_metadata

`function` · `datafusion_datasource_parquet::metadata::CachedParquetMetaData::parquet_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn parquet_metadata(&self) -> &Arc<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metadata::CachedParquetMetaData", "path": "CachedParquetMetaData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [928, 1], "end": [936, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:933`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
