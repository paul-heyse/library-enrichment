# `parquet::arrow::async_reader::metadata`

Crate `parquet` · 2 public items · structured records in [`model/parquet.arrow.async_reader.metadata.json`](../model/parquet.arrow.async_reader.metadata.json)

## MetadataFetch

`trait` · `parquet::arrow::async_reader::metadata::MetadataFetch`

```rust
trait MetadataFetch
```

**Implementors** (1)

- `datafusion_datasource_parquet::file_format::ObjectStoreFetch`

**Methods** (1)

```rust
fn fetch(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_reader.metadata.MetadataFetch.md).


 A data source that can be used with [`ParquetMetaDataReader`] to load [`ParquetMetaData`]

 Note that implementation is provided for [`AsyncFileReader`].

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

---

## MetadataSuffixFetch

`trait` · `parquet::arrow::async_reader::metadata::MetadataSuffixFetch`

```rust
trait MetadataSuffixFetch: MetadataFetch
```

**Methods** (1)

```rust
fn fetch_suffix(&mut self, suffix: usize) -> BoxFuture<'_, Result<Bytes>>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.async_reader.metadata.MetadataSuffixFetch.md).


A data source that can be used with [`ParquetMetaDataReader`] to load [`ParquetMetaData`] via suffix
requests, without knowing the file size

[`ParquetMetaDataReader`]: crate::file::metadata::reader::ParquetMetaDataReader
[`ParquetMetaData`]: crate::file::metadata::ParquetMetaData

---
