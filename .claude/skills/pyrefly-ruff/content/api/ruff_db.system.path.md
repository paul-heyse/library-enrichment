# `ruff_db::system::path`

Crate `ruff_db` · 6 public items · structured records in [`model/ruff_db.system.path.json`](../model/ruff_db.system.path.json)

## deduplicate_nested_paths

`function` · `ruff_db::system::path::deduplicate_nested_paths`

Also reachable as `ruff_db::system::deduplicate_nested_paths`

```rust
fn deduplicate_nested_paths<P, I>(paths: I) -> DeduplicatedNestedPathsIter<P> where I: IntoIterator<Item = P>, P: AsRef<SystemPath>
```

Deduplicates identical paths and removes nested paths.

# Examples
```rust
use ruff_db::system::{SystemPath, deduplicate_nested_paths};///

let paths = vec![SystemPath::new("/a/b/c"), SystemPath::new("/a/b"), SystemPath::new("/a/beta"), SystemPath::new("/a/b/c")];
assert_eq!(deduplicate_nested_paths(paths).collect::<Vec<_>>(), &[SystemPath::new("/a/b"), SystemPath::new("/a/beta")]);
```

---

## DeduplicatedNestedPathsIter

`struct` · `ruff_db::system::path::DeduplicatedNestedPathsIter`

Also reachable as `ruff_db::system::DeduplicatedNestedPathsIter`

```rust
struct DeduplicatedNestedPathsIter<P>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

---

## SystemPath

`struct` · `ruff_db::system::path::SystemPath`

Also reachable as `ruff_db::system::SystemPath`

```rust
struct SystemPath
```

**Implements**: `alloc::borrow::ToOwned`, `core::convert::AsRef`, `core::fmt::Display`

**Derives**: Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (20)

```rust
fn absolute(path: impl AsRef<SystemPath>, cwd: impl AsRef<SystemPath>) -> SystemPathBuf
fn ancestors(&self) -> impl Iterator<Item = &SystemPath>
fn as_std_path(&self) -> &Path
fn as_str(&self) -> &str
fn as_utf8_path(&self) -> &Utf8Path
fn components(&self) -> camino::Utf8Components<'_>
fn ends_with(&self, child: impl AsRef<SystemPath>) -> bool
fn extension(&self) -> Option<&str>
fn file_name(&self) -> Option<&str>
fn file_stem(&self) -> Option<&str>
fn from_std_path(path: &Path) -> Option<&SystemPath>
fn is_absolute(&self) -> bool
fn join(&self, path: impl AsRef<SystemPath>) -> SystemPathBuf
fn new(path: &impl AsRef<Utf8Path> + ?Sized) -> &Self
fn parent(&self) -> Option<&SystemPath>
fn simplified(&self) -> &SystemPath
fn starts_with(&self, base: impl AsRef<SystemPath>) -> bool
fn strip_prefix(&self, base: impl AsRef<SystemPath>) -> std::result::Result<&SystemPath, StripPrefixError>
fn to_path_buf(&self) -> SystemPathBuf
fn with_extension(&self, extension: &str) -> SystemPathBuf
```

**via `alloc::borrow::ToOwned`**

```rust
fn to_owned(&self) -> Self::Owned
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &SystemPath
fn as_ref(&self) -> &Path
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

A slice of a path on [`System`](super::System) (akin to [`str`]).

The path is guaranteed to be valid UTF-8.

---

## SystemPathBuf

`struct` · `ruff_db::system::path::SystemPathBuf`

Also reachable as `ruff_db::system::SystemPathBuf`

```rust
struct SystemPathBuf
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `get_size2::GetSize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (9)

```rust
fn as_path(&self) -> &SystemPath
fn from_path_buf(path: std::path::PathBuf) -> std::result::Result<Self, std::path::PathBuf>
fn from_path_buf_lossy(path: std::path::PathBuf) -> Self
fn from_utf8_path_buf(path: Utf8PathBuf) -> Self
fn into_std_path_buf(self) -> PathBuf
fn into_string(self) -> String
fn into_utf8_path_buf(self) -> Utf8PathBuf
fn new() -> Self
fn push(&mut self, path: impl AsRef<SystemPath>)
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &SystemPath
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &SystemPath
fn as_ref(&self) -> &Path
```

**via `core::convert::From`**

```rust
fn from(value: &str) -> Self
fn from(value: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size_with_tracker<T: get_size2::GetSizeTracker>(&self, tracker: T) -> (usize, T)
```

An owned, mutable path on [`System`](`super::System`) (akin to [`String`]).

The path is guaranteed to be valid UTF-8.

---

## SystemVirtualPath

`struct` · `ruff_db::system::path::SystemVirtualPath`

Also reachable as `ruff_db::system::SystemVirtualPath`

```rust
struct SystemVirtualPath
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`

**Derives**: Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str(&self) -> &str
fn extension(&self) -> Option<&str>
const fn new(path: &str) -> &SystemVirtualPath
fn to_path_buf(&self) -> SystemVirtualPathBuf
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &SystemVirtualPath
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

A slice of a virtual path on [`System`](super::System) (akin to [`str`]).

---

## SystemVirtualPathBuf

`struct` · `ruff_db::system::path::SystemVirtualPathBuf`

Also reachable as `ruff_db::system::SystemVirtualPathBuf`

```rust
struct SystemVirtualPathBuf
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::ops::deref::Deref`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
const fn as_path(&self) -> &SystemVirtualPath
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &SystemVirtualPath
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &SystemVirtualPath
```

**via `core::convert::From`**

```rust
fn from(value: String) -> Self
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
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

An owned, virtual path on [`System`](`super::System`) (akin to [`String`]).

---
