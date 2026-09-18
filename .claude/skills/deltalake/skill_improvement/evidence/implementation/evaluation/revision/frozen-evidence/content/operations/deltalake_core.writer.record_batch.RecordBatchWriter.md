# `deltalake_core::writer::record_batch::RecordBatchWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.record_batch.RecordBatchWriter.json).

<a id="op-ebb06e0ab50094f6cee95246"></a>
## RecordBatchWriter

`struct` · `deltalake_core::writer::record_batch::RecordBatchWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RecordBatchWriter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L45).

Source: `crates/core/src/writer/record_batch.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Writes messages to a delta lake table.

Batches are streamed to storage as they are written, and a flush window
commits all-or-nothing: if any write returns an error — including a
transient IO error from the object store — every batch buffered since the
last flush is discarded along with the failing one, and the caller must
re-write all of them. (Validation errors caught before the batch reaches
storage fail only that call and leave the window untouched.)

<a id="op-a2499a817966baf39c56c42d"></a>
## arrow_schema

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::arrow_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L267).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:267`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the writer's current arrow schema.

<a id="op-3c0ba892850431f2d7f707c8"></a>
## buffer_len

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::buffer_len` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn buffer_len(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L237).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:237`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Approximate encoded (parquet) size written since the last flush,
including files already finalized by a size roll or `MergeSchema`
rotation. Monotonic within a flush window, so usable as a threshold for
calling [`flush`](Self::flush).

<a id="op-a4d8d0237e9d70ab7226c9d1"></a>
## buffered_record_batch_count

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::buffered_record_batch_count` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn buffered_record_batch_count(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L242).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:242`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the number of record batches streamed since the last flush.

<a id="op-92ae5629191e7b980a2c6821"></a>
## flush

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::flush` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn flush(&mut self) -> Result<Vec<Add>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L393).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [442, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/record_batch.rs:393`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Finalize all files written since the last flush and return their [`Add`](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a)
actions, resetting internal state to handle another flush window.

<a id="op-aa37aeba99ca886c8ffe891b"></a>
## flush_and_commit

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::flush_and_commit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn flush_and_commit(&mut self, table: &mut DeltaTable) -> Result<Version, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L398).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [442, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/record_batch.rs:398`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Flush buffered files and commit them to the Delta log, creating a new version.

<a id="op-8cd333d429b0e7a78c449fa8"></a>
## fmt

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L54).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [57, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/writer/record_batch.rs:54`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a4a450bcfec64b4789c3c06"></a>
## for_blind_appends

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::for_blind_appends` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn for_blind_appends(table: &table::BlindDeltaTable) -> Result<Self, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L157).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:157`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a [`RecordBatchWriter`](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md#op-ebb06e0ab50094f6cee95246) to write data to an [`BlindDeltaTable`].

This is optimized for append-only writes where file statistics are not needed
during table loading.

[`BlindDeltaTable`]: crate::table::AppendableDeltaTable

<a id="op-ed1304f90217d8fc14f245c9"></a>
## for_table

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::for_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn for_table(table: &DeltaTable) -> Result<Self, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L129).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a [`RecordBatchWriter`](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md#op-ebb06e0ab50094f6cee95246) to write data to provided Delta Table

<a id="op-1e53e4dded1b255a594a5626"></a>
## reset

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::reset` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn reset(&mut self)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L253).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

**Reference annotation (source_runtime_reconciliation, separate from upstream):** The upstream phrase about flush committing buffered data concerns file finalization, not publication of a Delta log version. flush returns Add actions; use a commit operation to make them visible. write can already perform storage I/O and finalize files on size/schema rotation. [Evidence](../capabilities/delta.write.md).

Resets internal state, discarding any data written since the last flush.

The sink streams to storage as it writes: open files' in-progress
multipart uploads are aborted in the background, while files already
finalized by a size roll or `MergeSchema` rotation are left unreferenced
for a later vacuum. Call [`flush`](Self::flush) instead to commit
buffered data.

