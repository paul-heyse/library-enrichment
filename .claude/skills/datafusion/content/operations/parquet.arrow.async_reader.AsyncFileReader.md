# `parquet::arrow::async_reader::AsyncFileReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.AsyncFileReader.json).

<a id="op-69c2992a81570b27e07fe176"></a>
## AsyncFileReader

`trait` · `parquet::arrow::async_reader::AsyncFileReader` · parquet 59.3.0

```rust
trait AsyncFileReader: Send
```

Source: `src/arrow/async_reader/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The asynchronous interface used by [`ParquetRecordBatchStream`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-5aaf529c72b40fe254f58220) to read parquet files

Notes:

1. There is a default implementation for types that implement [`AsyncRead`]
   and [`AsyncSeek`], for example [`tokio::fs::File`].

2. Implementations for remote storage, such as the `object_store` crate,
   can implement this interface directly, typically by pairing a store
   handle with an object path and delegating [`Self::get_bytes`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-34c201af3541e9636b33a845) and
   [`Self::get_byte_ranges`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-db04d236525153b4a7585872) to ranged reads. [`SpawnedReader`](../operations/parquet.arrow.async_reader.spawn.SpawnedReader.md#op-e0eacd10964c1557d2522d82) can wrap
   such a reader to perform its I/O on a dedicated runtime, and
   [`ParquetMetaDataReader::with_arrow_reader_options`] simplifies
   implementing [`Self::get_metadata`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-c7b76ac1222f378c726ef92c).

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

Unresolved upstream links (retained, not inferred): ``AsyncSeek``, ``AsyncRead``.

<a id="op-db04d236525153b4a7585872"></a>
## get_byte_ranges

`function` · `parquet::arrow::async_reader::AsyncFileReader::get_byte_ranges` · parquet 59.3.0

```rust
fn get_byte_ranges(&mut self, ranges: Vec<Range<u64>>) -> BoxFuture<'_, Result<Vec<Bytes>>>
```

Source: `src/arrow/async_reader/mod.rs:167`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Retrieve multiple byte ranges. The default implementation will call `get_bytes` sequentially

<a id="op-34c201af3541e9636b33a845"></a>
## get_bytes

`function` · `parquet::arrow::async_reader::AsyncFileReader::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
```

Source: `src/arrow/async_reader/mod.rs:164`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Retrieve the bytes in `range`

<a id="op-c7b76ac1222f378c726ef92c"></a>
## get_metadata

`function` · `parquet::arrow::async_reader::AsyncFileReader::get_metadata` · parquet 59.3.0

```rust
fn get_metadata<'a>(&'a mut self, options: Option<&'a ArrowReaderOptions>) -> BoxFuture<'a, Result<Arc<ParquetMetaData>>>
```

Source: `src/arrow/async_reader/mod.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a future which results in the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) for this Parquet file.

This is an asynchronous operation as it may involve reading the file
footer and potentially other metadata from disk or a remote source.

Reading data from Parquet requires the metadata to understand the
schema, row groups, and location of pages within the file. This metadata
is stored primarily in the footer of the Parquet file, and can be read using
[`ParquetMetaDataReader`](../operations/parquet.file.metadata.reader.ParquetMetaDataReader.md#op-ecb8df4cd7e741eec25c5144).

However, implementations can significantly speed up reading Parquet by
supplying cached metadata or pre-fetched metadata via this API.

# Parameters
* `options`: Optional [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4) that may contain decryption
  and other options that affect how the metadata is read.
