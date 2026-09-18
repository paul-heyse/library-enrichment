# `parquet::arrow::async_reader::ParquetRecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_reader.ParquetRecordBatchStream.json).

<a id="op-5aaf529c72b40fe254f58220"></a>
## ParquetRecordBatchStream

`struct` · `parquet::arrow::async_reader::ParquetRecordBatchStream` · parquet 59.3.0

```rust
struct ParquetRecordBatchStream<T>
```

Source: `src/arrow/async_reader/mod.rs:783`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An asynchronous [`Stream`]of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) constructed using [`ParquetRecordBatchStreamBuilder`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStreamBuilder.md#op-36119f71505f76bb82a22932) to read parquet files.

`ParquetRecordBatchStream` also provides [`ParquetRecordBatchStream::next_row_group`](../operations/parquet.arrow.async_reader.ParquetRecordBatchStream.md#op-6a7b74ccf9b4191804415952) for fetching row groups,
allowing users to decode record batches separately from I/O.

# I/O Buffering

`ParquetRecordBatchStream` buffers *all* data pages selected after predicates
(projection + filtering, etc) and decodes the rows from those buffered pages.

For example, if all rows and columns are selected, the entire row group is
buffered in memory during decode. This minimizes the number of IO operations
required, which is especially important for object stores, where IO operations
have latencies in the hundreds of milliseconds

See [`ParquetPushDecoderBuilder`](../operations/parquet.arrow.push_decoder.ParquetPushDecoderBuilder.md#op-be5e1b449f8e3584201abd08) for an API with lower level control over
buffering.

[`Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html

<a id="op-db17bac4b28a8bfb118d6615"></a>
## Item

`assoc_type` · `parquet::arrow::async_reader::ParquetRecordBatchStream::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::ParquetRecordBatchStream", "path": "ParquetRecordBatchStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [863, 1], "end": [881, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/arrow/async_reader/mod.rs:867`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfff63f21c78dc417401f915"></a>
## fmt

`function` · `parquet::arrow::async_reader::ParquetRecordBatchStream::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::ParquetRecordBatchStream", "path": "ParquetRecordBatchStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [792, 1], "end": [798, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/async_reader/mod.rs:793`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a7b74ccf9b4191804415952"></a>
## next_row_group

`function` · `parquet::arrow::async_reader::ParquetRecordBatchStream::next_row_group` · parquet 59.3.0

```rust
async fn next_row_group(&mut self) -> Result<Option<ParquetRecordBatchReader>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::ParquetRecordBatchStream", "path": "ParquetRecordBatchStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [810, 1], "end": [861, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:827`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Fetches the next row group from the stream.

Users can continue to call this function to get row groups and decode them concurrently.

## Notes

ParquetRecordBatchStream should be used either as a `Stream` or with `next_row_group`; they should not be used simultaneously.

## Returns

- `Ok(None)` if the stream has ended.
- `Err(error)` if the stream has errored. All subsequent calls will return `Ok(None)`.
- `Ok(Some(reader))` which holds all the data for the row group.

<a id="op-3b4c4bf130dd2b1b7bad5410"></a>
## poll_next

`function` · `parquet::arrow::async_reader::ParquetRecordBatchStream::poll_next` · parquet 59.3.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::ParquetRecordBatchStream", "path": "ParquetRecordBatchStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::arrow::async_reader::AsyncFileReader", "path": "AsyncFileReader"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [863, 1], "end": [881, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/arrow/async_reader/mod.rs:868`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de7b88dbcad1c19af9a0d44d"></a>
## schema

`function` · `parquet::arrow::async_reader::ParquetRecordBatchStream::schema` · parquet 59.3.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "parquet::arrow::async_reader::ParquetRecordBatchStream", "path": "ParquetRecordBatchStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [800, 1], "end": [808, 2], "filename": "src/arrow/async_reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/async_reader/mod.rs:805`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the projected [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) for reading the parquet file.

Note that the schema metadata will be stripped here. See
[`ParquetRecordBatchStreamBuilder::schema`](../operations/parquet.arrow.arrow_reader.ArrowReaderBuilder.md#op-c408c3639fc9a3227bf40394) if the metadata is desired.
