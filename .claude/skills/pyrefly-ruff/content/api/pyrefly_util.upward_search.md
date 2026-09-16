# `pyrefly_util::upward_search`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.upward_search.json`](../model/pyrefly_util.upward_search.json)

## FileGroup

`struct` · `pyrefly_util::upward_search::FileGroup`

```rust
struct FileGroup<T>
```

**Methods** (2)

```rust
fn new(filenames: Vec<OsString>, predicate: impl Fn(&T) -> bool + Send + Sync + 'static) -> Self
fn new_simple(filenames: Vec<OsString>) -> Self
```

A single filegroup used in an upward search, paired with a predicate on whether
the loaded result should be used or the search should continue. This is especially
useful when you have one set of filenames that would be preferred, but require
introspection on whether the result is actually useful.

For example, whether a `pyproject.toml` should be considered a configuration file
(has a `[tool.pyrefly]` section) or marker file (is probably the root of a project).

---

## UpwardSearch

`struct` · `pyrefly_util::upward_search::UpwardSearch`

```rust
struct UpwardSearch<T>
```

**Methods** (5)

```rust
fn clear(&self)
fn directory(&self, dir: &Path) -> Option<T>
fn directory_absolute(&self, dir: &Path) -> Option<T>
fn new(filenames: Vec<OsString>, load: impl Fn(&Path) -> T + Send + Sync + 'static) -> Self
fn new_grouped(filegroups: Vec<FileGroup<T>>, load: impl Fn(&Path) -> T + Send + Sync + 'static) -> Self
```

A cached way to search the file system upwards for specific files.

---
