# `buoyant_kernel::committer::Committer`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.Committer.json).

<a id="op-32f11d6fd06afd07c4c2a455"></a>
## Committer

`trait` · `buoyant_kernel::committer::Committer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait Committer: Send
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/mod.rs#L56).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/mod.rs:56`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A Committer is the system by which transactions are committed to a table. Transactions are
effectively a collection of actions performed on the table at a specific version. The kernel
exposes this trait so different catalogs can build their own commit implementations. For
example, different catalogs may: commit directly to a database, commit to an object store, or
use another system entirely.

Critically, a Committer must implement [`commit`] which takes an engine and an iterator of
actions (as [`EngineData`] batches) to commit to the table at the given version
([`CommitMetadata::version`](../operations/buoyant_kernel.committer.commit_types.CommitMetadata.md#op-ffc5fefc6f9d1e05155c596f)).

[`commit`]: Committer::commit
[`EngineData`]: crate::EngineData

<a id="op-57d060399213d1e1597973da"></a>
## commit

`function` · `buoyant_kernel::committer::Committer::commit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commit(&self, engine: &dyn Engine, actions: DeltaResultIterator<'_, FilteredEngineData>, commit_metadata: CommitMetadata) -> DeltaResult<CommitResponse>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/mod.rs#L63).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/mod.rs:63`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Commits actions to the table at the version specified in [`CommitMetadata`](../operations/buoyant_kernel.committer.commit_types.CommitMetadata.md#op-f0a4d85b92f1e66219725449).

Implementations must ensure that actions are committed atomically and either:
1. Persisted directly to object storage as published deltas (for filesystem-based tables),
   or
2. Persisted as per the managing catalog's semantics (for catalog-managed tables)

<a id="op-a8bf5e5f970affff246f4684"></a>
## is_catalog_committer

`function` · `buoyant_kernel::committer::Committer::is_catalog_committer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_catalog_committer(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/mod.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/mod.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if this committer is for a catalog-managed table, else `false`.

<a id="op-280409d4aa4ba60251bd8819"></a>
## publish

`function` · `buoyant_kernel::committer::Committer::publish` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn publish(&self, engine: &dyn Engine, publish_metadata: PublishMetadata) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/mod.rs#L107).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/mod.rs:107`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Publishes catalog commits to the Delta log. Applicable only to catalog-managed tables.

Publishing is the act of copying ratified catalog commits to the Delta log as published
Delta files (e.g., `_delta_log/00000000000000000001.json`).

# When to call

This method should only be called on catalog committers (i.e., when [`is_catalog_committer`]
returns `true`). Filesystem committers will error if called with catalog commits to publish.

# Benefits

- Reduces the number of commits the catalog needs to store internally and serve to readers
- Enables table maintenance operations that must operate on published versions only, such as
  checkpointing and log compaction

# Requirements

- This method must ensure that all catalog commits are published to the Delta log up to and
  including the snapshot version specified in [`PublishMetadata`](../operations/buoyant_kernel.committer.publish_types.PublishMetadata.md#op-c9fe0cbc7fc4e65976b1d6ab)
- Commits must be published in order: version V-1 must be published before version V

# Catalog-specific semantics

Each catalog implementation may specify its own rules and semantics for publishing,
including whether it expects to be notified immediately upon publishing success, whether
published commits must appear with PUT-if-absent semantics in the Delta log, and whether
publishing happens in the client-side or server-side catalog component.

# Errors

Returns an error if the publish operation fails.

[`is_catalog_committer`]: Committer::is_catalog_committer
