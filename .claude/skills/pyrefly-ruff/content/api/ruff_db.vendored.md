# `ruff_db::vendored`

Crate `ruff_db` · 5 public items · structured records in [`model/ruff_db.vendored.json`](../model/ruff_db.vendored.json)

## FileType

`enum` · `ruff_db::vendored::FileType`

```rust
enum FileType
```

**Variants**: `File`, `Directory`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_directory(self) -> bool
const fn is_file(self) -> bool
```

---

## DirectoryEntry

`struct` · `ruff_db::vendored::DirectoryEntry`

```rust
struct DirectoryEntry
```

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn file_type(&self) -> FileType
fn into_path(self) -> VendoredPathBuf
fn new(path: VendoredPathBuf, file_type: FileType) -> Self
fn path(&self) -> &VendoredPath
```

---

## Metadata

`struct` · `ruff_db::vendored::Metadata`

```rust
struct Metadata
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn kind(&self) -> FileType
fn revision(&self) -> FileRevision
```

---

## VendoredFileSystem

`struct` · `ruff_db::vendored::VendoredFileSystem`

```rust
struct VendoredFileSystem
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn exists(&self, path: impl AsRef<VendoredPath>) -> bool
fn is_directory(&self, path: impl AsRef<VendoredPath>) -> bool
fn is_file(&self, path: impl AsRef<VendoredPath>) -> bool
fn metadata(&self, path: impl AsRef<VendoredPath>) -> io::Result<Metadata>
fn new(raw_bytes: Vec<u8>) -> io::Result<Self>
fn new_static(raw_bytes: &'static [u8]) -> io::Result<Self>
fn read_directory(&self, dir: impl AsRef<VendoredPath>) -> impl Iterator<Item = DirectoryEntry> + '_
fn read_to_string(&self, path: impl AsRef<VendoredPath>) -> io::Result<String>
```

File system that stores all content in a static zip archive
bundled as part of the Ruff binary.

"Files" in the `VendoredFileSystem` are read-only and immutable.
Directories are supported, but symlinks and hardlinks cannot exist.

# Path separators

At time of writing (2025-07-11), this implementation always uses `/` as a
path separator, even in Windows environments where `\` is traditionally
used as a file path separator. Namely, this is only currently used with zip
files built by `crates/ty_vendored/build.rs`.

Callers using this may provide paths that use a `\` as a separator. It will
be transparently normalized to `/`.

This is particularly important because the presence of a trailing separator
in a zip file is conventionally used to indicate a directory entry.

---

## VendoredFileSystemBuilder

`struct` · `ruff_db::vendored::VendoredFileSystemBuilder`

```rust
struct VendoredFileSystemBuilder
```

**Methods** (4)

```rust
fn add_directory(&mut self, path: impl AsRef<VendoredPath>) -> ZipResult<()>
fn add_file(&mut self, path: impl AsRef<VendoredPath>, content: &str) -> std::io::Result<()>
fn finish(self) -> io::Result<VendoredFileSystem>
fn new(compression_method: CompressionMethod) -> Self
```

---
