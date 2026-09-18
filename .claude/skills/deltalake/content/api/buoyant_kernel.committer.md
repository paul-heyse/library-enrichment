# `buoyant_kernel::committer`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.committer.json`](../model/buoyant_kernel.committer.json)

## Committer

`trait` · `buoyant_kernel::committer::Committer`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.committer.Committer.md)

Also reachable as `delta_kernel::committer::Committer`

```rust
trait Committer: Send
```

**Implementors** (1)

- `buoyant_kernel::committer::filesystem::FileSystemCommitter`

**Methods** (3)

```rust
fn commit(&self, engine: &dyn Engine, actions: DeltaResultIterator<'_, FilteredEngineData>, commit_metadata: CommitMetadata) -> DeltaResult<CommitResponse>
fn is_catalog_committer(&self) -> bool
fn publish(&self, engine: &dyn Engine, publish_metadata: PublishMetadata) -> DeltaResult<()>
```

A Committer is the system by which transactions are committed to a table. Transactions are
effectively a collection of actions performed on the table at a specific version. The kernel
exposes this trait so different catalogs can build their own commit implementations. For
example, different catalogs may: commit directly to a database, commit to an object store, or
use another system entirely.

Critically, a Committer must implement [`commit`] which takes an engine and an iterator of
actions (as [`EngineData`] batches) to commit to the table at the given version
([`CommitMetadata::version`]).

[`commit`]: Committer::commit
[`EngineData`]: crate::EngineData

---
