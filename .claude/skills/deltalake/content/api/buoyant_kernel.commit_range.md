# `buoyant_kernel::commit_range`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.commit_range.json`](../model/buoyant_kernel.commit_range.json)

## CommitRange

`struct` · `buoyant_kernel::commit_range::CommitRange`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.commit_range.CommitRange.md)

Also reachable as `delta_kernel::commit_range::CommitRange`

```rust
struct CommitRange
```

**Derives**: Debug

**Methods** (6)

```rust
fn builder_for(table_root: impl AsRef<str>, start_version: Version) -> CommitRangeBuilder
fn builder_from(snapshot: SnapshotRef, start_version: Version) -> CommitRangeBuilder
fn commits(&self, engine: Arc<dyn Engine>, start_snapshot: Option<SnapshotRef>, actions: &[DeltaAction]) -> DeltaResult<impl Iterator<Item = DeltaResult<CommitAction>> + Send>
fn end_version(&self) -> Version
fn start_version(&self) -> Version
fn table_root(&self) -> &Url
```

A contiguous range of Delta commits, holding resolved `[start_version, end_version]` bounds
plus the materialized commit-file pointers in `commit_files`.

The pointer order matches the [`CommitOrdering`] requested at build time (ascending by
default; reversed if [`CommitOrdering::DescendingOrder`]). Reading the underlying actions is
lazy via [`CommitRange::commits`].

---
