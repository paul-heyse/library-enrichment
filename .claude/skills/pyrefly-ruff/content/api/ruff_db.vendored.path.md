# `ruff_db::vendored::path`

Crate `ruff_db` · 2 public items · structured records in [`model/ruff_db.vendored.path.json`](../model/ruff_db.vendored.path.json)

## VendoredPath

`struct` · `ruff_db::vendored::path::VendoredPath`

Also reachable as `ruff_db::vendored::VendoredPath`

```rust
struct VendoredPath
```

**Implements**: `alloc::borrow::ToOwned`, `core::convert::AsRef`, `core::fmt::Display`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (14)

```rust
fn as_std_path(&self) -> &path::Path
fn as_str(&self) -> &str
fn as_utf8_path(&self) -> &camino::Utf8Path
fn components(&self) -> Utf8Components<'_>
fn ends_with(&self, suffix: impl AsRef<VendoredPath>) -> bool
fn extension(&self) -> Option<&str>
fn file_name(&self) -> Option<&str>
fn file_stem(&self) -> Option<&str>
fn join(&self, other: impl AsRef<VendoredPath>) -> VendoredPathBuf
fn new(path: &impl AsRef<Utf8Path> + ?Sized) -> &Self
fn parent(&self) -> Option<&Self>
fn strip_prefix(&self, prefix: impl AsRef<VendoredPath>) -> Result<&Self, path::StripPrefixError>
fn to_path_buf(&self) -> VendoredPathBuf
fn with_pyi_extension(&self) -> VendoredPathBuf
```

**via `alloc::borrow::ToOwned`**

```rust
fn to_owned(&self) -> VendoredPathBuf
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &path::Path
fn as_ref(&self) -> &VendoredPath
fn as_ref(&self) -> &Utf8Path
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

---

## VendoredPathBuf

`struct` · `ruff_db::vendored::path::VendoredPathBuf`

Also reachable as `ruff_db::vendored::VendoredPathBuf`

```rust
struct VendoredPathBuf
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::ops::deref::Deref`, `get_size2::GetSize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_path(&self) -> &VendoredPath
fn new() -> Self
fn push(&mut self, component: impl AsRef<VendoredPath>)
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &VendoredPath
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &VendoredPath
```

**via `core::convert::From`**

```rust
fn from(value: &str) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: path::PathBuf) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size_with_tracker<T: get_size2::GetSizeTracker>(&self, tracker: T) -> (usize, T)
```

---
