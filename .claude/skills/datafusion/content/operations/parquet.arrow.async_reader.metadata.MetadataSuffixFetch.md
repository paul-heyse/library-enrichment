# `parquet::arrow::async_reader::metadata::MetadataSuffixFetch`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.metadata.MetadataSuffixFetch.json).

<a id="op-02be94b084ca684a67294ee4"></a>
## MetadataSuffixFetch

`trait` · `parquet::arrow::async_reader::metadata::MetadataSuffixFetch` · parquet 59.3.0

```rust
trait MetadataSuffixFetch: MetadataFetch
```

Source: `src/arrow/async_reader/metadata.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A data source that can be used with [`ParquetMetaDataReader`] to load [`ParquetMetaData`] via suffix
requests, without knowing the file size

[`ParquetMetaDataReader`]: crate::file::metadata::reader::ParquetMetaDataReader
[`ParquetMetaData`]: crate::file::metadata::ParquetMetaData

<a id="op-83677b8518053b814677d32d"></a>
## fetch_suffix

`function` · `parquet::arrow::async_reader::metadata::MetadataSuffixFetch::fetch_suffix` · parquet 59.3.0

```rust
fn fetch_suffix(&mut self, suffix: usize) -> BoxFuture<'_, Result<Bytes>>
```

Source: `src/arrow/async_reader/metadata.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a future that fetches the last `n` bytes asynchronously

Note the returned type is a boxed future, often created by
[`FutureExt::boxed`]. See the trait documentation for an example

[`FutureExt::boxed`]: futures::FutureExt::boxed

Unresolved upstream links (retained, not inferred): `futures::FutureExt::boxed`.
