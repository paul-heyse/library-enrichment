# `pyrefly_util::arc_id`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.arc_id.json`](../model/pyrefly_util.arc_id.json)

## ArcId

`struct` · `pyrefly_util::arc_id::ArcId`

```rust
struct ArcId<T: ?Sized>
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::ops::deref::Deref`, `dupe::Dupe`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (4)

```rust
fn downgrade(&self) -> WeakArcId<T>
fn id(&self) -> usize
fn into_inner(self) -> Option<T>
fn new(id: T) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &T
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &T
```

An `Arc` where `Eq`, `Hash` and `Ord` are based on the pointer.
As a result, things like Hash/Ord might be unstable between multiple program runs.

---

## WeakArcId

`struct` · `pyrefly_util::arc_id::WeakArcId`

```rust
struct WeakArcId<T: ?Sized>
```

**Derives**: Clone, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn id(&self) -> usize
fn upgrade(&self) -> Option<ArcId<T>>
fn vacant(&self) -> bool
```

A [`Weak`] version of an [`ArcId`].
Hash and equality is based on the original `ArcId`'s id
without upgrading it.

---
