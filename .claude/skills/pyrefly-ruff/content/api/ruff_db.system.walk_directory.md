# `ruff_db::system::walk_directory`

Crate `ruff_db` · 10 public items · structured records in [`model/ruff_db.system.walk_directory.json`](../model/ruff_db.system.walk_directory.json)

## ErrorKind

`enum` · `ruff_db::system::walk_directory::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `Loop`, `Io`, `NonUtf8Path`

**Derives**: Debug

---

## WalkState

`enum` · `ruff_db::system::walk_directory::WalkState`

```rust
enum WalkState
```

**Variants**: `Continue`, `Skip`, `Quit`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## DirectoryEntry

`struct` · `ruff_db::system::walk_directory::DirectoryEntry`

```rust
struct DirectoryEntry
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn depth(&self) -> usize
fn file_type(&self) -> FileType
fn into_path(self) -> SystemPathBuf
fn path(&self) -> &SystemPath
```

An entry in a directory.

---

## Error

`struct` · `ruff_db::system::walk_directory::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn depth(&self) -> Option<usize>
fn kind(&self) -> &ErrorKind
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

---

## WalkDirectoryBuilder

`struct` · `ruff_db::system::walk_directory::WalkDirectoryBuilder`

```rust
struct WalkDirectoryBuilder
```

**Methods** (7)

```rust
fn add(self, path: impl AsRef<SystemPath>) -> Self
fn ignore_hidden(self, hidden: bool) -> Self
fn incremental_matcher(self) -> Box<dyn IgnoreIncremental>
fn new<W>(path: impl AsRef<SystemPath>, walker: W) -> Self where W: DirectoryWalker + 'static
fn run<'s, F>(self, builder: F) where F: FnMut() -> Box<dyn FnMut(std::result::Result<DirectoryEntry, Error>) -> WalkState + Send + 's>
fn standard_filters(self, standard_filters: bool) -> Self
fn visit(self, builder: &mut dyn WalkDirectoryVisitorBuilder<'_>)
```

A builder for constructing a directory recursive traversal.

---

## WalkDirectoryConfiguration

`struct` · `ruff_db::system::walk_directory::WalkDirectoryConfiguration`

```rust
struct WalkDirectoryConfiguration
```

**Fields**: `paths`, `ignore_hidden`, `standard_filters`

---

## DirectoryWalker

`trait` · `ruff_db::system::walk_directory::DirectoryWalker`

```rust
trait DirectoryWalker
```

**Methods** (2)

```rust
fn incremental_matcher(&self, configuration: WalkDirectoryConfiguration) -> Box<dyn IgnoreIncremental>
fn walk(&self, builder: &mut dyn WalkDirectoryVisitorBuilder<'_>, configuration: WalkDirectoryConfiguration)
```

Concrete walker that performs the directory walking.

---

## IgnoreIncremental

`trait` · `ruff_db::system::walk_directory::IgnoreIncremental`

```rust
trait IgnoreIncremental
```

**Methods** (1)

```rust
fn is_ignored(&mut self, path: &SystemPath, is_directory: bool) -> bool
```

A matcher for determining whether paths are ignored during incremental directory walking.

---

## WalkDirectoryVisitor

`trait` · `ruff_db::system::walk_directory::WalkDirectoryVisitor`

```rust
trait WalkDirectoryVisitor: Send
```

**Methods** (1)

```rust
fn visit(&mut self, entry: std::result::Result<DirectoryEntry, Error>) -> WalkState
```

Visitor handling the individual directory entries.

---

## WalkDirectoryVisitorBuilder

`trait` · `ruff_db::system::walk_directory::WalkDirectoryVisitorBuilder`

```rust
trait WalkDirectoryVisitorBuilder<'s>
```

**Methods** (1)

```rust
fn build(&mut self) -> Box<dyn WalkDirectoryVisitor + 's>
```

Creates a visitor for each thread that does the visiting.

---
