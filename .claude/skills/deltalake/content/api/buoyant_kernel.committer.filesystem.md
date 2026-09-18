# `buoyant_kernel::committer::filesystem`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.committer.filesystem.json`](../model/buoyant_kernel.committer.filesystem.json)

## FileSystemCommitter

`struct` · `buoyant_kernel::committer::filesystem::FileSystemCommitter`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.committer.filesystem.FileSystemCommitter.md)

Also reachable as `buoyant_kernel::committer::FileSystemCommitter`, `delta_kernel::committer::filesystem::FileSystemCommitter`

```rust
struct FileSystemCommitter
```

**Implements**: `buoyant_kernel::committer::Committer`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `buoyant_kernel::committer::Committer`**

```rust
fn commit(&self, engine: &dyn Engine, actions: DeltaResultIterator<'_, FilteredEngineData>, commit_metadata: CommitMetadata) -> DeltaResult<CommitResponse>
fn is_catalog_committer(&self) -> bool
fn publish(&self, _engine: &dyn Engine, publish_metadata: PublishMetadata) -> DeltaResult<()>
```

The `FileSystemCommitter` is an internal implementation of the `Committer` trait which
commits to a file system directly via `Engine::json_handler().write_json_file` for
non-catalog-managed tables.

SAFETY: it is _incorrect_ to use this committer for catalog-managed tables.

---
