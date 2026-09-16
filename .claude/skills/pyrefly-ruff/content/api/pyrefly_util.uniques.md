# `pyrefly_util::uniques`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.uniques.json`](../model/pyrefly_util.uniques.json)

## Unique

`struct` · `pyrefly_util::uniques::Unique`

```rust
struct Unique
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

A unique value, provided two given values were produced by the same factory.

---

## UniqueFactory

`struct` · `pyrefly_util::uniques::UniqueFactory`

```rust
struct UniqueFactory
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn fresh(&self) -> Unique
fn get_or_fresh(&self, key: usize) -> Unique
fn new() -> Self
```

Vend fresh unique `Var`s.
Deliberately NOT Clone.

---
