# `parquet::arrow::async_reader`

Crate `parquet` · 3 public items · structured records in [`model/parquet.arrow.async_reader.json`](../model/parquet.arrow.async_reader.json)

## ParquetRecordBatchStream

`struct` · `parquet::arrow::async_reader::ParquetRecordBatchStream`

```rust
struct ParquetRecordBatchStream<T>
```

**Implements**: `futures_core::stream::Stream`

**Derives**: Debug

**Methods** (2)

```rust
async fn next_row_group(&mut self) -> Result<Option<ParquetRecordBatchReader>>
fn schema(&self) -> &SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md).


An asynchronous [`Stream`]of [`RecordBatch`] constructed using [`ParquetRecordBatchStreamBuilder`] to read parquet files.

`ParquetRecordBatchStream` also provides [`ParquetRecordBatchStream::next_row_group`] for fetching row groups,
allowing users to decode record batches separately from I/O.

# I/O Buffering

`ParquetRecordBatchStream` buffers *all* data pages selected after predicates
(projection + filtering, etc) and decodes the rows from those buffered pages.

For example, if all rows and columns are selected, the entire row group is
buffered in memory during decode. This minimizes the number of IO operations
required, which is especially important for object stores, where IO operations
have latencies in the hundreds of milliseconds

See [`ParquetPushDecoderBuilder`] for an API with lower level control over
buffering.

[`Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html

---

## AsyncFileReader

`trait` · `parquet::arrow::async_reader::AsyncFileReader`

```rust
trait AsyncFileReader: Send
```

**Implementors** (4)

- `alloc::boxed::Box`
- `datafusion_datasource_parquet::reader::ParquetFileReader`
- `parquet::arrow::async_reader::spawn::SpawnedReader`
- `parquet::arrow::async_reader::store::ParquetObjectReader`

**Methods** (3)

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>>>
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_reader.AsyncFileReader.md).


The asynchronous interface used by [`ParquetRecordBatchStream`] to read parquet files

Notes:

1. There is a default implementation for types that implement [`AsyncRead`]
   and [`AsyncSeek`], for example [`tokio::fs::File`].

2. Implementations for remote storage, such as the `object_store` crate,
   can implement this interface directly, typically by pairing a store
   handle with an object path and delegating [`Self::get_bytes`] and
   [`Self::get_byte_ranges`] to ranged reads. [`SpawnedReader`] can wrap
   such a reader to perform its I/O on a dedicated runtime, and
   [`ParquetMetaDataReader::with_arrow_reader_options`] simplifies
   implementing [`Self::get_metadata`].

# Example: implementing `AsyncFileReader` for the `object_store` crate

```no_run
# use std::ops::Range;
# use std::sync::Arc;
use bytes::Bytes;
use futures::future::BoxFuture;
use futures::{FutureExt, TryFutureExt};
use object_store::path::Path;
use object_store::{GetOptions, GetRange, ObjectStore, ObjectStoreExt};
use parquet::arrow::arrow_reader::ArrowReaderOptions;
use parquet::arrow::async_reader::{AsyncFileReader, MetadataSuffixFetch};
use parquet::errors::{ParquetError, Result};
use parquet::file::metadata::{ParquetMetaData, ParquetMetaDataReader};

fn to_parquet_err(e: object_store::Error) -> ParquetError {
    ParquetError::External(Box::new(e))
}

#[derive(Clone)]
struct ObjectStoreReader {
    store: Arc<dyn ObjectStore>,
    path: Path,
}

impl AsyncFileReader for ObjectStoreReader {
    fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>> {
        self.store
            .get_range(&self.path, range)
            .map_err(to_parquet_err)
            .boxed()
    }

    fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>>> {
        async move {
            self.store
                .get_ranges(&self.path, &ranges)
                .await
                .map_err(to_parquet_err)
        }
        .boxed()
    }

    fn get_metadata<'a>(
        &'a mut self,
        options: Option<&'a ArrowReaderOptions>,
    ) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>> {
        async move {
            let metadata = ParquetMetaDataReader::new()
                .with_arrow_reader_options(options)
                .load_via_suffix_and_finish(self)
                .await?;
            Ok(Arc::new(metadata))
        }
        .boxed()
    }
}

/// Supports fetching the parquet footer without knowing the file size,
/// via suffix range requests
impl MetadataSuffixFetch for &mut ObjectStoreReader {
    fn fetch_suffix(&mut self, suffix: usize) -> BoxFuture<'_, Result<Bytes>> {
        let options = GetOptions {
            range: Some(GetRange::Suffix(suffix as u64)),
            ..Default::default()
        };
        async move {
            let resp = self
                .store
                .get_opts(&self.path, options)
                .await
                .map_err(to_parquet_err)?;
            resp.bytes().await.map_err(to_parquet_err)
        }
        .boxed()
    }
}
```

[`ParquetMetaDataReader::with_arrow_reader_options`]: crate::file::metadata::ParquetMetaDataReader::with_arrow_reader_options

[`tokio::fs::File`]: https://docs.rs/tokio/latest/tokio/fs/struct.File.html

---

## ParquetRecordBatchStreamBuilder

`type_alias` · `parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder`

Also reachable as `parquet::arrow::ParquetRecordBatchStreamBuilder`

```rust
type ParquetRecordBatchStreamBuilder<T> = arrow::arrow_reader::ArrowReaderBuilder<AsyncReader<T>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.md).


A builder for reading parquet files from an `async` source as  [`ParquetRecordBatchStream`]

This can be used to decode a Parquet file in streaming fashion (without
downloading the whole file at once) from a remote source, such as an object store.

This builder handles reading the parquet file metadata, allowing consumers
to use this information to select what specific columns, row groups, etc.
they wish to be read by the resulting stream.

See examples on [`ParquetRecordBatchStreamBuilder::new`], including how to
issue multiple I/O requests in parallel using multiple streams.

# See also:
* [`ParquetPushDecoderBuilder`] for lower level control over buffering and
  decoding.
* [`ParquetRecordBatchStream::next_row_group`] for I/O prefetching


See [`ArrowReaderBuilder`] for additional member functions

---
