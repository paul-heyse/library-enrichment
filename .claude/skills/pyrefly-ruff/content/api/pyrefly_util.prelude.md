# `pyrefly_util::prelude`

Crate `pyrefly_util` · 3 public items · structured records in [`model/pyrefly_util.prelude.json`](../model/pyrefly_util.prelude.json)

## ResultExt

`trait` · `pyrefly_util::prelude::ResultExt`

```rust
trait ResultExt
```

**Implementors** (1)

- `core::result::Result`

**Methods** (1)

```rust
fn into_inner(self) -> Self::Item
```

---

## SliceExt

`trait` · `pyrefly_util::prelude::SliceExt`

```rust
trait SliceExt
```

**Methods** (2)

```rust
fn map<'a, B, F>(&'a self, f: F) -> Vec<B> where F: FnMut(&'a Self::Item) -> B
fn try_map<'a, B, E, F>(&'a self, f: F) -> Result<Vec<B>, E> where F: FnMut(&'a Self::Item) -> Result<B, E>
```

---

## VecExt

`trait` · `pyrefly_util::prelude::VecExt`

```rust
trait VecExt
```

**Implementors** (1)

- `alloc::vec::Vec`

**Methods** (2)

```rust
fn into_map<B, F>(self, f: F) -> Vec<B> where F: FnMut(Self::Item) -> B
fn into_try_map<B, E, F>(self, f: F) -> Result<Vec<B>, E> where F: FnMut(Self::Item) -> Result<B, E>
```

Extension traits on [`Vec`].

---
