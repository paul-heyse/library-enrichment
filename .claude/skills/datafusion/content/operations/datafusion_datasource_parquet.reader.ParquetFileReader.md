# `datafusion_datasource_parquet::reader::ParquetFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.reader.ParquetFileReader.json).

<a id="op-af09614b104f848d29427d14"></a>
## ParquetFileReader

`struct` · `datafusion_datasource_parquet::reader::ParquetFileReader` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetFileReader
```

Source: `src/reader.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Implements [`AsyncFileReader`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-69c2992a81570b27e07fe176) for a parquet file in object storage.

This implementation reads data directly from the underlying [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)
on demand, as required, tracking the number of bytes read.

When configured via [`Self::with_metadata_cache`](../operations/datafusion_datasource_parquet.reader.ParquetFileReader.md#op-33e7840ca695b5f9929e9052), [`Self::get_metadata`](../operations/datafusion_datasource_parquet.reader.ParquetFileReader.md#op-10d8159e1a8262872b27f141)
reads footer and page metadata from the cache when available and populates
the cache otherwise. Without a cache, metadata is fetched fresh on every call.

# Notes

This implementation does not coalesce I/O operations or cache bytes. Such
optimizations can be done either at the object store level or by providing
a custom implementation of [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916).

<a id="op-457c1697f1377e59be587119"></a>
## drop

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::drop` · datafusion-datasource-parquet 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 1], "end": [315, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/reader.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-398a77f33576692bdfb4bae9"></a>
## file_metrics

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::file_metrics` · datafusion-datasource-parquet 55.1.0

```rust
fn file_metrics(&self) -> &ParquetFileMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [236, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns the metrics tracked while reading this file.

<a id="op-7a0e1c9cbb2928ac4d4723d3"></a>
## get_byte_ranges

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::get_byte_ranges` · datafusion-datasource-parquet 55.1.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, parquet::errors::Result<Vec<Bytes>>> where Self: Send
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [303, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/reader.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f116f2cb8d3a2bba65e094ad"></a>
## get_bytes

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::get_bytes` · datafusion-datasource-parquet 55.1.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, parquet::errors::Result<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [303, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/reader.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10d8159e1a8262872b27f141"></a>
## get_metadata

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::get_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, parquet::errors::Result<Arc<ParquetMetaData>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [303, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/reader.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-652fbc7867788088f6e73ec7"></a>
## partitioned_file

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::partitioned_file` · datafusion-datasource-parquet 55.1.0

```rust
fn partitioned_file(&self) -> &PartitionedFile
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [236, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Returns the file this reader is reading.

<a id="op-33e7840ca695b5f9929e9052"></a>
## with_metadata_cache

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::with_metadata_cache` · datafusion-datasource-parquet 55.1.0

```rust
fn with_metadata_cache(self, metadata_cache: Option<Arc<FileMetadataCache>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [236, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the [`FileMetadataCache`](../operations/datafusion_execution.cache.cache_manager.FileMetadataCache.md#op-eae1fe1b1bd732ee9949f01d) for this reader

<a id="op-8aac10bc4de015e01675f59b"></a>
## with_metadata_hint

`function` · `datafusion_datasource_parquet::reader::ParquetFileReader::with_metadata_hint` · datafusion-datasource-parquet 55.1.0

```rust
fn with_metadata_hint(self, metadata_size_hint: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::reader::ParquetFileReader", "path": "ParquetFileReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 1], "end": [236, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the metadata size hint for this reader.

See [`DFParquetMetadata::with_metadata_size_hint`](../operations/datafusion_datasource_parquet.metadata.DFParquetMetadata.md#op-c01c81fd7e994053361abf00) for more details.
