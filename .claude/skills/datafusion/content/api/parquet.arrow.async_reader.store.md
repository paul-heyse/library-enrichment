# `parquet::arrow::async_reader::store`

Crate `parquet` · 1 public items · structured records in [`model/parquet.arrow.async_reader.store.json`](../model/parquet.arrow.async_reader.store.json)

## ParquetObjectReader

`struct` · `parquet::arrow::async_reader::store::ParquetObjectReader`

> **Deprecated** — since 59.2.0: Implement `AsyncFileReader` directly instead; see the example on the `AsyncFileReader` trait documentation and `parquet/examples/object_store.rs`. Use `SpawnedReader` to perform I/O on a dedicated runtime. See https://github.com/apache/arrow-rs/issues/10308

```rust
struct ParquetObjectReader
```

**Implements**: `parquet::arrow::async_reader::AsyncFileReader`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
fn with_file_size(self, file_size: u64) -> Self
fn with_footer_size_hint(self, hint: usize) -> Self
fn with_preload_column_index(self, preload_column_index: bool) -> Self
fn with_preload_offset_index(self, preload_offset_index: bool) -> Self
fn with_runtime(self, handle: Handle) -> Self
```

**via `parquet::arrow::async_reader::AsyncFileReader`**

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>>> where Self: Send
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

Reads Parquet files in object storage using [`ObjectStore`].

```no_run
# use std::io::stdout;
# use std::sync::Arc;
# use object_store::azure::MicrosoftAzureBuilder;
# use object_store::{ObjectStore, ObjectStoreExt};
# use object_store::path::Path;
# use parquet::arrow::async_reader::ParquetObjectReader;
# use parquet::arrow::ParquetRecordBatchStreamBuilder;
# use parquet::schema::printer::print_parquet_metadata;
# async fn run() {
// Populate configuration from environment
let storage_container = Arc::new(MicrosoftAzureBuilder::from_env().build().unwrap());
let location = Path::from("path/to/blob.parquet");
let meta = storage_container.head(&location).await.unwrap();
println!("Found Blob with {}B at {}", meta.size, meta.location);

// Show Parquet metadata
let reader = ParquetObjectReader::new(storage_container, meta.location).with_file_size(meta.size);
let builder = ParquetRecordBatchStreamBuilder::new(reader).await.unwrap();
print_parquet_metadata(&mut stdout(), builder.metadata());
# }
```

---
