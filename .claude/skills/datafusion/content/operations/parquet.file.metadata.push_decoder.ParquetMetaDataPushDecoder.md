# `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.json).

<a id="op-e00d43ab5b4fd446b342b085"></a>
## ParquetMetaDataPushDecoder

`struct` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder` · parquet 59.3.0

```rust
struct ParquetMetaDataPushDecoder
```

Source: `src/file/metadata/push_decoder.rs:222`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A push decoder for [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b).

This structure implements a push API for decoding Parquet metadata, which
decouples IO from the metadata decoding logic (sometimes referred to as
[Sans-IO]).

See [`ParquetMetaDataReader`] for a pull-based API that incorporates IO and
is simpler to use for basic use cases. This decoder is best for customizing
your IO operations to minimize bytes read, prefetch data, or use async IO.

[Sans-IO]: https://sans-io.readthedocs.io
[`ParquetMetaDataReader`]: crate::file::metadata::ParquetMetaDataReader

# Example

The most basic usage is to feed the decoder with the necessary byte ranges
as requested as shown below. This minimizes the number of bytes read, but
requires the most IO operations - one to read the footer and then one
to read the metadata, and possibly more if page indexes are requested.

```rust
# use std::ops::Range;
# use bytes::Bytes;
# use arrow_array::record_batch;
# use parquet::DecodeResult;
# use parquet::arrow::ArrowWriter;
# use parquet::errors::ParquetError;
# use parquet::file::metadata::{ParquetMetaData, ParquetMetaDataPushDecoder};
#
# fn decode_metadata() -> Result<ParquetMetaData, ParquetError> {
# let file_bytes = {
#   let mut buffer = vec![0];
#   let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
#   let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None).unwrap();
#   writer.write(&batch).unwrap();
#   writer.close().unwrap();
#   Bytes::from(buffer)
# };
# // mimic IO by returning a function that returns the bytes for a given range
# let get_range = |range: &Range<u64>| -> Bytes {
#    let start = range.start as usize;
#     let end = range.end as usize;
#    file_bytes.slice(start..end)
# };
#
# let file_len = file_bytes.len() as u64;
// The `ParquetMetaDataPushDecoder` needs to know the file length.
let mut decoder = ParquetMetaDataPushDecoder::try_new(file_len).unwrap();
// try to decode the metadata. If more data is needed, the decoder will tell you what ranges
loop {
    match decoder.try_decode() {
       Ok(DecodeResult::Data(metadata)) => { return Ok(metadata); } // decode successful
       Ok(DecodeResult::NeedsData(ranges)) => {
          // The decoder needs more data
          //
          // In this example, we call a function that returns the bytes for each given range.
          // In a real application, you would likely read the data from a file or network.
          let data = ranges.iter().map(|range| get_range(range)).collect();
          // Push the data into the decoder and try to decode again on the next iteration.
          decoder.push_ranges(ranges, data).unwrap();
       }
       Ok(DecodeResult::Finished) => { unreachable!("returned metadata in previous match arm") }
       Err(e) => return Err(e),
    }
}
# }
```

# Example with "prefetching"

By default, the [`ParquetMetaDataPushDecoder`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085) will request only the exact byte
ranges it needs. This minimizes the number of bytes read, however it
requires at least two IO operations to read the metadata - one to read the
footer and then one to read the metadata.

If the file has a "Page Index" (see [Self::with_page_index_policy](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-48663bd4d902450e3347bcc6)), three
IO operations are required to read the metadata, as the page index is
not part of the normal metadata footer.

To reduce the number of IO operations in systems with high per operation
overhead (e.g. cloud storage), you can "prefetch" the data and then push
the data into the decoder before calling [`Self::try_decode`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-064d44bfc9d1c502efa056cc). If you do
not push enough bytes, the decoder will return the ranges that are still
needed.

This approach can also be used when you have the entire file already in memory
for other reasons.
```rust
# use std::ops::Range;
# use bytes::Bytes;
# use arrow_array::record_batch;
# use parquet::DecodeResult;
# use parquet::arrow::ArrowWriter;
# use parquet::errors::ParquetError;
# use parquet::file::metadata::{ParquetMetaData, ParquetMetaDataPushDecoder};
#
# fn decode_metadata() -> Result<ParquetMetaData, ParquetError> {
# let file_bytes = {
#   let mut buffer = vec![0];
#   let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
#   let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None).unwrap();
#   writer.write(&batch).unwrap();
#   writer.close().unwrap();
#   Bytes::from(buffer)
# };
#
let file_len = file_bytes.len() as u64;
// For this example, we "prefetch" all the bytes which we have in memory,
// but in a real application, you would likely read a chunk from the end
// for example 1MB.
let prefetched_bytes = file_bytes.clone();
let mut decoder = ParquetMetaDataPushDecoder::try_new(file_len).unwrap();
// push the prefetched bytes into the decoder
decoder.push_ranges(vec![0..file_len], vec![prefetched_bytes]).unwrap();
// The decoder will now be able to decode the metadata. Note in a real application,
// unless you can guarantee that the pushed data is enough to decode the metadata,
// you still need to call `try_decode` in a loop until it returns `DecodeResult::Data`
// as shown in  the previous example
    match decoder.try_decode() {
        Ok(DecodeResult::Data(metadata)) => { return Ok(metadata); } // decode successful
        other => { panic!("expected DecodeResult::Data, got: {other:?}") }
    }
