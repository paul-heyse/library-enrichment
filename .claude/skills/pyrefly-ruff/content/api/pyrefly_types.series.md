# `pyrefly_types::series`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.series.json`](../model/pyrefly_types.series.json)

## SeriesSchema

`struct` · `pyrefly_types::series::SeriesSchema`

```rust
struct SeriesSchema
```

**Fields**: `underlying`, `dtype`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn to_type(self) -> Type
fn underlying_type(&self) -> Type
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

A Series instance carrying its element dtype.

---
