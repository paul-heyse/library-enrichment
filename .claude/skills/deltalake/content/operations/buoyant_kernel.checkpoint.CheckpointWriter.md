# `buoyant_kernel::checkpoint::CheckpointWriter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.CheckpointWriter.json).

<a id="op-a939c219c148fee02b6e7899"></a>
## CheckpointWriter

`struct` · `buoyant_kernel::checkpoint::CheckpointWriter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CheckpointWriter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L358).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:358`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Orchestrates the process of creating a checkpoint for a table.

The [`CheckpointWriter`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-a939c219c148fee02b6e7899) is the entry point for generating checkpoint data for a Delta table.
It automatically selects the appropriate checkpoint format (V1/V2) based on whether the table
supports the `v2Checkpoints` reader/writer feature.

# Warning
The checkpoint data must be fully written to storage before calling
[`CheckpointWriter::finalize`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-ef0e358abb0e1bd2b521211e). Failing to do so may result in data loss or corruption.

# See Also
See the [module-level documentation](self) for the complete checkpoint workflow

<a id="op-45760f00db3731fb219db989"></a>
## checkpoint_data

`function` · `buoyant_kernel::checkpoint::CheckpointWriter::checkpoint_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn checkpoint_data(&self, engine: &dyn Engine) -> DeltaResult<ActionReconciliationIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L468).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointWriter", "path": "CheckpointWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [760, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:468`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the checkpoint data to be written to the checkpoint file.

This method reads actions from the log segment, processes them for checkpoint creation,
and applies stats transforms based on table properties:
- `delta.checkpoint.writeStatsAsJson` (default: true)
- `delta.checkpoint.writeStatsAsStruct` (default: false)

The returned [`ActionReconciliationIterator`](../operations/buoyant_kernel.action_reconciliation.log_replay.ActionReconciliationIterator.md#op-e9d4036b573b4f1725f0c862) yields [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) batches with
stats transforms already applied. Use [`ActionReconciliationIterator::state`](../operations/buoyant_kernel.action_reconciliation.log_replay.ActionReconciliationIterator.md#op-a293d81ea9f984def192499b) to get the
shared state for building a [`LastCheckpointHintStats`](../operations/buoyant_kernel.checkpoint.LastCheckpointHintStats.md#op-c0d681511b8dfe300769514f) after the iterator is exhausted.

# Engine Usage

```ignore
let mut checkpoint_data = writer.checkpoint_data(&engine)?;
let state = checkpoint_data.state();
while let Some(batch) = checkpoint_data.next() {
    let data = batch?.apply_selection_vector()?;
    parquet_writer.write(&data).await?;
}
drop(checkpoint_data);
let state = Arc::into_inner(state)
    .ok_or(Error::internal_error("checkpoint state Arc still has other references"))?;
let last_checkpoint_stats =
    LastCheckpointHintStats::from_reconciliation_state(state, size_in_bytes, 0)?;
writer.finalize(&engine, &last_checkpoint_stats)?;
```

<a id="op-0826bd673b994a7fe038b8f4"></a>
## checkpoint_path

`function` · `buoyant_kernel::checkpoint::CheckpointWriter::checkpoint_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn checkpoint_path(&self) -> DeltaResult<Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L427).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointWriter", "path": "CheckpointWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [760, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:427`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the URL where the checkpoint file should be written.

This method generates the checkpoint path based on the table's root and the version
of the underlying snapshot being checkpointed. The resulting path follows the classic
Delta checkpoint naming convention (where the version is zero-padded to 20 digits):

`<table_root>/<version>.checkpoint.parquet`

For example, if the table root is `s3://bucket/path` and the version is `10`,
the checkpoint path will be: `s3://bucket/path/00000000000000000010.checkpoint.parquet`

<a id="op-35d9c870637236f527da451e"></a>
## clone

`function` · `buoyant_kernel::checkpoint::CheckpointWriter::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CheckpointWriter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L357).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointWriter", "path": "CheckpointWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 17], "end": [357, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:357`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef0e358abb0e1bd2b521211e"></a>
## finalize

`function` · `buoyant_kernel::checkpoint::CheckpointWriter::finalize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn finalize(self, engine: &dyn Engine, last_checkpoint_stats: &LastCheckpointHintStats) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L533).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointWriter", "path": "CheckpointWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [760, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:533`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Finalizes checkpoint creation by saving metadata about the checkpoint.

# Important
This method **must** be called only after:
1. The checkpoint data iterator has been fully exhausted
2. All data has been successfully written to object storage

# Parameters
- `engine`: Implementation of [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) APIs.
- `last_checkpoint_stats`: The [`LastCheckpointHintStats`](../operations/buoyant_kernel.checkpoint.LastCheckpointHintStats.md#op-c0d681511b8dfe300769514f) containing fields needed to write
  the `_last_checkpoint` file.

# Returns: `Ok` if the checkpoint was successfully finalized

<a id="op-f17351883e23af2e8de1fac2"></a>
## fmt

`function` · `buoyant_kernel::checkpoint::CheckpointWriter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L357).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointWriter", "path": "CheckpointWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 10], "end": [357, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:357`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f2a261e0675e357459347ec"></a>
## is_v2

`struct_field` · `buoyant_kernel::checkpoint::CheckpointWriter::is_v2` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
is_v2: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L367).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:367`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4623c0449d8188065401f038"></a>
## output_schema

`struct_field` · `buoyant_kernel::checkpoint::CheckpointWriter::output_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
output_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L369).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:369`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f588d22f08d009ef6645fc8c"></a>
## read_schema

`struct_field` · `buoyant_kernel::checkpoint::CheckpointWriter::read_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
read_schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L368).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e98e10d56d06e81b5aec1f6f"></a>
## snapshot

`struct_field` · `buoyant_kernel::checkpoint::CheckpointWriter::snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: snapshot::SnapshotRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L360).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:360`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reference to the snapshot (i.e. version) of the table being checkpointed

<a id="op-287165f5c36ad912876ae611"></a>
## table_properties

`function` · `buoyant_kernel::checkpoint::CheckpointWriter::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L374).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointWriter", "path": "CheckpointWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [373, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::action_reconciliation::RetentionCalculator", "path": "RetentionCalculator"}, "trait_path": "buoyant_kernel::action_reconciliation::RetentionCalculator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:374`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a14ba1a70d83185e7157ac8b"></a>
## transform_expr

`struct_field` · `buoyant_kernel::checkpoint::CheckpointWriter::transform_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
transform_expr: expressions::ExpressionRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L370).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:370`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b92ba95c8dda98ed8dad8dbe"></a>
## version

`struct_field` · `buoyant_kernel::checkpoint::CheckpointWriter::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L365).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:365`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of the snapshot being checkpointed.
Note: Although the version is stored as a u64 in the snapshot, it is stored as an i64
field here to avoid multiple type conversions.
