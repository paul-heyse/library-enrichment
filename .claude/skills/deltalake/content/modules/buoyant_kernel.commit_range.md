# `buoyant_kernel::commit_range`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.commit_range.json).

<a id="op-234589133bf1990cbd24ed95"></a>
## commit_range

`module` · `buoyant_kernel::commit_range` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod commit_range
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read a contiguous range of raw Delta commits.

A [`CommitRange`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-7308777d4b0c84353f5470a4) holds an inclusive `[start_version, end_version]` range and the list of
commit files; reads are lazy (no JSON I/O until [`CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8) is called).

Two construction paths, differing only in how `_delta_log/` is listed at build time:
- [`CommitRange::builder_for`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-d558fffe48e24faa5c16db01): lists `_delta_log/` for the requested range.
- [`CommitRange::builder_from`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-b85017bbbf1059bb4f0f92d0): reuses an existing snapshot's `LogSegment`, avoiding the
  listing.

[`CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8) takes a list of [`DeltaAction`](../operations/buoyant_kernel.commit_range.actions.DeltaAction.md#op-ab52457262c777b7e2a7a5d8)
and an optional `start_snapshot`. When the snapshot is supplied, the iterator seeds its
`latest_protocol` / `latest_metadata` from it and the snapshot's version must match the
range's `start_version` (in ascending order) or `end_version` (in descending order). When
the snapshot is `None`, validation is purely commit-driven. See [`CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8)
for protocol-validation details.

# Example
```no_run
use std::sync::Arc;
use buoyant_kernel as delta_kernel;
use delta_kernel::commit_range::{CommitRange, DeltaAction};
use delta_kernel::{Engine, Error, Snapshot};
use delta_kernel::object_store::local::LocalFileSystem;
use test_utils::delta_kernel_default_engine::DefaultEngineBuilder;

let engine: Arc<dyn Engine> =
    Arc::new(DefaultEngineBuilder::new(Arc::new(LocalFileSystem::new())).build());
let start_snapshot = Snapshot::builder_for("file:///data/T").at_version(0).build(engine.as_ref())?;
let range = CommitRange::builder_for("file:///data/T", 0)
    .with_end_version(4)
    .build(engine.as_ref())?;

for commit in range.commits(
    engine.clone(),
    Some(start_snapshot),
    &[DeltaAction::Add, DeltaAction::Remove],
)? {
    let commit = commit?;
    println!("v={} ts={}", commit.version(), commit.timestamp());
    for batch in commit.get_actions(engine.as_ref())? {
        let _batch = batch?;
    }
}
# Ok::<(), Error>(())
```
