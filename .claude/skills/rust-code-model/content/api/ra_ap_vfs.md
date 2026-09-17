# `ra_ap_vfs`

Crate `ra_ap_vfs` · 7 public items · structured records in [`model/ra_ap_vfs.json`](../model/ra_ap_vfs.json)

## Change

`enum` · `ra_ap_vfs::Change`

```rust
enum Change
```

**Variants**: `Create`, `Modify`, `Delete`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

Kind of [file change](ChangedFile).

---

## ChangeKind

`enum` · `ra_ap_vfs::ChangeKind`

```rust
enum ChangeKind
```

**Variants**: `Create`, `Modify`, `Delete`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Kind of [file change](ChangedFile).

---

## FileExcluded

`enum` · `ra_ap_vfs::FileExcluded`

```rust
enum FileExcluded
```

**Variants**: `Yes`, `No`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## FileState

`enum` · `ra_ap_vfs::FileState`

```rust
enum FileState
```

**Variants**: `Exists`, `Deleted`, `Excluded`

**Derives**: Clone, Copy, Debug, PartialEq, PartialOrd, StructuralPartialEq

---

## ChangedFile

`struct` · `ra_ap_vfs::ChangedFile`

```rust
struct ChangedFile
```

**Fields**: `file_id`, `change`

**Derives**: Debug

**Methods** (5)

```rust
fn exists(&self) -> bool
fn is_created(&self) -> bool
fn is_created_or_deleted(&self) -> bool
fn is_modified(&self) -> bool
fn kind(&self) -> ChangeKind
```

Changed file in the [`Vfs`].

---

## FileId

`struct` · `ra_ap_vfs::FileId`

Also reachable as `ra_ap_base_db::FileId`, `ra_ap_ide::FileId`, `ra_ap_span::FileId`

```rust
struct FileId
```

**Implements**: `core::convert::From`, `nohash_hasher::IsEnabled`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
const fn from_raw(raw: u32) -> FileId
const fn index(self) -> u32
```

Handle to a file in [`Vfs`]

Most functions in rust-analyzer use this when they need to refer to a file.

---

## Vfs

`struct` · `ra_ap_vfs::Vfs`

```rust
struct Vfs
```

**Derives**: Debug, Default

**Methods** (7)

```rust
fn exists(&self, file_id: FileId) -> bool
fn file_id(&self, path: &VfsPath) -> Option<(FileId, FileExcluded)>
fn file_path(&self, file_id: FileId) -> &VfsPath
fn insert_excluded_file(&mut self, path: VfsPath)
fn iter(&self) -> impl Iterator<Item = (FileId, &VfsPath)> + '_
fn set_file_contents(&mut self, path: VfsPath, contents: Option<Vec<u8>>) -> bool
fn take_changes(&mut self) -> IndexMap<FileId, ChangedFile, BuildHasherDefault<FxHasher>>
```

Storage for all file changes and the file id to path mapping.

For more information see the [crate-level](crate) documentation.

---
