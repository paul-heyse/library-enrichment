# Maintenance

Maintenance is what keeps a Delta table fast rather than merely correct. Optimize compacts small files and can Z-order for multi-column skipping; vacuum deletes files no longer referenced. Vacuum is the one irreversible operation here: it deletes data that time travel and in-flight readers still need, which is exactly why the retention default is conservative.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::operations::optimize::OptimizeBuilder` | struct | 14 | [prose](../api/deltalake_core.operations.optimize.md#optimizebuilder) | [records](../model/deltalake_core.operations.optimize.json) |
| `deltalake_core::operations::vacuum::VacuumBuilder` | struct | 12 | [prose](../api/deltalake_core.operations.vacuum.md#vacuumbuilder) | [records](../model/deltalake_core.operations.vacuum.json) |
| `deltalake_core::operations::restore::RestoreBuilder` | struct | 9 | [prose](../api/deltalake_core.operations.restore.md#restorebuilder) | [records](../model/deltalake_core.operations.restore.json) |
| `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder` | struct | 6 | [prose](../api/deltalake_core.operations.filesystem_check.md#filesystemcheckbuilder) | [records](../model/deltalake_core.operations.filesystem_check.json) |
| `deltalake_core::operations::generate::GenerateBuilder` | struct | 4 | [prose](../api/deltalake_core.operations.generate.md#generatebuilder) | [records](../model/deltalake_core.operations.generate.json) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Run optimize on the partitions that changed, not the whole table.
- Z-order only pays for columns that are actually filtered on, and costs a full rewrite of the files it touches.
- Keep vacuum retention above the longest-running reader plus the time travel window you promise.

## Anti-patterns

- Lowering vacuum retention to reclaim space. It deletes files active readers are still using and destroys time travel. This is the one irreversible operation here.
- Expecting vacuum to make queries faster. It removes unreferenced files, which the query path never reads; compaction is what changes read performance.
- Z-ordering every column. The benefit dilutes with each column added, and the rewrite is not free.
- Running optimize and vacuum concurrently with heavy writes and expecting no conflicts.

## Agent checklist

- Is vacuum retention longer than the longest reader and the promised time travel window?
- Is optimize scoped to changed partitions?
