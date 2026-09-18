# `datafusion_datasource_parquet::reader::ParquetFileReaderFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.reader.ParquetFileReaderFactory.json).

<a id="op-864a9023955965d6e936e916"></a>
## ParquetFileReaderFactory

`trait` · `datafusion_datasource_parquet::reader::ParquetFileReaderFactory` · datafusion-datasource-parquet 55.1.0

```rust
trait ParquetFileReaderFactory: Debug + Send + Sync + 'static
```

Source: `src/reader.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Interface for reading Apache Parquet files.

The combined implementations of [`ParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.ParquetFileReaderFactory.md#op-864a9023955965d6e936e916) and
[`AsyncFileReader`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-69c2992a81570b27e07fe176) can be used to provide custom data access operations
such as pre-cached metadata, I/O coalescing, etc.

See [`DefaultParquetFileReaderFactory`](../operations/datafusion_datasource_parquet.reader.DefaultParquetFileReaderFactory.md#op-f7b9d0e1ab2507dcbfb1a612) for a simple implementation.

<a id="op-d79b8381f3038a4dee48f1b0"></a>
## create_reader

`function` · `datafusion_datasource_parquet::reader::ParquetFileReaderFactory::create_reader` · datafusion-datasource-parquet 55.1.0

```rust
fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile, metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet) -> datafusion_common::Result<Box<dyn AsyncFileReader + Send>>
```

Source: `src/reader.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Provides an `AsyncFileReader` for reading data from a parquet file specified

# Notes

If the resulting [`AsyncFileReader`](../operations/parquet.arrow.async_reader.AsyncFileReader.md#op-69c2992a81570b27e07fe176)  returns `ParquetMetaData` without
page index information, the reader will load it on demand. Thus it is important
to ensure that the returned `ParquetMetaData` has the necessary information
if you wish to avoid a subsequent I/O

# Arguments
* partition_index - Index of the partition (for reporting metrics)
* file - The file to be read
* metadata_size_hint - If specified, the first IO reads this many bytes from the footer
* metrics - Execution metrics
