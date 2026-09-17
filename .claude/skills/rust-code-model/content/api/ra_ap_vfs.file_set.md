# `ra_ap_vfs::file_set`

Crate `ra_ap_vfs` · 3 public items · structured records in [`model/ra_ap_vfs.file_set.json`](../model/ra_ap_vfs.file_set.json)

## FileSet

`struct` · `ra_ap_vfs::file_set::FileSet`

Also reachable as `ra_ap_base_db::FileSet`

```rust
struct FileSet
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn file_for_path(&self, path: &VfsPath) -> Option<&FileId>
fn insert(&mut self, file_id: FileId, path: VfsPath)
fn iter(&self) -> impl Iterator<Item = FileId> + '_
fn len(&self) -> usize
fn path_for_file(&self, file: &FileId) -> Option<&VfsPath>
fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId>
```

A set of [`VfsPath`]s identified by [`FileId`]s.

---

## FileSetConfig

`struct` · `ra_ap_vfs::file_set::FileSetConfig`

```rust
struct FileSetConfig
```

**Derives**: Debug, Default

**Methods** (4)

```rust
fn builder() -> FileSetConfigBuilder
fn classify_path(&self, path: &VfsPath) -> Option<usize>
fn partition(&self, vfs: &Vfs) -> Vec<FileSet>
fn roots(&self) -> Vec<(Vec<u8>, u64)>
```

This contains path prefixes to partition a [`Vfs`] into [`FileSet`]s.

# Example
```rust
# use vfs::{file_set::FileSetConfigBuilder, VfsPath, Vfs};
let mut builder = FileSetConfigBuilder::default();
builder.add_file_set(vec![VfsPath::new_virtual_path("/src".to_string())]);
let config = builder.build();
let mut file_system = Vfs::default();
file_system.set_file_contents(VfsPath::new_virtual_path("/src/main.rs".to_string()), Some(vec![]));
file_system.set_file_contents(VfsPath::new_virtual_path("/src/lib.rs".to_string()), Some(vec![]));
file_system.set_file_contents(VfsPath::new_virtual_path("/build.rs".to_string()), Some(vec![]));
// contains the sets :
// { "/src/main.rs", "/src/lib.rs" }
// { "build.rs" }
let sets = config.partition(&file_system);
```

---

## FileSetConfigBuilder

`struct` · `ra_ap_vfs::file_set::FileSetConfigBuilder`

```rust
struct FileSetConfigBuilder
```

**Derives**: Default

**Methods** (3)

```rust
fn add_file_set(&mut self, roots: Vec<VfsPath>)
fn build(self) -> FileSetConfig
fn len(&self) -> usize
```

Builder for [`FileSetConfig`].

---
