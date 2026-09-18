# `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.reader.DefaultParquetFileReaderFactory.json).

<a id="op-f7b9d0e1ab2507dcbfb1a612"></a>
## DefaultParquetFileReaderFactory

`struct` · `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory` · datafusion-datasource-parquet 55.1.0

```rust
struct DefaultParquetFileReaderFactory
```

Source: `src/reader.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Default implementation of [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916)

This implementation:
1. Reads parquet directly from an underlying [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) instance.
2. Reads the footer and page metadata on demand.
3. Does not cache metadata or coalesce I/O operations.

<a id="op-b5f2266c23d4e9aa545519fe"></a>
## create_reader

`function` · `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory::create_reader` · datafusion-datasource-parquet 55.1.0

```rust
fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile, metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet) -> datafusion_common::Result<Box<dyn AsyncFileReader + Send>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory", "path": "DefaultParquetFileReaderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [113, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReaderFactory", "path": "ParquetFileReaderFactory"}, "trait_path": "datafusion_datasource_parquet::reader::ParquetFileReaderFactory"}`

Source: `src/reader.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88e157e4deb42c95835be8c4"></a>
## fmt

`function` · `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory", "path": "DefaultParquetFileReaderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 10], "end": [79, 15], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-493cad3daaf0a28360c9bb9b"></a>
## new

`function` · `datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(store: Arc<dyn ObjectStore>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::DefaultParquetFileReaderFactory", "path": "DefaultParquetFileReaderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [89, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `DefaultParquetFileReaderFactory`.
