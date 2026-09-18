# `deltalake_core::logstore::parquet_reader`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.logstore.parquet_reader.json`](../model/deltalake_core.logstore.parquet_reader.json)

## ParquetObjectReader

`struct` · `deltalake_core::logstore::parquet_reader::ParquetObjectReader`
[Full member contracts, output types and access classification](../operations/deltalake_core.logstore.parquet_reader.ParquetObjectReader.md)

Also reachable as `deltalake::logstore::parquet_reader::ParquetObjectReader`

```rust
struct ParquetObjectReader
```

**Implements**: `parquet::arrow::async_reader::AsyncFileReader`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
fn with_file_size(self, file_size: u64) -> Self
fn with_footer_size_hint(self, hint: usize) -> Self
fn with_preload_column_index(self, preload: bool) -> Self
fn with_preload_offset_index(self, preload: bool) -> Self
```

**via `parquet::arrow::async_reader::AsyncFileReader`**

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

Reads Parquet files in object storage using [`ObjectStore`].

This struct provides a simple implementation of [`AsyncFileReader`] that
can be used with [`ParquetRecordBatchStreamBuilder`].

# Example

```no_run
# use std::sync::Arc;
# use deltalake_core::logstore::parquet_reader::ParquetObjectReader;
# use object_store::{ObjectStore, path::Path};
# use parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder;
# async fn run() {
# let store: Arc<dyn ObjectStore> = todo!();
# let location: Path = todo!();
# let file_size: u64 = todo!();
let reader = ParquetObjectReader::new(store, location).with_file_size(file_size);
let builder = ParquetRecordBatchStreamBuilder::new(reader).await.unwrap();
# }
```

---
