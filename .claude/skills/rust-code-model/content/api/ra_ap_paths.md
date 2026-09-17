# `ra_ap_paths`

Crate `ra_ap_paths` · 4 public items · structured records in [`model/ra_ap_paths.json`](../model/ra_ap_paths.json)

## AbsPath

`struct` · `ra_ap_paths::AbsPath`

Also reachable as `ra_ap_vfs::AbsPath`

```rust
struct AbsPath
```

**Implements**: `alloc::borrow::ToOwned`, `core::convert::AsRef`, `core::fmt::Display`

**Derives**: Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (19)

```rust
fn absolutize(&self, path: impl AsRef<Utf8Path>) -> AbsPathBuf
fn as_os_str(&self) -> &OsStr
fn as_str(&self) -> &str
fn assert(path: &Utf8Path) -> &AbsPath
fn canonicalize(&self) -> never
fn components(&self) -> Utf8Components<'_>
fn display(&self) -> never
fn ends_with(&self, suffix: &RelPath) -> bool
fn exists(&self) -> never
fn extension(&self) -> Option<&str>
fn file_name(&self) -> Option<&str>
fn file_stem(&self) -> Option<&str>
fn join(&self, path: impl AsRef<Utf8Path>) -> AbsPathBuf
fn name_and_extension(&self) -> Option<(&str, Option<&str>)>
fn normalize(&self) -> AbsPathBuf
fn parent(&self) -> Option<&AbsPath>
fn starts_with(&self, base: &AbsPath) -> bool
fn strip_prefix(&self, base: &AbsPath) -> Option<&RelPath>
fn to_path_buf(&self) -> AbsPathBuf
```

**via `alloc::borrow::ToOwned`**

```rust
fn to_owned(&self) -> Self::Owned
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &OsStr
fn as_ref(&self) -> &Path
fn as_ref(&self) -> &Utf8Path
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Wrapper around an absolute [`Utf8Path`].

---

## AbsPathBuf

`struct` · `ra_ap_paths::AbsPathBuf`

Also reachable as `ra_ap_base_db::AbsPathBuf`, `ra_ap_vfs::AbsPathBuf`

```rust
struct AbsPathBuf
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (6)

```rust
fn as_path(&self) -> &AbsPath
fn assert(path: Utf8PathBuf) -> AbsPathBuf
fn assert_utf8(path: PathBuf) -> AbsPathBuf
fn join(&self, path: impl AsRef<Utf8Path>) -> Self
fn pop(&mut self) -> bool
fn push<P: AsRef<Utf8Path>>(&mut self, suffix: P)
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &AbsPath
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &AbsPath
fn as_ref(&self) -> &Path
fn as_ref(&self) -> &OsStr
fn as_ref(&self) -> &Utf8Path
```

**via `core::convert::TryFrom`**

```rust
fn try_from(path_buf: Utf8PathBuf) -> Result<AbsPathBuf, Utf8PathBuf>
fn try_from(path: &str) -> Result<AbsPathBuf, Utf8PathBuf>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &AbsPath
```

A [`Utf8PathBuf`] that is guaranteed to be absolute.

---

## RelPath

`struct` · `ra_ap_paths::RelPath`

```rust
struct RelPath
```

**Implements**: `core::convert::AsRef`

**Derives**: Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str(&self) -> &str
fn as_utf8_path(&self) -> &Utf8Path
fn new_unchecked(path: &Utf8Path) -> &RelPath
fn to_path_buf(&self) -> RelPathBuf
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Path
fn as_ref(&self) -> &Utf8Path
```

Wrapper around a relative [`Utf8Path`].

---

## RelPathBuf

`struct` · `ra_ap_paths::RelPathBuf`

```rust
struct RelPathBuf
```

**Implements**: `core::convert::AsRef`, `core::convert::TryFrom`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_path(&self) -> &RelPath
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Path
fn as_ref(&self) -> &Utf8Path
```

**via `core::convert::TryFrom`**

```rust
fn try_from(path: &str) -> Result<RelPathBuf, Utf8PathBuf>
fn try_from(path_buf: Utf8PathBuf) -> Result<RelPathBuf, Utf8PathBuf>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &RelPath
```

Wrapper around a relative [`Utf8PathBuf`].

---
