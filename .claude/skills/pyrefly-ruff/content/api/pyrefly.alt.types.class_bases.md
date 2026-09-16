# `pyrefly::alt::types::class_bases`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.alt.types.class_bases.json`](../model/pyrefly.alt.types.class_bases.json)

## ClassBases

`struct` · `pyrefly::alt::types::class_bases::ClassBases`

```rust
struct ClassBases
```

**Fields**: `has_pydantic_strict_metadata`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn base_type_count(&self) -> usize
fn is_empty(&self) -> bool
fn iter(&self) -> impl Iterator<Item = &ClassType>
fn iter_with_ranges(&self) -> impl Iterator<Item = (&ClassType, TextRange)>
fn recursive() -> &'static Self
fn tuple_ancestor(&self) -> Option<&Tuple>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Information computed from the full base types of a class
- The base types themselves
- Some additional metadata derived from the bases

This is intended to be used for any downstream computation that needs to inspect the full types
(in particular, the targs of generic bases) of the bases of a class. If only the class objects are
needed, query `ClassMetadata` instead since that one doesn't require calculating the full types.

The reason this is tracked separately from `ClassMetadata` is to avoid the possibility of
cycles when type arguments of the base classes may depend on the class itself.

---