# }
```

# Example using [`AsyncRead`]

[`ParquetMetaDataPushDecoder`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085) is designed to work with any data source that can
provide byte ranges, including async IO sources. However, it does not
implement async IO itself. To use async IO, you simply write an async
wrapper around it that reads the required byte ranges and pushes them into the
decoder.
```rust
# use std::ops::Range;
# use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
# use arrow_array::record_batch;
# use parquet::DecodeResult;
# use parquet::arrow::ArrowWriter;
# use parquet::errors::ParquetError;
# use parquet::file::metadata::{ParquetMetaData, ParquetMetaDataPushDecoder};
#
// This function decodes Parquet Metadata from anything that implements
// [`AsyncRead`] and [`AsyncSeek`] such as a tokio::fs::File
async fn decode_metadata(
  file_len: u64,
  mut async_source: impl AsyncRead + AsyncSeek + Unpin
) -> Result<ParquetMetaData, ParquetError> {
  // We need a ParquetMetaDataPushDecoder to decode the metadata.
  let mut decoder = ParquetMetaDataPushDecoder::try_new(file_len).unwrap();
  loop {
    match decoder.try_decode() {
       Ok(DecodeResult::Data(metadata)) => { return Ok(metadata); } // decode successful
       Ok(DecodeResult::NeedsData(ranges)) => {
          // The decoder needs more data
          //
          // In this example we use the AsyncRead and AsyncSeek traits to read the
          // required ranges from the async source.
          let mut data = Vec::with_capacity(ranges.len());
          for range in &ranges {
            let mut buffer = vec![0; (range.end - range.start) as usize];
            async_source.seek(std::io::SeekFrom::Start(range.start)).await?;
            async_source.read_exact(&mut buffer).await?;
            data.push(Bytes::from(buffer));
          }
          // Push the data into the decoder and try to decode again on the next iteration.
          decoder.push_ranges(ranges, data).unwrap();
       }
       Ok(DecodeResult::Finished) => { unreachable!("returned metadata in previous match arm") }
       Err(e) => return Err(e),
    }
  }
}
```
[`AsyncRead`]: tokio::io::AsyncRead

Unresolved upstream links (retained, not inferred): `tokio::io::AsyncRead`.

<a id="op-d7d46ed28bdb3b19035497f2"></a>
## clear_all_ranges

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::clear_all_ranges` · parquet 59.3.0

```rust
fn clear_all_ranges(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:362`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Clear any staged byte ranges currently buffered for future decode work.

<a id="op-1df83a4f7ded1695a0fa3d4f"></a>
## fmt

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 10], "end": [221, 15], "filename": "src/file/metadata/push_decoder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/metadata/push_decoder.rs:221`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a11b3757e2438804aa2ed849"></a>
## push_range

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::push_range` · parquet 59.3.0

```rust
fn push_range(&mut self, range: Range<u64>, buffer: Bytes) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:351`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Pushes a single range of data into the decoder's buffer.

<a id="op-23cc78012283dc140264555d"></a>
## push_ranges

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::push_ranges` · parquet 59.3.0

