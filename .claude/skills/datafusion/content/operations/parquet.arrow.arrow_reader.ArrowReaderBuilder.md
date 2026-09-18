# `parquet::arrow::arrow_reader::ArrowReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.ArrowReaderBuilder.json).

<a id="op-2cb4803ec228a018e60491ee"></a>
## ArrowReaderBuilder

`struct` · `parquet::arrow::arrow_reader::ArrowReaderBuilder` · parquet 59.3.0

```rust
struct ArrowReaderBuilder<T>
```

Source: `src/arrow/arrow_reader/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Builder for constructing Parquet readers that decode into [Apache Arrow]
arrays.

Most users should use one of the following specializations:

* synchronous API: [`ParquetRecordBatchReaderBuilder`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md#op-1638827d919dceda05d4f2dd)
* `async` API: [`ParquetRecordBatchStreamBuilder`]
* decoder API: [`ParquetPushDecoderBuilder`]

# Features
* Projection pushdown: [`Self::with_projection`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-5bbdcadc7d33ef95704f8b2d)
* Cached metadata: [`ArrowReaderMetadata::load`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-af27598b22047b6ad64b6068)
* Offset skipping: [`Self::with_offset`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-782bde4062d02d2d645c13f6) and [`Self::with_limit`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-0b821f958a1e445121d8eac5)
* Row group filtering: [`Self::with_row_groups`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a242cc258e7519b5c6886ed9)
* Range filtering: [`Self::with_row_selection`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-521e662260c093bdc9d5c113)
* Row level filtering: [`Self::with_row_filter`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-8bea3b5ddc8fa0db0c89f3ec)

# Implementing Predicate Pushdown

[`Self::with_row_filter`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-8bea3b5ddc8fa0db0c89f3ec) permits filter evaluation *during* the decoding
process, which is efficient and allows the most low level optimizations.

However, most Parquet based systems will apply filters at many steps prior
to decoding such as pruning files, row groups and data pages. This crate
provides the low level APIs needed to implement such filtering, but does not
include any logic to actually evaluate predicates. For example:

* [`Self::with_row_groups`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a242cc258e7519b5c6886ed9) for Row Group pruning
* [`Self::with_row_selection`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-521e662260c093bdc9d5c113) for data page pruning
* [`StatisticsConverter`] to convert Parquet statistics to Arrow arrays

The rationale for this design is that implementing predicate pushdown is a
complex topic and varies significantly from system to system. For example

1. Predicates supported (do you support predicates like prefix matching, user defined functions, etc)
2. Evaluating predicates on multiple files (with potentially different but compatible schemas)
3. Evaluating predicates using information from an external metadata catalog (e.g. Apache Iceberg or similar)
4. Interleaving fetching metadata, evaluating predicates, and decoding files

You can read more about this design in the [Querying Parquet with
Millisecond Latency] Arrow blog post.

[`ParquetRecordBatchStreamBuilder`]: crate::arrow::async_reader::ParquetRecordBatchStreamBuilder
[`ParquetPushDecoderBuilder`]: crate::arrow::push_decoder::ParquetPushDecoderBuilder
[Apache Arrow]: https://arrow.apache.org/
[`StatisticsConverter`]: statistics::StatisticsConverter
[Querying Parquet with Millisecond Latency]: https://arrow.apache.org/blog/2022/12/26/querying-parquet-with-millisecond-latency/

<a id="op-026643d1cd87940830282c85"></a>
## build

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<ParquetPushDecoder, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [310, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:253`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ParquetPushDecoder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-4c043486cf5c22ecde7f148a) with the configured options

<a id="op-2f9b94ed70e7f0f64445661e"></a>
## build

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<ParquetRecordBatchStream<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "AsyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [702, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:650`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Build a new [`ParquetRecordBatchStream`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-5aaf529c72b40fe254f58220)

See examples on [`ParquetRecordBatchStreamBuilder::new`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a1314ec698521a426652f8d3)

<a id="op-81f883140218260103436973"></a>
## build

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> Result<ParquetRecordBatchReader>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "SyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1292, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1224`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Build a [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3)

Note: this will eagerly evaluate any `RowFilter` before returning

