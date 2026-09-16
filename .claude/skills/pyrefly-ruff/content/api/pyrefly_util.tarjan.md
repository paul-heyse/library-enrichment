# `pyrefly_util::tarjan`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.tarjan.json`](../model/pyrefly_util.tarjan.json)

## SccIdx

`struct` · `pyrefly_util::tarjan::SccIdx`

```rust
struct SccIdx
```

**Implements**: `core::convert::From`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `core::ops::arith::Rem`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `index_vec::Idx`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (9)

```rust
fn check_index(v: usize)
fn from_foreign<F: Idx>(value: F) -> Self
fn from_raw(value: u32) -> Self
const fn from_raw_unchecked(raw: u32) -> Self
fn from_usize(value: usize) -> Self
const fn from_usize_unchecked(value: usize) -> Self
const fn index(self) -> usize
fn new(value: usize) -> Self
const fn raw(self) -> u32
```

**via `core::convert::From`**

```rust
fn from(value: usize) -> Self
```

**via `core::ops::arith::Add`**

```rust
fn add(self, other: SccIdx) -> SccIdx
fn add(self, other: usize) -> Self
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, other: usize)
fn add_assign(&mut self, other: SccIdx)
```

**via `core::ops::arith::Rem`**

```rust
fn rem(self, other: usize) -> Self
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: usize) -> Self
fn sub(self, other: SccIdx) -> SccIdx
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: SccIdx)
fn sub_assign(&mut self, other: usize)
```

**via `index_vec::Idx`**

```rust
fn from_usize(value: usize) -> Self
fn index(self) -> usize
```

---

## Tarjan

`struct` · `pyrefly_util::tarjan::Tarjan`

```rust
struct Tarjan<T: Eq + Hash>
```

**Methods** (3)

```rust
fn iter_scc(&self, scc: SccIdx) -> impl Iterator<Item = &T>
fn new() -> Self
fn root(&mut self, root_key: T, visit: &impl Fn(&T, &mut dyn FnMut(T))) -> SccIdx
```

---
