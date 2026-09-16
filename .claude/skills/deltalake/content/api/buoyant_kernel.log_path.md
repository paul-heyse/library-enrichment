# `buoyant_kernel::log_path`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.log_path.json`](../model/buoyant_kernel.log_path.json)

## LogPath

`struct` · `buoyant_kernel::log_path::LogPath`

Also reachable as `buoyant_kernel::LogPath`, `delta_kernel::log_path::LogPath`

```rust
struct LogPath
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn staged_commit(table_root: Url, filename: &str, last_modified: i64, size: FileSize) -> DeltaResult<LogPath>
fn staged_commit_url(table_root: Url, filename: &str) -> DeltaResult<Url>
fn try_new(file_meta: FileMeta) -> DeltaResult<Self>
```

A path to a valid delta log file. You can parse a given `FileMeta` into a `LogPath` using
[`LogPath::try_new`].

Today, a `LogPath` is a file in the `_delta_log` directory of a Delta table; in the future,
this will expand to support providing inline data in the log path itself.

---
