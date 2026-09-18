# `parquet::arrow::push_decoder`

Crate `parquet` · 3 public items · structured records in [`model/parquet.arrow.push_decoder.json`](../model/parquet.arrow.push_decoder.json)

## ParquetPushDecoder

`struct` · `parquet::arrow::push_decoder::ParquetPushDecoder`

```rust
struct ParquetPushDecoder
```

**Derives**: Debug

**Methods** (10)

```rust
fn buffered_bytes(&self) -> u64
fn clear_all_ranges(&mut self)
fn into_builder(self) -> Result<ParquetPushDecoderBuilder, ParquetError>
fn is_at_row_group_boundary(&self) -> bool
fn peek_next_row_group(&self) -> Result<Option<usize>, ParquetError>
fn push_range(&mut self, range: Range<u64>, data: Bytes) -> Result<(), ParquetError>
fn push_ranges(&mut self, ranges: Vec<Range<u64>>, data: Vec<Bytes>) -> Result<(), ParquetError>
fn row_groups_remaining(&self) -> usize
fn try_decode(&mut self) -> Result<DecodeResult<RecordBatch>, ParquetError>
fn try_next_reader(&mut self) -> Result<DecodeResult<ParquetRecordBatchReader>, ParquetError>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md).


A push based Parquet Decoder

See [`ParquetPushDecoderBuilder`] for an example of how to build and use the decoder.

[`ParquetPushDecoder`] is a low level API for decoding Parquet data without an
underlying reader for performing IO, and thus offers fine grained control
over how data is fetched and decoded.

When more data is needed to make progress, instead of reading data directly
from a reader, the decoder returns [`DecodeResult`] indicating what ranges
are needed. Once the caller provides the requested ranges via
[`Self::push_ranges`], they try to decode again by calling
[`Self::try_decode`].

The decoder's internal state tracks what has been already decoded and what
is needed next.

---

## PushDecoderInput

`struct` · `parquet::arrow::push_decoder::PushDecoderInput`

```rust
struct PushDecoderInput
```

**Derives**: Debug, Default

[Full member, field, variant and typed contracts](../operations/parquet.arrow.push_decoder.PushDecoderInput.md).


The `input` of a [`ParquetPushDecoderBuilder`].

The shared [`ArrowReaderBuilder`] is generic over an `input`. The sync and
async builders read from a file or async reader; the push decoder has no
reader, so its input is the [`PushBuffers`] that caller-pushed bytes
accumulate in (empty for a fresh builder).

---

## ParquetPushDecoderBuilder

`type_alias` · `parquet::arrow::push_decoder::ParquetPushDecoderBuilder`

```rust
type ParquetPushDecoderBuilder = arrow::arrow_reader::ArrowReaderBuilder<PushDecoderInput>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md).


A builder for [`ParquetPushDecoder`].

To create a new decoder, use [`ParquetPushDecoderBuilder::try_new_decoder`].

You can decode the metadata from a Parquet file using either
[`ParquetMetadataReader`] or [`ParquetMetaDataPushDecoder`].

[`ParquetMetadataReader`]: crate::file::metadata::ParquetMetaDataReader
[`ParquetMetaDataPushDecoder`]: crate::file::metadata::ParquetMetaDataPushDecoder

Note the "input" type is `u64` which represents the length of the Parquet file
being decoded. This is needed to initialize the internal buffers that track
what data has been provided to the decoder.

# Example
```
# use std::ops::Range;
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::record_batch;
# use parquet::DecodeResult;
# use parquet::arrow::push_decoder::ParquetPushDecoderBuilder;
# use parquet::arrow::ArrowWriter;
# use parquet::file::metadata::ParquetMetaDataPushDecoder;
# let file_bytes = {
#   let mut buffer = vec![];
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
# let file_length = file_bytes.len() as u64;
# let mut metadata_decoder = ParquetMetaDataPushDecoder::try_new(file_length).unwrap();
# metadata_decoder.push_ranges(vec![0..file_length], vec![file_bytes.clone()]).unwrap();
# let DecodeResult::Data(parquet_metadata) = metadata_decoder.try_decode().unwrap() else { panic!("failed to decode metadata") };
# let parquet_metadata = Arc::new(parquet_metadata);
// The file length and metadata are required to create the decoder
let mut decoder =
    ParquetPushDecoderBuilder::try_new_decoder(parquet_metadata)
      .unwrap()
      // Optionally configure the decoder, e.g. batch size
      .with_batch_size(1024)
      // Build the decoder
      .build()
      .unwrap();

    // In a loop, ask the decoder what it needs next, and provide it with the required data
    loop {
        match decoder.try_decode().unwrap() {
            DecodeResult::NeedsData(ranges) => {
                // The decoder needs more data. Fetch the data for the given ranges
                let data = ranges.iter().map(|r| get_range(r)).collect::<Vec<_>>();
                // Push the data to the decoder
                decoder.push_ranges(ranges, data).unwrap();
                // After pushing the data, we can try to decode again on the next iteration
            }
            DecodeResult::Data(batch) => {
                // Successfully decoded a batch of data
                assert!(batch.num_rows() > 0);
            }
            DecodeResult::Finished => {
                // The decoder has finished decoding exit the loop
                break;
            }
        }
    }
```

