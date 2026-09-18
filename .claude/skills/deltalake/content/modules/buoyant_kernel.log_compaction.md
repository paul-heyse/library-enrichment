# `buoyant_kernel::log_compaction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_compaction.json).

<a id="op-490149c9991f098af05ed36a"></a>
## log_compaction

`module` · `buoyant_kernel::log_compaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod log_compaction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

# Log Compaction

Log compaction is now fully enabled and functional.

This module provides an API for writing log compaction files that aggregate
multiple commit JSON files into single compacted files. This improves performance
by reducing the number of individual log files that need to be processed during
log replay operations.

## Overview

Log compaction creates files with the naming pattern
`{start_version}.{end_version}.compacted.json` that contain the reconciled actions from all
commit files in the specified version range. Only commit/compaction files that intersect with
[start_version, end_version] are processed. Note that `end_version` must be greater than
`start_version` (equal versions are not allowed). This is similar to checkpoints but operates on
a subset of versions rather than the entire table.

## Usage

The log compaction API follows a similar pattern to the checkpoint API:

1. Create a [`LogCompactionWriter`](../operations/buoyant_kernel.log_compaction.writer.LogCompactionWriter.md#op-17a0145eb5a991e6c3d78695) using [`crate::Snapshot::log_compaction_writer`] to compact
   the log from a given start_version to end_version (inclusive)
2. Get the compaction path from [`LogCompactionWriter::compaction_path`](../operations/buoyant_kernel.log_compaction.writer.LogCompactionWriter.md#op-912bbfe809d7ff9700198dfb)
3. Get the compaction data from [`LogCompactionWriter::compaction_data`]
4. Write the data to the path in cloud storage (engine-specific)

## Example

```no_run
# use std::sync::Arc;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::{ActionReconciliationIterator, LogCompactionWriter};
# use delta_kernel::{Engine, Snapshot, DeltaResult, Error, FileMeta};
# use url::Url;

// Engine-specific function to write compaction data
fn write_compaction_file(path: &Url, data: ActionReconciliationIterator) -> DeltaResult<FileMeta> {
    // In a real implementation, this would write the data to cloud storage
    todo!("Write data batches to storage at path: {}", path)
}

# fn example(engine: &dyn Engine) -> DeltaResult<()> {
// Create a snapshot for the table
let table_root = Url::parse("file:///path/to/table")?;
let snapshot = Snapshot::builder_for(table_root).build(engine)?;

// Create a log compaction writer for versions 10-20
let mut writer = snapshot.log_compaction_writer(10, 20)?;

let compaction_data = writer.compaction_data(engine)?;
let compaction_path = writer.compaction_path();

// Write the compaction data to cloud storage
let _metadata: FileMeta = write_compaction_file(compaction_path, compaction_data)?;
# Ok(())
# }
```

## When to Use Log Compaction

Log compaction is beneficial when:
- Table has many small commit files that slow down log replay
- Reduce the number of files without creating a full checkpoint
- Optimize specific version ranges that are frequently accessed

The [`should_compact`](../operations/buoyant_kernel.log_compaction.writer.should_compact.md#op-3951f081c5c4aed237cd04a7) utility function can help determine when compaction is appropriate
based on version intervals.

Please see <https://github.com/delta-io/delta/blob/master/PROTOCOL.md#log-compaction-files>
for more details

## Relationship to Checkpoints

- **Checkpoints**: Aggregate the entire table state up to a specific version
- **Log Compaction**: Aggregates only a specific range of commit files
- Both use similar action reconciliation logic but serve different use cases

Unresolved upstream links (retained, not inferred): ``crate::Snapshot::log_compaction_writer``, ``LogCompactionWriter::compaction_data``.
