# `parquet::arrow::async_reader::store::ParquetObjectReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.store.ParquetObjectReader.json).

<a id="op-1c69fe071d041f945a91ea29"></a>
## ParquetObjectReader

`struct` · `parquet::arrow::async_reader::store::ParquetObjectReader` · parquet 59.3.0

```rust
struct ParquetObjectReader
```

Source: `src/arrow/async_reader/store.rs:59`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reads Parquet files in object storage using [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca).

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

<a id="op-372714b4e92d7390e9b2fb96"></a>
## clone

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ParquetObjectReader
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/arrow/async_reader/store.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/async_reader/store.rs:58`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a437cceff04cb7b31ab4e003"></a>
## fmt

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 17], "end": [58, 22], "filename": "src/arrow/async_reader/store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/async_reader/store.rs:58`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b704a38a81d74c74da3e0408"></a>
## get_byte_ranges

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::get_byte_ranges` · parquet 59.3.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>>> where Self: Send
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [258, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/arrow/async_reader/store.rs:203`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3499ff49889ee5fd300b8b0"></a>
## get_bytes

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [258, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/arrow/async_reader/store.rs:199`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72201af2a8db6ced1031c122"></a>
## get_metadata

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::get_metadata` · parquet 59.3.0

```rust
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [258, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}, "trait_path": "parquet::arrow::async_reader::AsyncFileReader"}`

Source: `src/arrow/async_reader/store.rs:216`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29501e97a64f415c0222d18e"></a>
## new

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::new` · parquet 59.3.0

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [178, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/store.rs:72`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new [`ParquetObjectReader`](../operations/parquet.arrow.async_reader.store.ParquetObjectReader.md#op-1c69fe071d041f945a91ea29) for the provided [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) and [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b).

<a id="op-48fdd10f4a2b294694ea4eb4"></a>
## with_file_size

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::with_file_size` · parquet 59.3.0

```rust
fn with_file_size(self, file_size: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [178, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/store.rs:102`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide the byte size of this file.

If provided, the file size will ensure that only bounded range requests are used. If file
size is not provided, the reader will use suffix range requests to fetch the metadata.

Providing this size up front is an important optimization to avoid extra calls when the
underlying store does not support suffix range requests.

The file size can be obtained using [`ObjectStore::list`](../operations/object_store.ObjectStore.md#op-eaf50d65c0fa0bf3019d428b) or [`ObjectStoreExt::head`](../operations/object_store.ObjectStoreExt.md#op-e14b1b9ae2393005fc5a5a50).

<a id="op-15a446df98dbd82b2c4f69fd"></a>
## with_footer_size_hint

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::with_footer_size_hint` · parquet 59.3.0

```rust
fn with_footer_size_hint(self, hint: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [178, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/store.rs:86`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a hint as to the size of the parquet file's footer,
see [`ParquetMetaDataReader::with_prefetch_hint`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-836e78360f27ee2a739d1678)

<a id="op-2b19248985a5ef3e3ca70207"></a>
## with_preload_column_index

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::with_preload_column_index` · parquet 59.3.0

```rust
fn with_preload_column_index(self, preload_column_index: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [178, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/store.rs:114`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether to load the Column Index as part of [`Self::get_metadata`](../operations/parquet.arrow.async_reader.store.ParquetObjectReader.md#op-72201af2a8db6ced1031c122)

Note: This setting may be overridden by [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4) `page_index_policy`.
If `page_index_policy` is `Optional` or `Required`, it will take precedence
over this preload flag. When it is `Skip` (default), this flag is used.

<a id="op-1743562db5d699efb8fb90d5"></a>
## with_preload_offset_index

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::with_preload_offset_index` · parquet 59.3.0

```rust
fn with_preload_offset_index(self, preload_offset_index: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [178, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/store.rs:126`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether to load the Offset Index as part of [`Self::get_metadata`](../operations/parquet.arrow.async_reader.store.ParquetObjectReader.md#op-72201af2a8db6ced1031c122)

Note: This setting may be overridden by [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4) `page_index_policy`.
If `page_index_policy` is `Optional` or `Required`, it will take precedence
over this preload flag. When it is `Skip` (default), this flag is used.

<a id="op-c24bdf69c07e5b7749ab936e"></a>
## with_runtime

`function` · `parquet::arrow::async_reader::store::ParquetObjectReader::with_runtime` · parquet 59.3.0

```rust
fn with_runtime(self, handle: Handle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_reader::store::ParquetObjectReader", "path": "ParquetObjectReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [178, 2], "filename": "src/arrow/async_reader/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/store.rs:145`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Perform IO on the provided tokio runtime

Tokio is a cooperative scheduler, and relies on tasks yielding in a timely manner
to service IO. Therefore, running IO and CPU-bound tasks, such as parquet decoding,
on the same tokio runtime can lead to degraded throughput, dropped connections and
other issues. For more information see [here].

[here]: https://www.influxdata.com/blog/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/
