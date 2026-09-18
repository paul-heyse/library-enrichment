# `parquet`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.json).

<a id="op-a2a0c4ff17ff9b4865c11f69"></a>
## parquet

`module` · `parquet` · parquet 59.3.0

```rust
mod parquet
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).


This crate contains the official Native Rust implementation of
[Apache Parquet](https://parquet.apache.org/), part of
the [Apache Arrow](https://arrow.apache.org/) project.
The crate provides a number of APIs to read and write Parquet files,
covering a range of use cases.

Please see the [parquet crates.io](https://crates.io/crates/parquet)
page for feature flags and tips to improve performance.

# Format Overview

Parquet is a columnar format, which means that unlike row formats like [CSV], values are
iterated along columns instead of rows. Parquet is similar in spirit to [Arrow], but
focuses on storage efficiency whereas Arrow prioritizes compute efficiency.

Parquet files are partitioned for scalability. Each file contains metadata,
along with zero or more "row groups", each row group containing one or
more columns. The APIs in this crate reflect this structure.

Data in Parquet files is strongly typed and differentiates between logical
and physical types (see [`schema`](../modules/parquet.schema.md#op-8f68e8d1972333f8e4689b62)). In addition, Parquet files may contain
other metadata, such as statistics, which can be used to optimize reading
(see [`file::metadata`](../modules/parquet.file.metadata.md#op-83908534150f436bfde8c42b)).
For more details about the Parquet format itself, see the [Parquet spec]

[Parquet spec]: https://github.com/apache/parquet-format/blob/master/README.md#file-format

# APIs

This crate exposes a number of APIs for different use-cases.

## Metadata and Schema

The [`schema`](../modules/parquet.schema.md#op-8f68e8d1972333f8e4689b62) module provides APIs to work with Parquet schemas. The
[`file::metadata`](../modules/parquet.file.metadata.md#op-83908534150f436bfde8c42b) module provides APIs to work with Parquet metadata.

## Reading and Writing Arrow (`arrow` feature)

The [`arrow`](../modules/parquet.arrow.md#op-b2d3d71b2a2cc696513bb543) module supports reading and writing Parquet data to/from
Arrow [`RecordBatch`]es. Using Arrow is simple and performant, and allows workloads
to leverage the wide range of data transforms provided by the [arrow] crate, and by the
ecosystem of [Arrow] compatible systems.

Most users will use [`ArrowWriter`] for writing and [`ParquetRecordBatchReaderBuilder`] for
reading from synchronous IO sources such as files or in-memory buffers.

Lower level APIs include
* [`ParquetPushDecoder`] for file grained control over interleaving of IO and CPU.
* [`ArrowColumnWriter`] for writing using multiple threads,
* [`RowFilter`] to apply filters during decode

### EXPERIMENTAL: Content-Defined Chunking

[`ArrowWriter`] supports content-defined chunking (CDC), which creates data page
boundaries based on content rather than fixed sizes. CDC enables efficient
deduplication in content-addressable storage (CAS) systems: when the same data
appears in successive file versions, it will produce identical byte sequences that
CAS backends can deduplicate.

Enable CDC via [`WriterProperties`]:

```rust
# use parquet::file::properties::{WriterProperties, CdcOptions};
let props = WriterProperties::builder()
    .set_content_defined_chunking(Some(CdcOptions::default()))
    .build();
```

See [`CdcOptions`] for chunk size and normalization parameters.

[`WriterProperties`]: file::properties::WriterProperties
[`CdcOptions`]: file::properties::CdcOptions

[`ArrowWriter`]: arrow::arrow_writer::ArrowWriter
[`ParquetRecordBatchReaderBuilder`]: arrow::arrow_reader::ParquetRecordBatchReaderBuilder
[`ParquetPushDecoder`]: arrow::push_decoder::ParquetPushDecoder
[`ArrowColumnWriter`]: arrow::arrow_writer::ArrowColumnWriter
[`RowFilter`]: arrow::arrow_reader::RowFilter

## `async` Reading and Writing Arrow (`arrow` feature + `async` feature)

The [`async_reader`] and [`async_writer`] modules provide async APIs to
read and write [`RecordBatch`]es  asynchronously.

Most users will use [`AsyncArrowWriter`] for writing and [`ParquetRecordBatchStreamBuilder`]
for reading, automatically optimizing IO based on any predicates or projections provided.
Object storage services such as S3 can be integrated by implementing
[`AsyncFileReader`] on top of a client such as the [object_store] crate,
or by passing a writer implementing [`AsyncWrite`] (such as
`object_store::buffered::BufWriter`) to [`AsyncArrowWriter`].

[`async_reader`]: arrow::async_reader
[`async_writer`]: arrow::async_writer
[`AsyncArrowWriter`]: arrow::async_writer::AsyncArrowWriter
[`AsyncFileReader`]: arrow::async_reader::AsyncFileReader
[`AsyncWrite`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWrite.html
[`ParquetRecordBatchStreamBuilder`]: arrow::async_reader::ParquetRecordBatchStreamBuilder

## Variant Logical Type (`variant_experimental` feature)

The [`variant`](../modules/parquet.variant.md#op-661d0febb503222365f1b4d8) module supports reading and writing Parquet files
with the [Variant Binary Encoding] logical type, which can represent
semi-structured data such as JSON efficiently.

[Variant Binary Encoding]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md

## Read/Write Parquet Directly

Workloads needing finer-grained control, or to avoid a dependence on arrow,
can use the APIs in [`mod@file`](../modules/parquet.file.md#op-32c82f677bb3901ca5a9d887) directly. These APIs  are harder to use
as they directly use the underlying Parquet data model, and require knowledge
of the Parquet format, including the details of [Dremel] record shredding
and [Logical Types].

[arrow]: https://docs.rs/arrow/latest/arrow/index.html
[Arrow]: https://arrow.apache.org/
[`RecordBatch`]: https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html
[CSV]: https://en.wikipedia.org/wiki/Comma-separated_values
[Dremel]: https://research.google/pubs/pub36632/
[Logical Types]: https://github.com/apache/parquet-format/blob/master/LogicalTypes.md
[object_store]: https://docs.rs/object_store/latest/object_store/
