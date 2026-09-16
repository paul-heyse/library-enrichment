# `ruff_db::files::directory`

Crate `ruff_db` · 3 public items · structured records in [`model/ruff_db.files.directory.json`](../model/ruff_db.files.directory.json)

## directory_listing

`function` · `ruff_db::files::directory::directory_listing`

Also reachable as `ruff_db::files::directory_listing`

```rust
fn directory_listing<'db>(db: &'db dyn Db, path: &system::SystemPath) -> Result<&'db DirectoryListing, DirectoryListingError>
```

---

## DirectoryListing

`struct` · `ruff_db::files::directory::DirectoryListing`

Also reachable as `ruff_db::files::DirectoryListing`

```rust
struct DirectoryListing
```

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn contains_name_with_prefix(&self, prefix: &str) -> bool
fn entry_is_directory(&self, db: &dyn Db, directory: &SystemPath, name: &str) -> bool
fn entry_is_file(&self, db: &dyn Db, directory: &SystemPath, name: &str) -> bool
fn iter(&self) -> impl Iterator<Item = (&str, FileType)>
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A cached snapshot of the direct children in a directory.

The entries are sorted by name for efficient lookups.

---

## DirectoryListingError

`struct` · `ruff_db::files::directory::DirectoryListingError`

Also reachable as `ruff_db::files::DirectoryListingError`

```rust
struct DirectoryListingError
```

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(error: std::io::Error) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---
