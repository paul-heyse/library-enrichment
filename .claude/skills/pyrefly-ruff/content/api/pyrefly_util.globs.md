# `pyrefly_util::globs`

Crate `pyrefly_util` · 5 public items · structured records in [`model/pyrefly_util.globs.json`](../model/pyrefly_util.globs.json)

## HiddenDirFilter

`enum` · `pyrefly_util::globs::HiddenDirFilter`

```rust
enum HiddenDirFilter
```

**Variants**: `Disabled`, `All`, `RelativeTo`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Controls whether paths with hidden directory components (names starting
with `.`, excluding `.` and `..`) are excluded during file filtering.

---

## FilteredGlobs

`struct` · `pyrefly_util::globs::FilteredGlobs`

```rust
struct FilteredGlobs
```

**Implements**: `pyrefly_util::includes::Includes`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn files_iter_with_limit(&self, limit: usize) -> anyhow::Result<Box<dyn Iterator<Item = PathBuf> + '_>>
fn new(includes: Globs, excludes: Globs, ignorefile_search_start: Option<&Path>, hidden_dir_filter: HiddenDirFilter) -> Self
```

**via `pyrefly_util::includes::Includes`**

```rust
fn covers(&self, path: &Path) -> bool
fn covers_ignoring_excludes(&self, path: &Path) -> bool
fn errors(&mut self) -> Vec<anyhow::Error>
fn files_iter(&self) -> anyhow::Result<Box<dyn Iterator<Item = PathBuf> + '_>>
fn roots(&self) -> Vec<PathBuf>
```

---

## Glob

`struct` · `pyrefly_util::globs::Glob`

```rust
struct Glob
```

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn as_path(&self) -> &Path
fn as_str(&self) -> &str
fn from_root(self, root: &Path) -> Self
fn matches(&self, file: &Path) -> bool
fn new(pattern: String) -> anyhow::Result<Self>
fn new_with_root(root: &Path, pattern: String) -> anyhow::Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

A glob pattern for matching files.
Patterns must use `/` as the path separator for cross-platform consistency.

Only matches Python files (.py, .pyi, .pyw) and automatically excludes:
- Files that don't have .py, .pyi, or .pyw extensions
- Files whose names start with '.' (dot files)

---

## GlobFilter

`struct` · `pyrefly_util::globs::GlobFilter`

```rust
struct GlobFilter
```

**Implements**: `core::fmt::Display`

**Derives**: Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn empty() -> Self
fn errors(&mut self) -> Vec<anyhow::Error>
fn ignore_files(root: &Path) -> (Vec<Gitignore>, Vec<anyhow::Error>, Vec<PathBuf>)
fn is_excluded(&self, path: &Path) -> bool
fn new(excludes: Globs, ignorefile_search_start: Option<&Path>, hidden_dir_filter: HiddenDirFilter) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A struct which allows filtering by matching a high-priority [`Globs`] of excludes
and several ignore files. The first positive (ignore) or negative (allowlist)
match that's found from the following order is what's used.
1. `excludes`: user-provided paths, either from a config or CLI.
2. `.gitignore`: if one exists from an upward search from `root`, the first
   positive or negative match (`!`) is used
3. `.ignore`: if it exists, behaves similar to `.gitignore`
4. `.git/info/excludes`: if it exists, behaves similar to `.gitignore`

---

## Globs

`struct` · `pyrefly_util::globs::Globs`

```rust
struct Globs
```

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn append(&mut self, patterns: &[Glob])
fn covers(&self, path: &Path) -> bool
fn empty() -> Self
fn files_iter(&self) -> anyhow::Result<Box<dyn Iterator<Item = PathBuf>>>
fn from_root(self, root: &Path) -> Self
fn globs(&self) -> &[Glob]
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(patterns: Vec<String>) -> anyhow::Result<Self>
fn new_with_root(root: &Path, patterns: Vec<String>) -> anyhow::Result<Self>
fn roots(&self) -> Vec<PathBuf>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
