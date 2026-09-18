# `parquet::file::metadata::push_decoder`

Crate `parquet` · 1 public items · structured records in [`model/parquet.file.metadata.push_decoder.json`](../model/parquet.file.metadata.push_decoder.json)

## ParquetMetaDataPushDecoder

`struct` · `parquet::file::metadata::push_decoder::ParquetMetaDataPushDecoder`

Also reachable as `parquet::file::metadata::ParquetMetaDataPushDecoder`

```rust
struct ParquetMetaDataPushDecoder
```

**Derives**: Debug

**Methods** (11)

```rust
fn clear_all_ranges(&mut self)
fn push_range(&mut self, range: Range<u64>, buffer: Bytes) -> Result<()>
fn push_ranges(&mut self, ranges: Vec<Range<u64>>, buffers: Vec<Bytes>) -> Result<()>
fn try_decode(&mut self) -> Result<DecodeResult<ParquetMetaData>>
fn try_new(file_len: u64) -> Result<Self>
fn try_new_with_metadata(file_len: u64, metadata: ParquetMetaData) -> Result<Self>
fn with_column_index_policy(self, column_index_policy: PageIndexPolicy) -> Self
fn with_file_decryption_properties(self, file_decryption_properties: Option<std::sync::Arc<FileDecryptionProperties>>) -> Self
fn with_metadata_options(self, options: Option<Arc<ParquetMetaDataOptions>>) -> Self
fn with_offset_index_policy(self, offset_index_policy: PageIndexPolicy) -> Self
fn with_page_index_policy(self, page_index_policy: PageIndexPolicy) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.file.metadata.push_decoder.ParquetMetaDataPushDecoder.md).


A push decoder for [`ParquetMetaData`].

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

By default, the [`ParquetMetaDataPushDecoder`] will request only the exact byte
ranges it needs. This minimizes the number of bytes read, however it
requires at least two IO operations to read the metadata - one to read the
footer and then one to read the metadata.

If the file has a "Page Index" (see [Self::with_page_index_policy]), three
IO operations are required to read the metadata, as the page index is
not part of the normal metadata footer.

To reduce the number of IO operations in systems with high per operation
overhead (e.g. cloud storage), you can "prefetch" the data and then push
the data into the decoder before calling [`Self::try_decode`]. If you do
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

[`ParquetMetaDataPushDecoder`] is designed to work with any data source that can
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

---
