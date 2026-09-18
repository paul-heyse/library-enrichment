# `buoyant_kernel::committer`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.json).

<a id="op-69b504c43c61cb3d2ca60e68"></a>
## committer

`module` · `buoyant_kernel::committer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod committer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The `committer` module provides a [`Committer`](../operations/buoyant_kernel.committer.Committer.md#op-32f11d6fd06afd07c4c2a455) trait which allows different implementations to
define how to commit transactions to a catalog or filesystem. For catalog-managed tables, a
[`Committer`](../operations/buoyant_kernel.committer.Committer.md#op-32f11d6fd06afd07c4c2a455) specific to the managing catalog should be provided. For non-catalog-managed
tables, the [`FileSystemCommitter`](../operations/buoyant_kernel.committer.filesystem.FileSystemCommitter.md#op-c9d7ce0544dbb4d345b6296d) should be used to commit directly to the object store (via
put-if-absent call to storage to atomically write new commit files).

By implementing the [`Committer`](../operations/buoyant_kernel.committer.Committer.md#op-32f11d6fd06afd07c4c2a455) trait, different catalogs can define what happens when the
kernel needs to commit a transaction to a table. The goal terminal state of every
[`Transaction`] is to be committed to the table. This means writing the changes (we call these
actions) in the transaction as a new version of the table. The [`Committer`](../operations/buoyant_kernel.committer.Committer.md#op-32f11d6fd06afd07c4c2a455) trait exposes a
single method, [`commit`] which takes an engine, an iterator of actions (as [`EngineData`]
batches), and [`CommitMetadata`](../operations/buoyant_kernel.committer.commit_types.CommitMetadata.md#op-f0a4d85b92f1e66219725449) (which includes critical commit metadata like the version to
commit) to allow different catalogs to define what it means to 'commit' the actions to a table.
For some, this may mean writing staged commits to object storage and retaining an in-memory list
(server side) of commits. For others, this may mean writing new (version, actions) tuples to a
database.

The implementation of [`commit`] must ensure that the actions are committed atomically to the
table at the given version and either (1) persisted directly to object storage as published
deltas as in non-catalog-managed tables or (2) persisted within the catalog and made available
to readers during snapshot contstruction via the [`log_tail`] API.

[`Transaction`]: crate::transaction::Transaction
[`commit`]: crate::committer::Committer::commit
[`log_tail`]: crate::snapshot::SnapshotBuilder::with_log_tail
[`EngineData`]: crate::EngineData
