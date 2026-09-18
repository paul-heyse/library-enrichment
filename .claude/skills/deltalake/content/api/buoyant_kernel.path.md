# `buoyant_kernel::path`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.path.json`](../model/buoyant_kernel.path.json)

## LogPathFileType

`enum` · `buoyant_kernel::path::LogPathFileType`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.path.LogPathFileType.md)

Also reachable as `delta_kernel::path::LogPathFileType`

```rust
enum LogPathFileType
```

**Variants**: `Commit`, `StagedCommit`, `SinglePartCheckpoint`, `UuidCheckpoint`, `MultiPartCheckpoint`, `CompactedCommit`, `Crc`, `Unknown`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## ParsedLogPath

`struct` · `buoyant_kernel::path::ParsedLogPath`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.path.ParsedLogPath.md)

Also reachable as `delta_kernel::path::ParsedLogPath`

```rust
struct ParsedLogPath<Location: AsUrl = FileMeta>
```

**Fields**: `location`, `filename`, `extension`, `version`, `file_type`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn is_checkpoint(&self) -> bool
fn is_commit(&self) -> bool
fn is_unknown(&self) -> bool
fn new_crc(table_root: &Url, version: Version) -> DeltaResult<Self>
fn try_from(location: Location) -> DeltaResult<Option<ParsedLogPath<Location>>>
```

**via `core::convert::From`**

```rust
fn from(p: LogPath) -> Self
```

A ParsedLogPath is a well-understood path to a file in the _delta_log directory.

Note this includes things like checkpoints and commits (containing current table state), but
also files used for various optimizations like CRC, compaction, etc.

Every parsed log path has a version. And additionally, we implement a 'should_list' method
which controls whether or not we include this file in our listing. For example, when we list
the _delta_log we may see _staged_commits/00000000000000000000.{uuid}.json, but we MUST NOT
include those in listing, as only the catalog can tell us which are valid commits.

---

## AsUrl

`trait` · `buoyant_kernel::path::AsUrl`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.path.AsUrl.md)

Also reachable as `delta_kernel::path::AsUrl`

```rust
trait AsUrl
```

**Implementors** (2)

- `buoyant_kernel::FileMeta`
- `url::Url`

**Methods** (1)

```rust
fn as_url(&self) -> &Url
```

---
