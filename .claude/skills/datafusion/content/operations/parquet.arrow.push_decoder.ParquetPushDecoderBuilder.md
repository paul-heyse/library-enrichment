# `parquet::arrow::push_decoder::ParquetPushDecoderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.push_decoder.ParquetPushDecoderBuilder.json).

<a id="op-be5e1b449f8e3584201abd08"></a>
## ParquetPushDecoderBuilder

`type_alias` · `parquet::arrow::push_decoder::ParquetPushDecoderBuilder` · parquet 59.3.0

```rust
type ParquetPushDecoderBuilder = arrow::arrow_reader::ArrowReaderBuilder<PushDecoderInput>
```

Source: `src/arrow/push_decoder/mod.rs:193`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A builder for [`ParquetPushDecoder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-4c043486cf5c22ecde7f148a).

To create a new decoder, use [`ParquetPushDecoderBuilder::try_new_decoder`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-9e58acd79ec0c339224c26c5).

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

The important API for this is [`ParquetPushDecoder::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6).
Unlike [`try_decode`](ParquetPushDecoder::try_decode), which barrels
straight through row-group boundaries, `try_next_reader` returns once per
row group — leaving a clean window *between* row groups. At any such
boundary, [`ParquetPushDecoder::into_builder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-dd963fb5591f7fda2ae03019) hands back a
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