<a id="op-5f1a0c06b3631139e7d596a6"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [170, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/mod.rs:153`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aa1f4ff93594bc163536d27"></a>
## get_row_group_column_bloom_filter

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::get_row_group_column_bloom_filter` · parquet 59.3.0

```rust
async fn get_row_group_column_bloom_filter(&mut self, row_group_idx: usize, column_idx: usize) -> Result<Option<Sbbf>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "AsyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [702, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:585`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read bloom filter for a column in a row group

Returns `None` if the column does not have a bloom filter

We should call this function after other forms pruning, such as projection and predicate pushdown.

<a id="op-8695eaac95a2c9ba4c1f958e"></a>
## get_row_group_column_bloom_filter

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::get_row_group_column_bloom_filter` · parquet 59.3.0

```rust
fn get_row_group_column_bloom_filter(&self, row_group_idx: usize, column_idx: usize) -> Result<Option<Sbbf>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "SyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1292, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1166`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read bloom filter for a column in a row group

Returns `None` if the column does not have a bloom filter

We should call this function after other forms pruning, such as projection and predicate pushdown.

<a id="op-8ff55f79679ef0bb8c26fbf7"></a>
## metadata

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::metadata` · parquet 59.3.0

```rust
fn metadata(&self) -> &Arc<ParquetMetaData>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:193`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) for this parquet file

<a id="op-a1314ec698521a426652f8d3"></a>
## new

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::new` · parquet 59.3.0

```rust
async fn new(input: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "AsyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [702, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:520`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetRecordBatchStreamBuilder`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.md#op-36119f71505f76bb82a22932) for reading from the
specified source.

# Examples:
* [Basic example reading from an async source](#example)
* [Configuring options and reading metadata](#example-configuring-options-and-reading-metadata)
* [Reading Row Groups in Parallel](#example-reading-row-groups-in-parallel)

# Example
```
# #[tokio::main(flavor="current_thread")]
# async fn main() {
#
# use arrow_array::RecordBatch;
# use arrow::util::pretty::pretty_format_batches;
# use futures::TryStreamExt;
#
# use parquet::arrow::{ParquetRecordBatchStreamBuilder, ProjectionMask};
#
# fn assert_batches_eq(batches: &[RecordBatch], expected_lines: &[&str]) {
#     let formatted = pretty_format_batches(batches).unwrap().to_string();
#     let actual_lines: Vec<_> = formatted.trim().lines().collect();
#     assert_eq!(
#          &actual_lines, expected_lines,
#          "\n\nexpected:\n\n{:#?}\nactual:\n\n{:#?}\n\n",
#          expected_lines, actual_lines
#      );
#  }
#
# let testdata = arrow::util::test_util::parquet_test_data();
# let path = format!("{}/alltypes_plain.parquet", testdata);
// Use tokio::fs::File to read data using an async I/O. This can be replaced with
// another async I/O reader such as a reader from an object store.
let file = tokio::fs::File::open(path).await.unwrap();

// Configure options for reading from the async source
let builder = ParquetRecordBatchStreamBuilder::new(file)
    .await
    .unwrap();
// Building the stream opens the parquet file (reads metadata, etc) and returns
// a stream that can be used to incrementally read the data in batches
let stream = builder.build().unwrap();
// In this example, we collect the stream into a Vec<RecordBatch>
// but real applications would likely process the batches as they are read
let results = stream.try_collect::<Vec<_>>().await.unwrap();
// Demonstrate the results are as expected
assert_batches_eq(
    &results,
    &[
      "+----+----------+-------------+--------------+---------+------------+-----------+------------+------------------+------------+---------------------+",
      "| id | bool_col | tinyint_col | smallint_col | int_col | bigint_col | float_col | double_col | date_string_col  | string_col | timestamp_col       |",
      "+----+----------+-------------+--------------+---------+------------+-----------+------------+------------------+------------+---------------------+",
      "| 4  | true     | 0           | 0            | 0       | 0          | 0.0       | 0.0        | 30332f30312f3039 | 30         | 2009-03-01T00:00:00 |",
      "| 5  | false    | 1           | 1            | 1       | 10         | 1.1       | 10.1       | 30332f30312f3039 | 31         | 2009-03-01T00:01:00 |",
      "| 6  | true     | 0           | 0            | 0       | 0          | 0.0       | 0.0        | 30342f30312f3039 | 30         | 2009-04-01T00:00:00 |",
      "| 7  | false    | 1           | 1            | 1       | 10         | 1.1       | 10.1       | 30342f30312f3039 | 31         | 2009-04-01T00:01:00 |",
      "| 2  | true     | 0           | 0            | 0       | 0          | 0.0       | 0.0        | 30322f30312f3039 | 30         | 2009-02-01T00:00:00 |",
      "| 3  | false    | 1           | 1            | 1       | 10         | 1.1       | 10.1       | 30322f30312f3039 | 31         | 2009-02-01T00:01:00 |",
      "| 0  | true     | 0           | 0            | 0       | 0          | 0.0       | 0.0        | 30312f30312f3039 | 30         | 2009-01-01T00:00:00 |",
      "| 1  | false    | 1           | 1            | 1       | 10         | 1.1       | 10.1       | 30312f30312f3039 | 31         | 2009-01-01T00:01:00 |",
      "+----+----------+-------------+--------------+---------+------------+-----------+------------+------------------+------------+---------------------+",
     ],
 );
# }
```

# Example Configuring Options and Reading Metadata

There are many options that control the behavior of the reader, such as
`with_batch_size`, `with_projection`, `with_filter`, etc...

```
# #[tokio::main(flavor="current_thread")]
# async fn main() {
#
# use arrow_array::RecordBatch;
# use arrow::util::pretty::pretty_format_batches;
# use futures::TryStreamExt;
#
# use parquet::arrow::{ParquetRecordBatchStreamBuilder, ProjectionMask};
#
# fn assert_batches_eq(batches: &[RecordBatch], expected_lines: &[&str]) {
#     let formatted = pretty_format_batches(batches).unwrap().to_string();
#     let actual_lines: Vec<_> = formatted.trim().lines().collect();
#     assert_eq!(
#          &actual_lines, expected_lines,
#          "\n\nexpected:\n\n{:#?}\nactual:\n\n{:#?}\n\n",
#          expected_lines, actual_lines
#      );
#  }
#
# let testdata = arrow::util::test_util::parquet_test_data();
# let path = format!("{}/alltypes_plain.parquet", testdata);
// As before, use tokio::fs::File to read data using an async I/O.
let file = tokio::fs::File::open(path).await.unwrap();

// Configure options for reading from the async source, in this case we set the batch size
// to 3 which produces 3 rows at a time.
let builder = ParquetRecordBatchStreamBuilder::new(file)
    .await
    .unwrap()
    .with_batch_size(3);

// We can also read the metadata to inspect the schema and other metadata
// before actually reading the data
let file_metadata = builder.metadata().file_metadata();
// Specify that we only want to read the 1st, 2nd, and 6th columns
let mask = ProjectionMask::roots(file_metadata.schema_descr(), [1, 2, 6]);

let stream = builder.with_projection(mask).build().unwrap();
let results = stream.try_collect::<Vec<_>>().await.unwrap();
// Print out the results
assert_batches_eq(
    &results,
    &[
        "+----------+-------------+-----------+",
        "| bool_col | tinyint_col | float_col |",
        "+----------+-------------+-----------+",
        "| true     | 0           | 0.0       |",
        "| false    | 1           | 1.1       |",
        "| true     | 0           | 0.0       |",
        "| false    | 1           | 1.1       |",
        "| true     | 0           | 0.0       |",
        "| false    | 1           | 1.1       |",
        "| true     | 0           | 0.0       |",
        "| false    | 1           | 1.1       |",
        "+----------+-------------+-----------+",
     ],
 );

// The results has 8 rows, so since we set the batch size to 3, we expect
// 3 batches, two with 3 rows each and the last batch with 2 rows.
assert_eq!(results.len(), 3);
# }
```

# Example reading Row Groups in Parallel

Each [`ParquetRecordBatchStream`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-5aaf529c72b40fe254f58220) is independent and can be used to read
from the same underlying source in parallel. Use
[`ParquetRecordBatchStream::next_row_group`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-6a7b74ccf9b4191804415952) with a single stream to
begin prefetching the next Row Group. To read a file in parallel, create
a stream for each subset of the file. For example, you can read each
row group in parallel by creating a stream for each row group using the
[`ParquetRecordBatchStreamBuilder::with_row_groups`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a242cc258e7519b5c6886ed9) API as shown below

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch};
# use arrow::util::pretty::pretty_format_batches;
# use futures::{StreamExt, TryStreamExt};
# use tempfile::NamedTempFile;
# use parquet::arrow::{ArrowWriter, ParquetRecordBatchStreamBuilder, ProjectionMask};
# use parquet::arrow::arrow_reader::{ArrowReaderMetadata, ArrowReaderOptions};
# use parquet::file::metadata::ParquetMetaDataReader;
# use parquet::file::properties::{WriterProperties};
# // write to a temporary file with 10 RowGroups and read back with async API
# fn write_file() -> parquet::errors::Result<NamedTempFile> {
#   let mut file = NamedTempFile::new().unwrap();
#   let small_batch = RecordBatch::try_from_iter([
#      ("id", Arc::new(Int32Array::from(vec![0, 1, 2, 3, 4])) as ArrayRef),
#   ]).unwrap();
#   let props = WriterProperties::builder()
#     .set_max_row_group_row_count(Some(5))
#     .set_write_batch_size(5)
#     .build();
#   let mut writer = ArrowWriter::try_new(&mut file, small_batch.schema(), Some(props))?;
#   for i in 0..10 {
#     writer.write(&small_batch)?
#   };
#   writer.close()?;
#   Ok(file)
# }
# #[tokio::main(flavor="current_thread")]
# async fn main() -> parquet::errors::Result<()> {
# let t = write_file()?;
# let path = t.path();
// This example uses a tokio::fs::File as the async source, but it
// could be any async source such as an object store reader)
let mut file = tokio::fs::File::open(path).await?;
// To read Row Groups in parallel, create a separate stream builder for each Row Group.
// First get the metadata to find the row group information
let file_size = file.metadata().await?.len();
let metadata = ParquetMetaDataReader::new().load_and_finish(&mut file, file_size).await?;
assert_eq!(metadata.num_row_groups(), 10); // file has 10 row groups with 5 rows each
// Create a stream reader for each row group
let reader_metadata = ArrowReaderMetadata::try_new(
  Arc::new(metadata),
  ArrowReaderOptions::new()
)?;
let mut streams = vec![];
 for row_group_index in 0..10 {
  // Each stream needs its own source instance to issue
  // parallel IO requests, so clone the file for each stream
  let this_file = file.try_clone().await?;
  let stream = ParquetRecordBatchStreamBuilder::new_with_metadata(
       this_file,
       reader_metadata.clone()
     )
     .with_row_groups(vec![row_group_index]) // read only this row group
     .build()?;
    streams.push(stream);
}
// Each reader can now be polled independently and in parallel, for
// example using StreamExt::buffered to read from 3 at a time
let results = futures::stream::iter(streams)
 .map(|stream| async move { stream })
 .buffered(3)
 .flatten()
 .try_collect::<Vec<_>>().await?;
// read all 50 rows (10 row groups x 5 rows per group)
assert_eq!(50, results.iter().map(|s| s.num_rows()).sum::<usize>());
# Ok(())
# }
```

<a id="op-59906ce44f5eaa0ba521c48e"></a>
## new_with_metadata

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::new_with_metadata` · parquet 59.3.0

```rust
fn new_with_metadata(input: T, metadata: ArrowReaderMetadata) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "AsyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [702, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:576`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ParquetRecordBatchStreamBuilder`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.md#op-36119f71505f76bb82a22932) from the provided [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17)

This allows loading metadata once and using it to create multiple builders with
potentially different settings, that can be read in parallel.

# Example of reading from multiple streams in parallel

```
# use std::fs::metadata;
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::arrow_reader::ArrowReaderMetadata;
# use parquet::arrow::{ArrowWriter, ParquetRecordBatchStreamBuilder};
# use tempfile::tempfile;
# use futures::StreamExt;
# #[tokio::main(flavor="current_thread")]
# async fn main() {
#
# let mut file = tempfile().unwrap();
# let schema = Arc::new(Schema::new(vec![Field::new("i32", DataType::Int32, false)]));
# let mut writer = ArrowWriter::try_new(&mut file, schema.clone(), None).unwrap();
# let batch = RecordBatch::try_new(schema, vec![Arc::new(Int32Array::from(vec![1, 2, 3]))]).unwrap();
# writer.write(&batch).unwrap();
# writer.close().unwrap();
// open file with parquet data
let mut file = tokio::fs::File::from_std(file);
// load metadata once
let meta = ArrowReaderMetadata::load_async(&mut file, Default::default()).await.unwrap();
// create two readers, a and b, from the same underlying file
// without reading the metadata again
let mut a = ParquetRecordBatchStreamBuilder::new_with_metadata(
    file.try_clone().await.unwrap(),
    meta.clone()
).build().unwrap();
let mut b = ParquetRecordBatchStreamBuilder::new_with_metadata(file, meta).build().unwrap();

// Can read batches from both readers in parallel
assert_eq!(
  a.next().await.unwrap().unwrap(),
  b.next().await.unwrap().unwrap(),
);
# }
```

<a id="op-c2a20ea33d287eead67a9673"></a>
## new_with_metadata

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::new_with_metadata` · parquet 59.3.0

```rust
fn new_with_metadata(arrow_reader_metadata: ArrowReaderMetadata) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [310, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:239`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `ParquetDecoderBuilder` given [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17).

See [`ArrowReaderMetadata::try_new`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-885196b08a0102c7b2b48506) for how to create the metadata from
the Parquet metadata and reader options.

<a id="op-d99349eb3adc8af58b40d077"></a>
## new_with_metadata

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::new_with_metadata` · parquet 59.3.0

```rust
fn new_with_metadata(input: T, metadata: ArrowReaderMetadata) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "SyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1292, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1157`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ParquetRecordBatchReaderBuilder`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md#op-1638827d919dceda05d4f2dd) from the provided [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17)

Use this method if you already have [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) for a file.
This interface allows:

1. Loading metadata once and using it to create multiple builders with
   potentially different settings or run on different threads

2. Using a cached copy of the metadata rather than re-reading it from the
   file each time a reader is constructed.

See the docs on [`ArrowReaderMetadata`](../operations/parquet.arrow.arrow_reader.ArrowReaderMetadata.md#op-c28319e4a1a2024ec909ac17) for more details

# Example
```
# use std::fs::metadata;
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::arrow_reader::{ArrowReaderMetadata, ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};
# use parquet::arrow::ArrowWriter;
#
# let mut file: Vec<u8> = Vec::with_capacity(1024);
# let schema = Arc::new(Schema::new(vec![Field::new("i32", DataType::Int32, false)]));
# let mut writer = ArrowWriter::try_new(&mut file, schema.clone(), None).unwrap();
# let batch = RecordBatch::try_new(schema, vec![Arc::new(Int32Array::from(vec![1, 2, 3]))]).unwrap();
# writer.write(&batch).unwrap();
# writer.close().unwrap();
# let file = Bytes::from(file);
#
let metadata = ArrowReaderMetadata::load(&file, Default::default()).unwrap();
let mut a = ParquetRecordBatchReaderBuilder::new_with_metadata(file.clone(), metadata.clone()).build().unwrap();
let mut b = ParquetRecordBatchReaderBuilder::new_with_metadata(file, metadata).build().unwrap();

// Should be able to read from both in parallel
assert_eq!(a.next().unwrap().unwrap(), b.next().unwrap().unwrap());
```

<a id="op-91d756a485ec20ec374490b6"></a>
## new_with_options

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::new_with_options` · parquet 59.3.0

```rust
async fn new_with_options(input: T, options: ArrowReaderOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "AsyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [702, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:526`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetRecordBatchStreamBuilder`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.md#op-36119f71505f76bb82a22932) with the provided async source
and [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4).

<a id="op-a307ef2af1043b4c2cabbde8"></a>
## parquet_schema

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::parquet_schema` · parquet 59.3.0

```rust
fn parquet_schema(&self) -> &SchemaDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the parquet [`SchemaDescriptor`](../operations/parquet.schema.types.SchemaDescriptor.md#op-cb960d451851ace5640135f7) for this parquet file

<a id="op-c408c3639fc9a3227bf40394"></a>
## schema

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::schema` · parquet 59.3.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:203`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the arrow [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) for this parquet file

<a id="op-f9cc54e8e1dc746c9785bdfe"></a>
## try_new

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::try_new` · parquet 59.3.0

```rust
fn try_new(reader: T) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "SyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1292, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1106`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetRecordBatchReaderBuilder`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md#op-1638827d919dceda05d4f2dd)

```
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};
# use parquet::arrow::ArrowWriter;
# let mut file: Vec<u8> = Vec::with_capacity(1024);
# let schema = Arc::new(Schema::new(vec![Field::new("i32", DataType::Int32, false)]));
# let mut writer = ArrowWriter::try_new(&mut file, schema.clone(), None).unwrap();
# let batch = RecordBatch::try_new(schema, vec![Arc::new(Int32Array::from(vec![1, 2, 3]))]).unwrap();
# writer.write(&batch).unwrap();
# writer.close().unwrap();
# let file = Bytes::from(file);
// Build the reader from anything that implements `ChunkReader`
// such as a `File`, or `Bytes`
let mut builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
// The builder has access to ParquetMetaData such
// as the number and layout of row groups
assert_eq!(builder.metadata().num_row_groups(), 1);
// Call build to create the reader
let mut reader: ParquetRecordBatchReader = builder.build().unwrap();
// Read data
while let Some(batch) = reader.next().transpose()? {
    println!("Read {} rows", batch.num_rows());
}
# Ok::<(), parquet::errors::ParquetError>(())
```

<a id="op-9e58acd79ec0c339224c26c5"></a>
## try_new_decoder

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::try_new_decoder` · parquet 59.3.0

```rust
fn try_new_decoder(parquet_metadata: Arc<ParquetMetaData>) -> Result<Self, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [310, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:217`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `ParquetDecoderBuilder` for configuring a Parquet decoder for the given file.

See [`ParquetMetadataDecoder`] for a builder that can read the metadata from a Parquet file.

[`ParquetMetadataDecoder`]: crate::file::metadata::ParquetMetaDataPushDecoder

See example on [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08)

<a id="op-f3d0c97d9eb290d1ebb16c60"></a>
## try_new_decoder_with_options

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::try_new_decoder_with_options` · parquet 59.3.0

```rust
fn try_new_decoder_with_options(parquet_metadata: Arc<ParquetMetaData>, arrow_reader_options: ArrowReaderOptions) -> Result<Self, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [310, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:226`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `ParquetDecoderBuilder` for configuring a Parquet decoder for the given file
with the given reader options.

This is similar to [`Self::try_new_decoder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-9e58acd79ec0c339224c26c5) but allows configuring
options such as Arrow schema

<a id="op-61bc90b220729954175c1066"></a>
## try_new_with_options

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::try_new_with_options` · parquet 59.3.0

```rust
fn try_new_with_options(reader: T, options: ArrowReaderOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "unresolved", "path": "SyncReader"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::file::reader::ChunkReader", "path": "ChunkReader"}}}, {"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1292, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:1114`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ParquetRecordBatchReaderBuilder`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReaderBuilder.md#op-1638827d919dceda05d4f2dd) with [`ArrowReaderOptions`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-d1d83f9f3dc4572b085ef8a4)

Use this method if you want to control the options for reading the
[`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b)

<a id="op-63bb8da413e75548489f7d3a"></a>
## with_batch_size

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_batch_size` · parquet 59.3.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:213`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the size of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) to produce. Defaults to [`DEFAULT_BATCH_SIZE`](../operations/parquet.arrow.arrow_reader.DEFAULT_BATCH_SIZE.md#op-60f9465f51232ce044a698df).

This may be used as a hint for internal allocations, but does not
guarantee exact internal buffer capacities.

If `batch_size` is more than the file row count, use the file row count.

<a id="op-d418d65b26297e8565f36c66"></a>
## with_buffers

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_buffers` · parquet 59.3.0

```rust
fn with_buffers(self, buffers: PushBuffers) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::PushDecoderInput", "path": "PushDecoderInput"}}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [310, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:245`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a preexisting [`PushBuffers`] for the built decoder to read
from, so bytes already fetched are not requested again.

Unresolved upstream links (retained, not inferred): ``PushBuffers``.

<a id="op-0b821f958a1e445121d8eac5"></a>
## with_limit

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_limit` · parquet 59.3.0

```rust
fn with_limit(self, limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:366`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a limit to the number of rows to be read

The limit will be applied after any [`Self::with_row_selection`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-521e662260c093bdc9d5c113) and [`Self::with_row_filter`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-8bea3b5ddc8fa0db0c89f3ec)
allowing it to limit the final set of rows decoded after any pushed down predicates

It is recommended to enable reading the page index if using this functionality, to allow
more efficient skipping over data pages. See [`ArrowReaderOptions::with_page_index`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25)

<a id="op-496a367160b66b5da37fafbc"></a>
## with_max_predicate_cache_size

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_max_predicate_cache_size` · parquet 59.3.0

```rust
fn with_max_predicate_cache_size(self, max_predicate_cache_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:439`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the maximum size (per row group) of the predicate cache in bytes for
the async decoder.

Defaults to 100MB (across all columns). Set to `usize::MAX` to use
unlimited cache size.

This cache is used to store decoded arrays that are used in
predicate evaluation ([`Self::with_row_filter`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-8bea3b5ddc8fa0db0c89f3ec)).

This cache is only used for the "async" decoder, [`ParquetRecordBatchStream`]. See
[this ticket] for more details and alternatives.

[`ParquetRecordBatchStream`]: https://docs.rs/parquet/latest/parquet/arrow/async_reader/struct.ParquetRecordBatchStream.html
[this ticket]: https://github.com/apache/arrow-rs/issues/8000

<a id="op-b5e5ab0625e34d213a5f04cf"></a>
## with_metrics

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_metrics` · parquet 59.3.0

```rust
fn with_metrics(self, metrics: ArrowReaderMetrics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:421`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Specify metrics collection during reading

To access the metrics, create an [`ArrowReaderMetrics`](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.md#op-c2424d6c9242ce925e8a9bf7) and pass a
clone of the provided metrics to the builder.

For example:

```rust
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};
use parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics;
# use parquet::arrow::ArrowWriter;
# let mut file: Vec<u8> = Vec::with_capacity(1024);
# let schema = Arc::new(Schema::new(vec![Field::new("i32", DataType::Int32, false)]));
# let mut writer = ArrowWriter::try_new(&mut file, schema.clone(), None).unwrap();
# let batch = RecordBatch::try_new(schema, vec![Arc::new(Int32Array::from(vec![1, 2, 3]))]).unwrap();
# writer.write(&batch).unwrap();
# writer.close().unwrap();
# let file = Bytes::from(file);
// Create metrics object to pass into the reader
let metrics = ArrowReaderMetrics::enabled();
let reader = ParquetRecordBatchReaderBuilder::try_new(file).unwrap()
  // Configure the builder to use the metrics by passing a clone
  .with_metrics(metrics.clone())
  // Build the reader
  .build().unwrap();
// .. read data from the reader ..

// check the metrics
assert!(metrics.records_read_from_inner().is_some());
```

<a id="op-782bde4062d02d2d645c13f6"></a>
## with_offset

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_offset` · parquet 59.3.0

```rust
fn with_offset(self, offset: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:380`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide an offset to skip over the given number of rows

The offset will be applied after any [`Self::with_row_selection`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-521e662260c093bdc9d5c113) and [`Self::with_row_filter`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-8bea3b5ddc8fa0db0c89f3ec)
allowing it to skip rows after any pushed down predicates

It is recommended to enable reading the page index if using this functionality, to allow
more efficient skipping over data pages. See [`ArrowReaderOptions::with_page_index`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25)

<a id="op-5bbdcadc7d33ef95704f8b2d"></a>
## with_projection

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_projection` · parquet 59.3.0

```rust
fn with_projection(self, mask: ProjectionMask) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Only read data from the provided column indexes

<a id="op-8bea3b5ddc8fa0db0c89f3ec"></a>
## with_row_filter

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_filter` · parquet 59.3.0

```rust
fn with_row_filter(self, filter: RowFilter) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:352`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a [`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) to skip decoding rows

Row filters are applied after row group selection and row selection

It is recommended to enable reading the page index if using this functionality, to allow
more efficient skipping over data pages. See [`ArrowReaderOptions::with_page_index`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25).

See the [blog post on late materialization] for a more technical explanation.

[blog post on late materialization]: https://arrow.apache.org/blog/2025/12/11/parquet-late-materialization-deep-dive

# Example
```rust
# use std::fs::File;
# use arrow_array::Int32Array;
# use parquet::arrow::ProjectionMask;
# use parquet::arrow::arrow_reader::{ArrowPredicateFn, ParquetRecordBatchReaderBuilder, RowFilter};
# fn main() -> Result<(), parquet::errors::ParquetError> {
# let testdata = arrow::util::test_util::parquet_test_data();
# let path = format!("{testdata}/alltypes_plain.parquet");
# let file = File::open(&path)?;
let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
let schema_desc = builder.metadata().file_metadata().schema_descr_ptr();
// Create predicate that evaluates `int_col != 1`.
// `int_col` column has index 4 (zero based) in the schema
let projection = ProjectionMask::leaves(&schema_desc, [4]);
// Only the projection columns are passed to the predicate so
// int_col is column 0 in the predicate
let predicate = ArrowPredicateFn::new(projection, |batch| {
    let int_col = batch.column(0);
    arrow::compute::kernels::cmp::neq(int_col, &Int32Array::new_scalar(1))
});
let row_filter = RowFilter::new(vec![Box::new(predicate)]);
// The filter will be invoked during the reading process
let reader = builder.with_row_filter(row_filter).build()?;
# for b in reader { let _ = b?; }
# Ok(())
# }
```

<a id="op-a242cc258e7519b5c6886ed9"></a>
## with_row_groups

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_groups` · parquet 59.3.0

```rust
fn with_row_groups(self, row_groups: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:222`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Only read data from the provided row group indexes

This is also called row group filtering

<a id="op-521e662260c093bdc9d5c113"></a>
## with_row_selection

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_selection` · parquet 59.3.0

```rust
fn with_row_selection(self, selection: RowSelection) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:306`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) to filter out rows, and avoid fetching their
data into memory.

This feature is used to restrict which rows are decoded within row
groups, skipping ranges of rows that are not needed. Such selections
could be determined by evaluating predicates against the parquet page
[`Index`] or some other external information available to a query
engine.

# Notes

Row group filtering (see [`Self::with_row_groups`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-a242cc258e7519b5c6886ed9)) is applied prior to
applying the row selection, and therefore rows from skipped row groups
should not be included in the [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) (see example below)

It is recommended to enable writing the page index if using this
functionality, to allow more efficient skipping over data pages. See
[`ArrowReaderOptions::with_page_index`](../operations/parquet.arrow.arrow_reader.ArrowReaderOptions.md#op-4c872ea4ae6007c2db6ecd25).

# Example

Given a parquet file with 4 row groups, and a row group filter of `[0,
2, 3]`, in order to scan rows 50-100 in row group 2 and rows 200-300 in
row group 3:

```text
  Row Group 0, 1000 rows (selected)
  Row Group 1, 1000 rows (skipped)
  Row Group 2, 1000 rows (selected, but want to only scan rows 50-100)
  Row Group 3, 1000 rows (selected, but want to only scan rows 200-300)
```

You could pass the following [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5):

```text
 Select 1000    (scan all rows in row group 0)
 Skip 50        (skip the first 50 rows in row group 2)
 Select 50      (scan rows 50-100 in row group 2)
 Skip 900       (skip the remaining rows in row group 2)
 Skip 200       (skip the first 200 rows in row group 3)
 Select 100     (scan rows 200-300 in row group 3)
 Skip 700       (skip the remaining rows in row group 3)
```
Note there is no entry for the (entirely) skipped row group 1.

Note you can represent the same selection with fewer entries. Instead of

```text
 Skip 900       (skip the remaining rows in row group 2)
 Skip 200       (skip the first 200 rows in row group 3)
```

you could use

```text
Skip 1100      (skip the remaining 900 rows in row group 2 and the first 200 rows in row group 3)
```

[`Index`]: crate::file::page_index::column_index::ColumnIndexMetaData

<a id="op-48f4a93c44669da8125e4195"></a>
## with_row_selection_policy

`function` · `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_selection_policy` · parquet 59.3.0

```rust
fn with_row_selection_policy(self, policy: RowSelectionPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::ArrowReaderBuilder", "path": "ArrowReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [445, 2], "filename": "src/arrow/arrow_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/mod.rs:240`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Configure how row selections should be materialised during execution

See [`RowSelectionPolicy`](../operations/parquet.arrow.arrow_reader.selection.cursor.RowSelectionPolicy.md#op-8c879ac1c58e2c50914ced8e) for more details
