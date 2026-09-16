# `buoyant_kernel::snapshot::builder`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.snapshot.builder.json`](../model/buoyant_kernel.snapshot.builder.json)

## IncrementalReplay

`enum` · `buoyant_kernel::snapshot::builder::IncrementalReplay`

Also reachable as `buoyant_kernel::snapshot::IncrementalReplay`, `delta_kernel::snapshot::builder::IncrementalReplay`

```rust
enum IncrementalReplay
```

**Variants**: `Disabled`, `UpToCommits`, `Unlimited`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Controls whether kernel replays commits to advance a stale base CRC (the existing snapshot's
in-memory CRC, or an on-disk CRC) to the target snapshot version on load. A CRC already at the
target version is always used regardless of this setting; this only bounds the cost of
advancing a *stale* CRC.

A resolved CRC gives the snapshot precomputed file statistics (file count and sizes, useful
for query optimization and for writers producing a post-commit CRC) along with domain metadata
and set transactions (useful for writers), all without extra log replay.

---

## SnapshotBuilder

`struct` · `buoyant_kernel::snapshot::builder::SnapshotBuilder`

Also reachable as `buoyant_kernel::snapshot::SnapshotBuilder`, `delta_kernel::snapshot::builder::SnapshotBuilder`

```rust
struct SnapshotBuilder
```

**Derives**: Debug

**Methods** (6)

```rust
fn at_version(self, version: Version) -> Self
fn build(self, engine: &dyn Engine) -> DeltaResult<SnapshotRef>
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
fn with_incremental_crc_replay(self, mode: IncrementalReplay) -> Self
fn with_log_tail(self, log_tail: Vec<LogPath>) -> Self
fn with_max_catalog_version(self, max_catalog_version: Version) -> Self
```

Builder for creating [`Snapshot`] instances.

# Example

```no_run
# use buoyant_kernel as delta_kernel;
# use delta_kernel::{Snapshot, Engine};
# use url::Url;
# fn example(engine: &dyn Engine) -> delta_kernel::DeltaResult<()> {
let table_root = Url::parse("file:///path/to/table")?;

// Build a snapshot
let snapshot = Snapshot::builder_for(table_root.clone())
    .at_version(5) // Optional: specify a time-travel version (default is latest version)
    .build(engine)?;

# Ok(())
# }
```

---
