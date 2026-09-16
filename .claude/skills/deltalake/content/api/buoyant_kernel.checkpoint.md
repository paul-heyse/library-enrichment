# `buoyant_kernel::checkpoint`

Crate `buoyant_kernel` · 5 public items · structured records in [`model/buoyant_kernel.checkpoint.json`](../model/buoyant_kernel.checkpoint.json)

## DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT

`constant` · `buoyant_kernel::checkpoint::DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT`

Also reachable as `delta_kernel::checkpoint::DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT`

```rust
const DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT: usize = 50_000
```

Default value for [`V2CheckpointConfig::WithSidecar::file_actions_per_sidecar_hint`].
It's the suggested upper bound of file actions (`add` and `remove`) per sidecar file when
the caller does not provide an explicit hint.

---

## CheckpointSpec

`enum` · `buoyant_kernel::checkpoint::CheckpointSpec`

Also reachable as `delta_kernel::checkpoint::CheckpointSpec`

```rust
enum CheckpointSpec
```

**Variants**: `V1`, `V2`

**Derives**: Debug

Specifies the checkpoint format and behavior.

---

## V2CheckpointConfig

`enum` · `buoyant_kernel::checkpoint::V2CheckpointConfig`

Also reachable as `delta_kernel::checkpoint::V2CheckpointConfig`

```rust
enum V2CheckpointConfig
```

**Variants**: `NoSidecar`, `WithSidecar`

**Derives**: Debug

Configuration for V2 checkpoints.

Note: "File actions" here means `add` and `remove` actions. "Non-file actions" means
the rest (`protocol`, `metaData`, `txn`, etc.).

---

## CheckpointWriter

`struct` · `buoyant_kernel::checkpoint::CheckpointWriter`

Also reachable as `delta_kernel::checkpoint::CheckpointWriter`

```rust
struct CheckpointWriter
```

**Implements**: `buoyant_kernel::action_reconciliation::RetentionCalculator`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn checkpoint_data(&self, engine: &dyn Engine) -> DeltaResult<ActionReconciliationIterator>
fn checkpoint_path(&self) -> DeltaResult<Url>
fn finalize(self, engine: &dyn Engine, last_checkpoint_stats: &LastCheckpointHintStats) -> DeltaResult<()>
```

**via `buoyant_kernel::action_reconciliation::RetentionCalculator`**

```rust
fn table_properties(&self) -> &TableProperties
```

Orchestrates the process of creating a checkpoint for a table.

The [`CheckpointWriter`] is the entry point for generating checkpoint data for a Delta table.
It automatically selects the appropriate checkpoint format (V1/V2) based on whether the table
supports the `v2Checkpoints` reader/writer feature.

# Warning
The checkpoint data must be fully written to storage before calling
[`CheckpointWriter::finalize`]. Failing to do so may result in data loss or corruption.

# See Also
See the [module-level documentation](self) for the complete checkpoint workflow

---

## LastCheckpointHintStats

`struct` · `buoyant_kernel::checkpoint::LastCheckpointHintStats`

Also reachable as `delta_kernel::checkpoint::LastCheckpointHintStats`

```rust
struct LastCheckpointHintStats
```

**Derives**: Debug

**Methods** (1)

```rust
fn from_reconciliation_state(state: ActionReconciliationIteratorState, size_in_bytes: u64, num_sidecars: u64) -> DeltaResult<Self>
```

Information about a freshly-written checkpoint. Pass it to
[`CheckpointWriter::finalize`] to produce the `_last_checkpoint` hint file.

Unlike [`LastCheckpointHint`], which is used on the read path and has many optional
fields, this struct focuses on the write path and all fields are required. Note this
is a kernel requirement; the Delta protocol itself marks some of these fields as
optional in the `_last_checkpoint` hint. See the [Last Checkpoint File Schema] for
more details.

Construct via [`LastCheckpointHintStats::from_reconciliation_state`].

# Note
This is intended for sophisticated connectors that customize how checkpoints are
written. If you are not deeply familiar with the Delta protocol, you may use
[`Snapshot::checkpoint`](crate::snapshot::Snapshot::checkpoint), which handles
checkpoint writing end-to-end.

[Last Checkpoint File Schema]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#last-checkpoint-file-schema

---
