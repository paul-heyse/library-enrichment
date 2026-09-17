# `walkdir::dent`

Crate `walkdir` · 2 public items · structured records in [`model/walkdir.dent.json`](../model/walkdir.dent.json)

## DirEntry

`struct` · `walkdir::dent::DirEntry`

Also reachable as `walkdir::DirEntry`

```rust
struct DirEntry
```

**Implements**: `walkdir::dent::DirEntryExt`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn depth(&self) -> usize
fn file_name(&self) -> &OsStr
fn file_type(&self) -> fs::FileType
fn into_path(self) -> PathBuf
fn metadata(&self) -> Result<fs::Metadata>
fn path(&self) -> &Path
fn path_is_symlink(&self) -> bool
```

**via `walkdir::dent::DirEntryExt`**

```rust
fn ino(&self) -> u64
```

A directory entry.

This is the type of value that is yielded from the iterators defined in
this crate.

On Unix systems, this type implements the [`DirEntryExt`] trait, which
provides efficient access to the inode number of the directory entry.

# Differences with `std::fs::DirEntry`

This type mostly mirrors the type by the same name in [`std::fs`]. There
are some differences however:

* All recursive directory iterators must inspect the entry's type.
Therefore, the value is stored and its access is guaranteed to be cheap and
successful.
* [`path`] and [`file_name`] return borrowed variants.
* If [`follow_links`] was enabled on the originating iterator, then all
operations except for [`path`] operate on the link target. Otherwise, all
operations operate on the symbolic link.

[`std::fs`]: https://doc.rust-lang.org/stable/std/fs/index.html
[`path`]: #method.path
[`file_name`]: #method.file_name
[`follow_links`]: struct.WalkDir.html#method.follow_links
[`DirEntryExt`]: trait.DirEntryExt.html

---

## DirEntryExt

`trait` · `walkdir::dent::DirEntryExt`

Also reachable as `walkdir::DirEntryExt`

```rust
trait DirEntryExt
```

**Implementors** (1)

- `walkdir::dent::DirEntry`

**Methods** (1)

```rust
fn ino(&self) -> u64
```

Unix-specific extension methods for `walkdir::DirEntry`

---
