# `parquet::arrow::async_writer::store`

Crate `parquet` · 1 public items · structured records in [`model/parquet.arrow.async_writer.store.json`](../model/parquet.arrow.async_writer.store.json)

## ParquetObjectWriter

`struct` · `parquet::arrow::async_writer::store::ParquetObjectWriter`

> **Deprecated** — since 59.2.0: Pass an `object_store::buffered::BufWriter` to `AsyncArrowWriter` directly instead; see https://github.com/apache/arrow-rs/issues/10308 and `parquet/examples/object_store.rs`.

```rust
struct ParquetObjectWriter
```

**Implements**: `core::convert::From`, `parquet::arrow::async_writer::AsyncFileWriter`

**Derives**: Debug

**Methods** (3)

```rust
fn from_buf_writer(w: BufWriter) -> Self
fn into_inner(self) -> BufWriter
fn new(store: Arc<dyn ObjectStore>, path: Path) -> Self
```

**via `core::convert::From`**

```rust
fn from(w: BufWriter) -> Self
```

**via `parquet::arrow::async_writer::AsyncFileWriter`**

```rust
fn complete(&mut self) -> BoxFuture<'_, Result<()>>
fn write(&mut self, bs: Bytes) -> BoxFuture<'_, Result<()>>
```

[`ParquetObjectWriter`] for writing to parquet to [`ObjectStore`]

This type is deprecated: [`BufWriter`] implements [`AsyncWrite`] and can
therefore be passed to [`AsyncArrowWriter`] directly via the blanket
[`AsyncFileWriter`] implementation for [`AsyncWrite`] types:

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

---
