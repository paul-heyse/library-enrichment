# `parquet::arrow::push_decoder::ParquetPushDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.push_decoder.ParquetPushDecoder.json).

<a id="op-4c043486cf5c22ecde7f148a"></a>
## ParquetPushDecoder

`struct` · `parquet::arrow::push_decoder::ParquetPushDecoder` · parquet 59.3.0

```rust
struct ParquetPushDecoder
```

Source: `src/arrow/push_decoder/mod.rs:378`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A push based Parquet Decoder

See [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08) for an example of how to build and use the decoder.

[`ParquetPushDecoder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-4c043486cf5c22ecde7f148a) is a low level API for decoding Parquet data without an
underlying reader for performing IO, and thus offers fine grained control
over how data is fetched and decoded.

When more data is needed to make progress, instead of reading data directly
from a reader, the decoder returns [`DecodeResult`](../operations/parquet.DecodeResult.md#op-672a46d318916cbad7299962) indicating what ranges
are needed. Once the caller provides the requested ranges via
[`Self::push_ranges`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-36cb4a369d50d9e95db3eaec), they try to decode again by calling
[`Self::try_decode`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-6d44969e9b07c7b70719f977).

The decoder's internal state tracks what has been already decoded and what
is needed next.

<a id="op-73d1d43da55008b4b5cc12ee"></a>
## buffered_bytes

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::buffered_bytes` · parquet 59.3.0

```rust
fn buffered_bytes(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:508`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the total number of buffered bytes in the decoder

This is the sum of the size of all [`Bytes`] that has been pushed to the
decoder but not yet consumed.

Note that this does not include any overhead of the internal data
structures and that since [`Bytes`] are ref counted memory, this may not
reflect additional memory usage.

This can be used to monitor memory usage of the decoder.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-dae9e43904c0b0602fb674e3"></a>
## clear_all_ranges

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::clear_all_ranges` · parquet 59.3.0

```rust
fn clear_all_ranges(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:517`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Clear any staged byte ranges currently buffered for future decode work.

This clears byte ranges still owned by the decoder's internal
`PushBuffers`. It does not affect any data that has already been handed
off to an active [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3).

<a id="op-cb6748e3e4a6291cbf6ad1db"></a>
## fmt

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 10], "end": [377, 15], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/push_decoder/mod.rs:377`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd963fb5591f7fda2ae03019"></a>
## into_builder

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::into_builder` · parquet 59.3.0

```rust
fn into_builder(self) -> Result<ParquetPushDecoderBuilder, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:615`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decompose this decoder back into a [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08) for the
row groups that have *not* yet been decoded.

This is the API for *adaptive* scans. Drive the decoder with
[`Self::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6); at any row-group boundary, call
`into_builder` to recover a builder, adjust it with the usual
[`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08) setters, and
[`build`](ParquetPushDecoderBuilder::build) a fresh decoder that resumes
from the next row group:

```no_run
# use parquet::arrow::push_decoder::ParquetPushDecoder;
# use parquet::arrow::arrow_reader::RowFilter;
# fn get_decoder() -> ParquetPushDecoder { unimplemented!() }
# fn new_filter() -> RowFilter { unimplemented!() }
let mut decoder = get_decoder();
// ... drive `decoder.try_next_reader()` for a few row groups ...
if decoder.is_at_row_group_boundary() && decoder.row_groups_remaining() > 0 {
    decoder = decoder
        .into_builder()
        .unwrap()
        // any builder option can be changed here, e.g. promote a
        // filter into a row filter based on observed selectivity
        .with_row_filter(new_filter())
        .build()
        .unwrap();
}
```

The returned builder pins the not-yet-decoded row groups (via
[`with_row_groups`](ArrowReaderBuilder::with_row_groups)) and carries the
not-yet-consumed row selection and offset/limit budget, so rows from
already-decoded row groups are not produced again. Every other option —
projection, row filter, row selection policy, batch size, metrics,
predicate-cache size — is left exactly as the decoder had it and can be
overridden before [`build`](ParquetPushDecoderBuilder::build).

# Errors

Returns `Err(ParquetError::General)` when the decoder is not at a
row-group boundary (check [`Self::is_at_row_group_boundary`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-b6852ec8dcd780f1b29a8c1b) first) or
has already finished. The decoder is consumed either way.

# Buffered bytes

The decoder's buffered bytes are carried across the rebuild: bytes
already fetched for row groups the new configuration still reads are
not re-requested. Bytes the new configuration no longer needs stay
buffered until [`clear_all_ranges`](Self::clear_all_ranges) is called
or the rebuilt decoder is dropped.

<a id="op-b6852ec8dcd780f1b29a8c1b"></a>
## is_at_row_group_boundary

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::is_at_row_group_boundary` · parquet 59.3.0

```rust
fn is_at_row_group_boundary(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:531`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

True iff the decoder is at a row-group boundary, where
[`Self::into_builder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-dd963fb5591f7fda2ae03019) can reconfigure the scan.

A boundary is "between row groups": the previous row group's
[`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) has been fully extracted (via
[`Self::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6)) or fully drained (via [`Self::try_decode`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-6d44969e9b07c7b70719f977)),
and the next row group has not yet been planned. While
[`Self::try_decode`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-6d44969e9b07c7b70719f977) is iterating an active row group's reader this
returns `false`; with [`Self::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6) there is a clean
window between two consecutive returns where this is `true`.

<a id="op-d692284581c860cbfde5ea41"></a>
## peek_next_row_group

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::peek_next_row_group` · parquet 59.3.0

```rust
fn peek_next_row_group(&self) -> Result<Option<usize>, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:561`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the row-group index that the next call to
[`Self::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6) will yield a reader for, after applying
any internal skipping (row selection emptiness, exhausted budget,
finished state).

Safe to call at any time. When called mid-row-group (i.e. while a
previously-emitted [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) is still being
drained), the returned index refers to the row group that
[`Self::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6) will produce *after* the current one.

Returns `Ok(None)` when:
- the decoder has no more row groups to read, or
- every remaining row group would be skipped.

This method does not mutate decoder state. It is useful for
callers that maintain per-row-group state in lock-step with the
decoder (e.g. dynamic row-group pruners) to determine which row
group the next reader corresponds to, since
[`Self::try_next_reader`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-075b97a6ac48b171add8f1f6) may silently advance past row groups
based on filtering and other criteria.

<a id="op-7b1b6d505da5179d71fea8a1"></a>
## push_range

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::push_range` · parquet 59.3.0

```rust
fn push_range(&mut self, range: Range<u64>, data: Bytes) -> Result<(), ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:481`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Push data into the decoder for processing

This is a convenience wrapper around [`Self::push_ranges`](../operations/parquet.arrow.push_decoder.ParquetPushDecoder.md#op-36cb4a369d50d9e95db3eaec) for pushing a
single range of data.

Note this can be the entire file or just a part of it. If it is part of the file,
the ranges should correspond to the data ranges requested by the decoder.

See example in [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08)

<a id="op-36cb4a369d50d9e95db3eaec"></a>
## push_ranges

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::push_ranges` · parquet 59.3.0

```rust
fn push_ranges(&mut self, ranges: Vec<Range<u64>>, data: Vec<Bytes>) -> Result<(), ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:488`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Push data into the decoder for processing

This should correspond to the data ranges requested by the decoder

<a id="op-10f0d54a6fcd2e286eaf6d2f"></a>
## row_groups_remaining

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::row_groups_remaining` · parquet 59.3.0

```rust
fn row_groups_remaining(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:537`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of row groups left to decode after the one currently in flight.
Useful as a "should I bother reconfiguring the scan?" signal.

<a id="op-6d44969e9b07c7b70719f977"></a>
## try_decode

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::try_decode` · parquet 59.3.0

```rust
fn try_decode(&mut self) -> Result<DecodeResult<RecordBatch>, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:418`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

 Attempt to decode the next batch of data, or return what data is needed

 The the decoder communicates the next state with a [`DecodeResult`](../operations/parquet.DecodeResult.md#op-672a46d318916cbad7299962)

 See full example in [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08)

 ```no_run
 # use parquet::arrow::push_decoder::ParquetPushDecoder;
 use parquet::DecodeResult;
 # fn get_decoder() -> ParquetPushDecoder { unimplemented!() }
 # fn push_data(decoder: &mut ParquetPushDecoder, ranges: Vec<std::ops::Range<u64>>) { unimplemented!() }
 let mut decoder = get_decoder();
 loop {
    match decoder.try_decode().unwrap() {
       DecodeResult::NeedsData(ranges) => {
         // The decoder needs more data. Fetch the data for the given ranges
         // call decoder.push_ranges(ranges, data) and call again
         push_data(&mut decoder, ranges);
       }
       DecodeResult::Data(batch) => {
         // Successfully decoded the next batch of data
         println!("Got batch with {} rows", batch.num_rows());
       }
       DecodeResult::Finished => {
         // The decoder has finished decoding all data
         break;
       }
    }
 }
```

<a id="op-075b97a6ac48b171add8f1f6"></a>
## try_next_reader

`function` · `parquet::arrow::push_decoder::ParquetPushDecoder::try_next_reader` · parquet 59.3.0

```rust
fn try_next_reader(&mut self) -> Result<DecodeResult<ParquetRecordBatchReader>, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::push_decoder::ParquetPushDecoder", "path": "ParquetPushDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [387, 1], "end": [618, 2], "filename": "src/arrow/push_decoder/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/push_decoder/mod.rs:463`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

 Return a [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) that reads the next set of rows, or
 return what data is needed to produce it.

 This API can be used to get a reader for decoding the next set of
 RecordBatches while proceeding to begin fetching data for the set (e.g
 row group)

 Example
 ```no_run
 # use parquet::arrow::push_decoder::ParquetPushDecoder;
 use parquet::DecodeResult;
 # fn get_decoder() -> ParquetPushDecoder { unimplemented!() }
 # fn push_data(decoder: &mut ParquetPushDecoder, ranges: Vec<std::ops::Range<u64>>) { unimplemented!() }
 let mut decoder = get_decoder();
 loop {
    match decoder.try_next_reader().unwrap() {
       DecodeResult::NeedsData(ranges) => {
         // The decoder needs more data. Fetch the data for the given ranges
         // call decoder.push_ranges(ranges, data) and call again
         push_data(&mut decoder, ranges);
       }
       DecodeResult::Data(reader) => {
          // spawn a thread to read the batches in parallel
          // with fetching the next row group / data
          std::thread::spawn(move || {
            for batch in reader {
              let batch = batch.unwrap();
              println!("Got batch with {} rows", batch.num_rows());
            }
         });
       }
       DecodeResult::Finished => {
         // The decoder has finished decoding all data
         break;
       }
    }
 }
```
