# `ra_ap_vfs::vfs_path`

Crate `ra_ap_vfs` · 1 public items · structured records in [`model/ra_ap_vfs.vfs_path.json`](../model/ra_ap_vfs.vfs_path.json)

## VfsPath

`struct` · `ra_ap_vfs::vfs_path::VfsPath`

Also reachable as `ra_ap_base_db::VfsPath`, `ra_ap_vfs::VfsPath`

```rust
struct VfsPath
```

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (10)

```rust
fn as_path(&self) -> Option<&AbsPath>
fn into_abs_path(self) -> Option<AbsPathBuf>
fn join(&self, path: &str) -> Option<VfsPath>
fn name_and_extension(&self) -> Option<(&str, Option<&str>)>
fn new_real_path(path: String) -> VfsPath
fn new_virtual_path(path: String) -> VfsPath
fn parent(&self) -> Option<VfsPath>
fn pop(&mut self) -> bool
fn starts_with(&self, other: &VfsPath) -> bool
fn strip_prefix(&self, other: &VfsPath) -> Option<&RelPath>
```

**via `core::convert::From`**

```rust
fn from(v: AbsPathBuf) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Path in [`Vfs`].

Long-term, we want to support files which do not reside in the file-system,
so we treat `VfsPath`s as opaque identifiers.

[`Vfs`]: crate::Vfs

---