```rust
fn push_ranges(&mut self, ranges: Vec<Range<u64>>, buffers: Vec<Bytes>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:340`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Push the data into the decoder's buffer.

The decoder does not immediately attempt to decode the metadata
after pushing data. Instead, it accumulates the pushed data until you
call [`Self::try_decode`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-064d44bfc9d1c502efa056cc).

# Determining required data:

To determine what ranges are required to decode the metadata, you can
either:

1. Call [`Self::try_decode`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-064d44bfc9d1c502efa056cc) first to get the exact ranges required (see
   example on [`Self`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085))

2. Speculatively push any data that you have available, which may
   include more than the footer data or requested bytes.

Speculatively pushing data can be used when  "prefetching" data. See
example on [`Self`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085)

<a id="op-064d44bfc9d1c502efa056cc"></a>
## try_decode

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::try_decode` · parquet 59.3.0

```rust
fn try_decode(&mut self) -> Result<DecodeResult<ParquetMetaData>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:368`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Try to decode the metadata from the pushed data, returning the
decoded metadata or an error if not enough data is available.

<a id="op-7c9e560fe355239993cc9f5c"></a>
## try_new

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::try_new` · parquet 59.3.0

```rust
fn try_new(file_len: u64) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:242`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new `ParquetMetaDataPushDecoder` with the given file length.

By default, this will read page indexes and column indexes. See
[`ParquetMetaDataPushDecoder::with_page_index_policy`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-48663bd4d902450e3347bcc6) for more detail.

See examples on [`ParquetMetaDataPushDecoder`](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md#op-e00d43ab5b4fd446b342b085).

<a id="op-282e6cb2539655c9b3da157c"></a>
## try_new_with_metadata

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::try_new_with_metadata` · parquet 59.3.0

```rust
fn try_new_with_metadata(file_len: u64, metadata: ParquetMetaData) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:269`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a decoder with the given `ParquetMetaData` already known.

This can be used to parse and populate the page index structures
after the metadata has already been decoded.

<a id="op-0cac64151977b37124ce0a8e"></a>
## with_column_index_policy

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::with_column_index_policy` · parquet 59.3.0

```rust
fn with_column_index_policy(self, column_index_policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:292`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the policy for reading the ColumnIndex (part of the PageIndex)

<a id="op-79c7c8c84a228f52286631dc"></a>
## with_file_decryption_properties

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::with_file_decryption_properties` · parquet 59.3.0

```rust
fn with_file_decryption_properties(self, file_decryption_properties: Option<std::sync::Arc<FileDecryptionProperties>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:311`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Provide decryption properties for decoding encrypted Parquet files

<a id="op-91b28bac9f0fda3e26a2cd65"></a>
## with_metadata_options

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::with_metadata_options` · parquet 59.3.0

```rust
fn with_metadata_options(self, options: Option<Arc<ParquetMetaDataOptions>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:304`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the options to use when decoding the Parquet metadata.

<a id="op-e0ad5385972e8a106701a700"></a>
## with_offset_index_policy

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::with_offset_index_policy` · parquet 59.3.0

```rust
fn with_offset_index_policy(self, offset_index_policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:298`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the policy for reading the OffsetIndex (part of the PageIndex)

<a id="op-48663bd4d902450e3347bcc6"></a>
## with_page_index_policy

`function` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder::with_page_index_policy` · parquet 59.3.0

```rust
fn with_page_index_policy(self, page_index_policy: PageIndexPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder", "path": "ParquetMetaDataPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [458, 2], "filename": "src/file/metadata/push_decoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/metadata/push_decoder.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Enable or disable reading the page index structures described in
"[Parquet page index] Layout to Support Page Skipping".

Defaults to [`PageIndexPolicy::Optional`](../operations/parquet.file.metadata.reader.PageIndexPolicy.md#op-048be08cc587a4d901ecc115)

This requires
1. The Parquet file to have been written with page indexes
2. Additional data to be pushed into the decoder (as the page indexes are not part of the thrift footer)

[Parquet page index]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
