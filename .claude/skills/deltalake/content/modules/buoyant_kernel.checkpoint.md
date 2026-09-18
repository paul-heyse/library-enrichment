# `buoyant_kernel::checkpoint`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.json).

<a id="op-00431814b37f77a78d019358"></a>
## checkpoint

`module` · `buoyant_kernel::checkpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod checkpoint
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This module implements the API for **customized** Delta checkpoint writes, where the
caller drives the write themselves. The entry point is [`Snapshot::create_checkpoint_writer`].

If you want an all-in-one API that handles writing the checkpoint, use
[`Snapshot::checkpoint`] instead.

## Checkpoint Types and Selection Logic
This API supports two checkpoint types, selected based on table features:

| Table Feature    | Resulting Checkpoint Type                   | Description                                                                                                             |
|------------------|---------------------------------------------|-------------------------------------------------------------------------------------------------------------------------|
| No v2Checkpoints | Single-file Classic-named V1                | Follows V1 specification without [`CheckpointMetadata`] action                                                          |
| v2Checkpoints    | Classic-named V2 (with or without sidecars) | Follows V2 specification with [`CheckpointMetadata`] action while maintaining backward compatibility via classic naming |

For more information on the V1/V2 specifications, see the following protocol section:
<https://github.com/delta-io/delta/blob/master/PROTOCOL.md#checkpoint-specs>

## Architecture

- [`CheckpointWriter`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-a939c219c148fee02b6e7899) - Core component that manages the checkpoint creation workflow
- [`ActionReconciliationIterator`](../operations/buoyant_kernel.action_reconciliation.log_replay.ActionReconciliationIterator.md#op-e9d4036b573b4f1725f0c862) - Iterator over the checkpoint data to be written

## Usage

The following steps outline the process of creating a checkpoint:

1. Create a [`CheckpointWriter`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-a939c219c148fee02b6e7899) using [`Snapshot::create_checkpoint_writer`]
2. Get the checkpoint path from [`CheckpointWriter::checkpoint_path`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-0826bd673b994a7fe038b8f4)
3. Get the checkpoint data from [`CheckpointWriter::checkpoint_data`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-45760f00db3731fb219db989)
4. Write the data to the path in object storage (engine-specific)
5. Collect metadata ([`FileMeta`]) from the write operation
6. Build a [`LastCheckpointHintStats`](../operations/buoyant_kernel.checkpoint.LastCheckpointHintStats.md#op-c0d681511b8dfe300769514f) from the exhausted iterator state
7. Pass the [`LastCheckpointHintStats`](../operations/buoyant_kernel.checkpoint.LastCheckpointHintStats.md#op-c0d681511b8dfe300769514f) to [`CheckpointWriter::finalize`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-ef0e358abb0e1bd2b521211e)

```no_run
# use std::sync::Arc;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::ActionReconciliationIterator;
# use delta_kernel::checkpoint::CheckpointWriter;
# use delta_kernel::Engine;
# use delta_kernel::Snapshot;
# use delta_kernel::SnapshotRef;
# use delta_kernel::DeltaResult;
# use delta_kernel::Error;
# use delta_kernel::FileMeta;
# use url::Url;
fn write_checkpoint_file(path: Url, data: ActionReconciliationIterator) -> DeltaResult<FileMeta> {
    todo!() /* engine-specific logic to write data to object storage*/
}

let engine: &dyn Engine = todo!(); /* create engine instance */

// Create a snapshot for the table at the version you want to checkpoint
let url = delta_kernel::try_parse_uri("./tests/data/app-txn-no-checkpoint")?;
let snapshot = Snapshot::builder_for(url).build(engine)?;

// Create a checkpoint writer from the snapshot
let writer = snapshot.create_checkpoint_writer(engine)?;

// Get the checkpoint path and data
let checkpoint_path = writer.checkpoint_path()?;
let checkpoint_data = writer.checkpoint_data(engine)?;

// Get the iterator state before consuming the data
let state = checkpoint_data.state();

// Write the checkpoint data to the object store and collect metadata
// The write function consumes the iterator, dropping its Arc reference to the state.
let metadata: FileMeta = write_checkpoint_file(checkpoint_path, checkpoint_data)?;
/* IMPORTANT: All data must be written before finalizing the checkpoint */

// Build the [`LastCheckpointHintStats`] from the exhausted iterator state
let state = std::sync::Arc::into_inner(state)
    .ok_or(Error::internal_error("checkpoint state Arc still has other references"))?;
let last_checkpoint_stats =
    delta_kernel::checkpoint::LastCheckpointHintStats::from_reconciliation_state(
        state,
        metadata.size,
        0, /* num_sidecars */
    )?;

// Finalize the checkpoint by passing the stats
writer.finalize(engine, &last_checkpoint_stats)?;

# Ok::<_, Error>(())
```

## Warning
Multi-part (V1) checkpoints are DEPRECATED and UNSAFE.

## Note
We currently do not plan to support UUID-named V2 checkpoints, since S3's put-if-absent
semantics remove the need for UUIDs to ensure uniqueness. Supporting only classic-named
checkpoints avoids added complexity, such as coordinating naming decisions between kernel and
engine, and handling coexistence with legacy V1 checkpoints. If a compelling use case arises
in the future, we can revisit this decision.

[`CheckpointMetadata`]: crate::actions::CheckpointMetadata
[`FileMeta`]: crate::FileMeta
[`LastCheckpointHint`]: crate::last_checkpoint_hint::LastCheckpointHint
[`Snapshot::checkpoint`]: crate::Snapshot::checkpoint
[`Snapshot::create_checkpoint_writer`]: crate::Snapshot::create_checkpoint_writer
