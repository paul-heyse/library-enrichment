# `deltalake_core::writer::json::JsonWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.json.JsonWriter.json).

<a id="op-f17cd885f120835ecab1eedb"></a>
## JsonWriter

`struct` · `deltalake_core::writer::json::JsonWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct JsonWriter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L33).

Source: `crates/core/src/writer/json.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Writes messages to a delta lake table.

Batches are streamed to storage as they are written, and a flush window
commits all-or-nothing: if any write returns an error — including a
transient IO error from the object store — every batch buffered since the
last flush is discarded along with the failing one, and the caller must
re-write all of them. (Validation errors caught before the data reaches
storage fail only that call and leave the window untouched. A write with
malformed JSON records rejects the whole call and writes nothing.)

<a id="op-cccf5f3c602b8341d30d094c"></a>
## arrow_schema

`function` · `deltalake_core::writer::json::JsonWriter::arrow_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the arrow schema this writer encodes under (fixed at construction).

<a id="op-cdfdc5704651025dd5de33f4"></a>
## buffer_len

`function` · `deltalake_core::writer::json::JsonWriter::buffer_len` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn buffer_len(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L113).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Approximate encoded (parquet) size written since the last flush,
including files already finalized by a size roll. Monotonic within a
flush window, so usable as a threshold for calling [`flush`](Self::flush).

<a id="op-0e378633c56f60d29de1353c"></a>
## buffered_record_batch_count

`function` · `deltalake_core::writer::json::JsonWriter::buffered_record_batch_count` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn buffered_record_batch_count(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L118).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the number of record batches streamed since the last flush.

<a id="op-72d1903b3a565ce932c10d82"></a>
## flush

`function` · `deltalake_core::writer::json::JsonWriter::flush` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L253).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [260, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "serde_json::value::Value", "path": "Value"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/json.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Finalize all files written since the last flush and return their [`Add`](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a)
actions, resetting internal state to handle another flush window.

These actions should be committed to the [DeltaTable](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) for the written data.

<a id="op-fffa7780a78c24c1d52c25a6"></a>
## fmt

`function` · `deltalake_core::writer::json::JsonWriter::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [44, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/writer/json.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d9e086cd0929cd73797a8c5"></a>
## for_table

`function` · `deltalake_core::writer::json::JsonWriter::for_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn for_table(table: &DeltaTable) -> Result<JsonWriter, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a JsonWriter to write to the given table, using the table's schema.

<a id="op-dbccfa22d44be3e96f0683b6"></a>
## reset

`function` · `deltalake_core::writer::json::JsonWriter::reset` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn reset(&mut self)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Resets internal state, discarding any data written since the last flush.

The sink streams to storage as it writes: open files' in-progress
multipart uploads are aborted in the background, while files already
finalized by a size roll are left unreferenced for a later vacuum. Call
[`flush`](Self::flush) instead to commit buffered data.

<a id="op-c7a1c8e8872d63a05e679b21"></a>
## try_new

`function` · `deltalake_core::writer::json::JsonWriter::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(table_url: Url, schema_ref: ArrowSchemaRef, partition_columns: Option<Vec<String>>, storage_options: Option<HashMap<String, String>>) -> Result<Self, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L48).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:48`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new JsonWriter instance

<a id="op-b053044580d721c3e9f7e684"></a>
## with_target_file_size

`function` · `deltalake_core::writer::json::JsonWriter::with_target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_target_file_size(self, target_file_size: u64) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L139).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [149, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/json.rs:139`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets a target file size; once an in-progress file reaches it the writer
finalizes it and rolls a new one. Without this the writer emits a single
file per partition per flush (the default).

A `target_file_size` of `0` means "no limit": size-based rolling is
disabled and the writer keeps one file per partition per flush, exactly
as if this method had not been called.

<a id="op-1be527ce7d4bbe4f65c3b42a"></a>
## write

`function` · `deltalake_core::writer::json::JsonWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, values: Vec<Value>) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L197).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [260, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "serde_json::value::Value", "path": "Value"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/json.rs:197`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write a chunk of values into the internal write buffers with the default write mode

<a id="op-f429bbad207cf55d40c46bac"></a>
## write_with_mode

`function` · `deltalake_core::writer::json::JsonWriter::write_with_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_with_mode(&mut self, values: Vec<Value>, mode: WriteMode) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L212).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::json::JsonWriter", "path": "JsonWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [260, 2], "filename": "crates/core/src/writer/json.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "serde_json::value::Value", "path": "Value"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/json.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Decode the JSON values into a record batch and stream it into the dataset
writer; partitioning and parquet encoding happen incrementally, and files
are finalized at flush.

JSON decode and validation errors (wrong types, nulls in non-nullable
columns) are reported here, per write, as [`DeltaTableError::InvalidData`](../operations/deltalake_core.errors.DeltaTableError.md#op-4f49890dde81661f4f48264c)
naming the offending records by input index, and leave the flush window
untouched — drop or fix those records and retry the write. Parquet-encoding
/ object-store errors can surface here (encoding is incremental) or later at
[`flush`](JsonWriter::flush); those fail the whole flush window — every
batch since the last flush.

<a id="op-942bd466159b5435c7fe37c6"></a>
## window

`struct_field` · `deltalake_core::writer::json::JsonWriter::window` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
window: super::window::WriteWindow
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/json.rs#L37).

Source: `crates/core/src/writer/json.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

All mutable per-flush-window state. The schema is fixed at construction
(from the provided schema ref, or the table's schema). `JsonWriter` never
evolves the schema, so the window never widens.