# Adaptive scans

The scan strategy is not fixed once [`build`](Self::build) is called: it
can be changed *while decoding*, at row-group boundaries.

The important API for this is [`ParquetPushDecoder::try_next_reader`].
Unlike [`try_decode`](ParquetPushDecoder::try_decode), which barrels
straight through row-group boundaries, `try_next_reader` returns once per
row group — leaving a clean window *between* row groups. At any such
boundary, [`ParquetPushDecoder::into_builder`] hands back a
`ParquetPushDecoderBuilder` for the row groups not yet decoded. Change any
option on it (projection, row filter, row selection policy, …) and
[`build`](Self::build) a fresh decoder that resumes from the next row
group. This is how a query engine promotes or demotes filters — for
example turning a row filter on or off — based on the selectivity observed
in the row groups decoded so far.

```
# use std::ops::Range;
# use std::sync::Arc;
# use bytes::Bytes;
# use arrow_array::record_batch;
# use parquet::DecodeResult;
# use parquet::arrow::ProjectionMask;
# use parquet::arrow::push_decoder::ParquetPushDecoderBuilder;
# use parquet::arrow::ArrowWriter;
# use parquet::file::metadata::ParquetMetaDataPushDecoder;
# use parquet::file::properties::WriterProperties;
# let file_bytes = {
#   let batch = record_batch!(
#       ("a", Int32, [1, 2, 3, 4, 5, 6]),
#       ("b", Int32, [6, 5, 4, 3, 2, 1])
#   ).unwrap();
#   // Small row groups so the test file has two of them.
#   let props = WriterProperties::builder().set_max_row_group_row_count(Some(3)).build();
#   let mut buffer = vec![];
#   let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), Some(props)).unwrap();
#   writer.write(&batch).unwrap();
#   writer.close().unwrap();
#   Bytes::from(buffer)
# };
# let get_range = |r: &Range<u64>| file_bytes.slice(r.start as usize..r.end as usize);
# let file_length = file_bytes.len() as u64;
# let mut metadata_decoder = ParquetMetaDataPushDecoder::try_new(file_length).unwrap();
# metadata_decoder.push_ranges(vec![0..file_length], vec![file_bytes.clone()]).unwrap();
# let DecodeResult::Data(parquet_metadata) = metadata_decoder.try_decode().unwrap() else { panic!() };
# let parquet_metadata = Arc::new(parquet_metadata);
let mut decoder = ParquetPushDecoderBuilder::try_new_decoder(parquet_metadata)
    .unwrap()
    .build()
    .unwrap();

// Drive the decoder one row group at a time with `try_next_reader`.
loop {
    match decoder.try_next_reader().unwrap() {
        DecodeResult::NeedsData(ranges) => {
            // Fetch and hand over the bytes the decoder asked for.
            let data = ranges.iter().map(|r| get_range(r)).collect();
            decoder.push_ranges(ranges, data).unwrap();
        }
        DecodeResult::Data(reader) => {
            // Decode this row group's batches.
            for batch in reader {
                assert!(batch.unwrap().num_rows() > 0);
            }
            // We are now at a row-group boundary. Based on whatever stats
            // were gathered, optionally change strategy for the row groups
            // still to come: drop or promote a row filter, narrow or widen
            // the projection, etc.
            if decoder.is_at_row_group_boundary() && decoder.row_groups_remaining() > 0 {
                let builder = decoder.into_builder().unwrap();
                // e.g. column "b" turned out not to be needed.
                let projection = ProjectionMask::columns(builder.parquet_schema(), ["a"]);
                decoder = builder.with_projection(projection).build().unwrap();
            }
        }
        DecodeResult::Finished => break,
    }
}
```

---
