# `parquet::arrow::async_writer::store::ParquetObjectWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_writer.store.ParquetObjectWriter.json).

<a id="op-8e75eaffdac8304a2df1acdd"></a>
## ParquetObjectWriter

`struct` · `parquet::arrow::async_writer::store::ParquetObjectWriter` · parquet 59.3.0

```rust
struct ParquetObjectWriter
```

Source: `src/arrow/async_writer/store.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[`ParquetObjectWriter`](../operations/parquet.arrow.async_writer.store.ParquetObjectWriter.md#op-8e75eaffdac8304a2df1acdd) for writing to parquet to [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)

This type is deprecated: [`BufWriter`](../operations/object_store.buffered.BufWriter.md#op-f092e35da4d4adde7c2b58f9) implements [`AsyncWrite`] and can
therefore be passed to [`AsyncArrowWriter`] directly via the blanket
[`AsyncFileWriter`](../operations/parquet.arrow.async_writer.AsyncFileWriter.md#op-d1acaf94f7f04028351c03f9) implementation for [`AsyncWrite`] types:

```
# use arrow_array::{ArrayRef, Int64Array, RecordBatch};
# use object_store::buffered::BufWriter;
# use object_store::memory::InMemory;
# use object_store::path::Path;
# use object_store::{ObjectStore, ObjectStoreExt};
# use std::sync::Arc;

# use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
# use parquet::arrow::AsyncArrowWriter;

# #[tokio::main(flavor="current_thread")]
# async fn main() {
    let store = Arc::new(InMemory::new());

    let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
    let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

    let object_store_writer = BufWriter::new(store.clone(), Path::from("test"));
    let mut writer =
        AsyncArrowWriter::try_new(object_store_writer, to_write.schema(), None).unwrap();
    writer.write(&to_write).await.unwrap();
    writer.close().await.unwrap();

    let buffer = store
        .get(&Path::from("test"))
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let mut reader = ParquetRecordBatchReaderBuilder::try_new(buffer)
        .unwrap()
        .build()
        .unwrap();
    let read = reader.next().unwrap().unwrap();

    assert_eq!(to_write, read);
# }
```

[`AsyncWrite`]: tokio::io::AsyncWrite
[`AsyncArrowWriter`]: crate::arrow::async_writer::AsyncArrowWriter

Unresolved upstream links (retained, not inferred): `tokio::io::AsyncWrite`.

<a id="op-ef3af8bfb25a5cb79d072f98"></a>
## complete

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::complete` · parquet 59.3.0

```rust
fn complete(&mut self) -> BoxFuture<'_, Result<()>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [126, 2], "filename": "src/arrow/async_writer/store.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}, "trait_path": "parquet::arrow::async_writer::AsyncFileWriter"}`

Source: `src/arrow/async_writer/store.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3550327b13bc2965bd499a8"></a>
## fmt

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 10], "end": [82, 15], "filename": "src/arrow/async_writer/store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/async_writer/store.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f258531909fbfb3deaf960e2"></a>
## from

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::from` · parquet 59.3.0

```rust
fn from(w: BufWriter) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [132, 2], "filename": "src/arrow/async_writer/store.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::buffered::BufWriter", "path": "BufWriter"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/arrow/async_writer/store.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2f55ce10cf8bd6c9179a0ba"></a>
## from_buf_writer

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::from_buf_writer` · parquet 59.3.0

```rust
fn from_buf_writer(w: BufWriter) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [105, 2], "filename": "src/arrow/async_writer/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/store.rs:97`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Construct a new ParquetObjectWriter via a existing BufWriter.

<a id="op-ec940fe5e853bff959e06d2b"></a>
## into_inner

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> BufWriter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [105, 2], "filename": "src/arrow/async_writer/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/store.rs:102`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Consume the writer and return the underlying BufWriter.

<a id="op-db267944b3a9f0f03fb27c05"></a>
## new

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::new` · parquet 59.3.0

```rust
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [105, 2], "filename": "src/arrow/async_writer/store.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_writer/store.rs:92`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetObjectWriter`](../operations/parquet.arrow.async_writer.store.ParquetObjectWriter.md#op-8e75eaffdac8304a2df1acdd) that writes to the specified path in the given store.

To configure the writer behavior, please build [`BufWriter`](../operations/object_store.buffered.BufWriter.md#op-f092e35da4d4adde7c2b58f9) and then use [`Self::from_buf_writer`](../operations/parquet.arrow.async_writer.store.ParquetObjectWriter.md#op-f2f55ce10cf8bd6c9179a0ba)

<a id="op-e1bbfbdfe06d037188d537f2"></a>
## write

`function` · `parquet::arrow::async_writer::store::ParquetObjectWriter::write` · parquet 59.3.0

```rust
fn write(&mut self, bs: Bytes) -> BoxFuture<'_, Result<()>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::async_writer::store::ParquetObjectWriter", "path": "ParquetObjectWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [126, 2], "filename": "src/arrow/async_writer/store.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_writer::AsyncFileWriter", "path": "AsyncFileWriter"}, "trait_path": "parquet::arrow::async_writer::AsyncFileWriter"}`

Source: `src/arrow/async_writer/store.rs:109`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