<a id="op-0dcd0b0dced044b59e725b55"></a>
## try_new

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(table_uri: impl AsRef<str>, schema: ArrowSchemaRef, partition_columns: Option<Vec<String>>, storage_options: Option<HashMap<String, String>>) -> Result<Self, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L61).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`RecordBatchWriter`](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md#op-ebb06e0ab50094f6cee95246) instance

<a id="op-8656972077dbbf7819b6eae7"></a>
## try_new_checked

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::try_new_checked` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new_checked(table_uri: impl AsRef<str>, schema: ArrowSchemaRef, partition_columns: Option<Vec<String>>, storage_options: Option<HashMap<String, String>>) -> Result<Self, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L91).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:91`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`RecordBatchWriter`](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md#op-ebb06e0ab50094f6cee95246) for an existing table after validating table metadata.

<a id="op-99d04e56e8394c034d7e8202"></a>
## with_commit_properties

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::with_commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_commit_properties(self, properties: CommitProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add the [CommitProperties](../operations/deltalake_core.kernel.transaction.CommitProperties.md#op-0dd291fe008825d5408d61a7) to the [RecordBatchWriter](../operations/deltalake_core.writer.record_batch.RecordBatchWriter.md#op-ebb06e0ab50094f6cee95246) to be used when the writer flushes
the write into storage.

This can be useful for situations where slight modifications to the commit behavior are
required.

<a id="op-a277c10452a3b6627d95f58f"></a>
## with_target_file_size

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::with_target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_target_file_size(self, target_file_size: u64) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L260).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:260`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets a target file size; once an in-progress file reaches it the writer
finalizes it and rolls a new one. Without this — or with `0`, meaning no
limit — the writer emits a single file per partition per flush.

<a id="op-1ac72ff40b82602948f8f26b"></a>
## with_writer_properties

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::with_writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L272).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:272`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sets the writer properties for the underlying arrow writer.

<a id="op-96da0ba25156f1ed69042f33"></a>
## write

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write(&mut self, values: RecordBatch) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L326).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [442, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/record_batch.rs:326`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write a chunk of values into the internal write buffers with the default write mode

<a id="op-6f9487a75e98974e5f020a07"></a>
## write_partition

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::write_partition` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_partition(&mut self, record_batch: RecordBatch, partition_values: &IndexMap<String, Scalar>, mode: WriteMode) -> Result<ArrowSchemaRef, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L293).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [321, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/writer/record_batch.rs:293`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write a record batch that belongs entirely to the partition identified by
`partition_values`, streaming it into the partition's open file.

The batch may be provided with or without its partition columns (they are
stripped before encoding) but must otherwise conform to the writer's
schema. Returns the writer's current arrow schema.

With [`WriteMode::MergeSchema`](../operations/deltalake_core.writer.WriteMode.md#op-e42b4c72e462a41d8a8b74f6) new columns widen the writer's schema
(sealing the files written so far) and the merged schema is returned; a
widening write on a partitioned table is rejected as unsupported.
A later [`flush_and_commit`](super::DeltaWriter::flush_and_commit) commits
the evolved metadata along with the data; on the [`flush`](super::DeltaWriter::flush)
+ manual-commit path, committing the evolved metadata is the caller's
responsibility.

Validation errors fail only this call and leave the flush window untouched.

<a id="op-352d4ffd349f333ca6feb71c"></a>
## write_with_mode

`function` · `deltalake_core::writer::record_batch::RecordBatchWriter::write_with_mode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_with_mode(&mut self, values: RecordBatch, mode: WriteMode) -> Result<(), DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L337).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::record_batch::RecordBatchWriter", "path": "RecordBatchWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 1], "end": [442, 2], "filename": "crates/core/src/writer/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}], "constraints": []}}, "id": "deltalake_core::writer::DeltaWriter", "path": "DeltaWriter"}, "trait_path": "deltalake_core::writer::DeltaWriter"}`

Source: `crates/core/src/writer/record_batch.rs:337`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stream a record batch (partition columns included) into the dataset
writer, resolving schema evolution. Partitioning and parquet encoding
happen incrementally; files are finalized at flush (or when a target file
size is reached, or when a `MergeSchema` write widens the schema).

Validation errors — schema mismatch, or nulls in a non-nullable column
(reported with row indices) — fail only this call and leave the flush
window untouched.

<a id="op-3dc627129a031eb8eb4d600b"></a>
## commit_properties

`struct_field` · `deltalake_core::writer::record_batch::RecordBatchWriter::commit_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_properties: Option<kernel::transaction::CommitProperties>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L50).

Source: `crates/core/src/writer/record_batch.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17b0c3b259f17bf73643a1af"></a>
## window

`struct_field` · `deltalake_core::writer::record_batch::RecordBatchWriter::window` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
window: super::window::WriteWindow
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L49).

Source: `crates/core/src/writer/record_batch.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

All mutable per-flush-window state (open sink, sealed rotations, current
and committed schema, batch count). See [`WriteWindow`] for the invariant
it enforces.

Unresolved upstream links (retained, not inferred): ``WriteWindow``.
