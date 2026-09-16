# `ruff_db::file_revision`

Crate `ruff_db` · 1 public items · structured records in [`model/ruff_db.file_revision.json`](../model/ruff_db.file_revision.json)

## FileRevision

`struct` · `ruff_db::file_revision::FileRevision`

```rust
struct FileRevision
```

**Implements**: `core::convert::From`, `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(value: u128) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: u64) -> Self
fn from(value: filetime::FileTime) -> Self
fn from(value: u128) -> Self
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A number representing the revision of a file.

Two revisions that don't compare equal signify that the file has been modified.
Revisions aren't guaranteed to be monotonically increasing or in any specific order.

Possible revisions are:
* The last modification time of the file.
* The hash of the file's content.
* The revision as it comes from an external system, for example the LSP.

---
