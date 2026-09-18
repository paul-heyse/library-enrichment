# `parquet::arrow::async_reader::metadata::MetadataFetch`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.metadata.MetadataFetch.json).

<a id="op-2f1a6adade5c3e921474d943"></a>
## MetadataFetch

`trait` · `parquet::arrow::async_reader::metadata::MetadataFetch` · parquet 59.3.0

```rust
trait MetadataFetch
```

Source: `src/arrow/async_reader/metadata.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

 A data source that can be used with [`ParquetMetaDataReader`] to load [`ParquetMetaData`]

 Note that implementation is provided for [`AsyncFileReader`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-69c2992a81570b27e07fe176).

 # Example `MetadataFetch` for a custom async data source

 ```rust
 # use parquet::errors::Result;
 # use parquet::arrow::async_reader::MetadataFetch;
 # use bytes::Bytes;
 # use std::ops::Range;
 # use std::io::SeekFrom;
 # use futures::future::BoxFuture;
 # use futures::FutureExt;
 # use tokio::io::{AsyncReadExt, AsyncSeekExt};
 // Adapter that implements the API for reading bytes from an async source (in
 // this case a tokio::fs::File)
 struct TokioFileMetadata {
     file: tokio::fs::File,
 }
 impl MetadataFetch for TokioFileMetadata {
     fn fetch(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>> {
         // return a future that fetches data in range
         async move {
             let len = (range.end - range.start).try_into().unwrap();
             let mut buf = vec![0; len]; // target buffer
             // seek to the start of the range and read the data
             self.file.seek(SeekFrom::Start(range.start)).await?;
             self.file.read_exact(&mut buf).await?;
             Ok(Bytes::from(buf)) // convert to Bytes
         }
             .boxed() // turn into BoxedFuture, using FutureExt::boxed
     }
 }
```

 [`ParquetMetaDataReader`]: crate::file::metadata::reader::ParquetMetaDataReader
 [`ParquetMetaData`]: crate::file::metadata::ParquetMetaData

<a id="op-df816694ef4933bd3a55671a"></a>
## fetch

`function` · `parquet::arrow::async_reader::metadata::MetadataFetch::fetch` · parquet 59.3.0

```rust
fn fetch(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
```

Source: `src/arrow/async_reader/metadata.rs:69`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a future that fetches the specified range of bytes asynchronously

Note the returned type is a boxed future, often created by
[`FutureExt::boxed`]. See the trait documentation for an example

[`FutureExt::boxed`]: futures::FutureExt::boxed

Unresolved upstream links (retained, not inferred): `futures::FutureExt::boxed`.
