# `ruff_db::files::path`

Crate `ruff_db` · 1 public items · structured records in [`model/ruff_db.files.path.json`](../model/ruff_db.files.path.json)

## FilePath

`enum` · `ruff_db::files::path::FilePath`

Also reachable as `ruff_db::files::FilePath`

```rust
enum FilePath
```

**Variants**: `System`, `SystemVirtual`, `Vendored`

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn as_str(&self) -> &str
fn as_system_path(&self) -> Option<&SystemPath>
fn as_vendored_path(&self) -> Option<&VendoredPath>
fn extension(&self) -> Option<&str>
const fn is_system_path(&self) -> bool
const fn is_system_virtual_path(&self) -> bool
const fn is_vendored_path(&self) -> bool
fn system(path: impl AsRef<SystemPath>) -> Self
fn to_file(&self, db: &dyn Db) -> Option<File>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(value: &SystemVirtualPath) -> Self
fn from(value: SystemVirtualPathBuf) -> Self
fn from(value: &VendoredPath) -> Self
fn from(value: &SystemPath) -> Self
fn from(value: VendoredPathBuf) -> Self
fn from(value: SystemPathBuf) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Path to a file.

The path abstracts that files in Ruff can come from different sources:

* a file stored on the [host system](crate::system::System).
* a virtual file stored on the [host system](crate::system::System).
* a vendored file stored in the [vendored file system](crate::vendored::VendoredFileSystem).

---
