# `pyrefly_types::type_var::flag`

Crate `pyrefly_types` · 2 public items · structured records in [`model/pyrefly_types.type_var.flag.json`](../model/pyrefly_types.type_var.flag.json)

## FlagMember

`enum` · `pyrefly_types::type_var::flag::FlagMember`

Also reachable as `pyrefly_types::type_var::FlagMember`

```rust
enum FlagMember
```

**Variants**: `Int`, `Bool`, `Str`, `IntTuple`, `NoneType`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn class_name(self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A member of the builtin universe a `Flag` type parameter can range over. The variant
order is the canonical order in which a domain materializes, names, and displays its
members, so every consumer sees the same sequence for the same domain.

---

## FlagDomain

`struct` · `pyrefly_types::type_var::flag::FlagDomain`

Also reachable as `pyrefly_types::type_var::FlagDomain`

```rust
struct FlagDomain
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (13)

```rust
fn accepts(self, ty: &Type) -> bool
fn accepts_literal(self, ty: &Type) -> bool
fn accepts_with_str_subclasses(self, ty: &Type, is_str_subclass: impl FnMut(&Type) -> bool) -> bool
fn as_type(self, stdlib: &Stdlib, heap: &TypeHeap) -> Type
fn class_names(self) -> Vec<&'static str>
fn contains(self, member: FlagMember) -> bool
fn fixed_tuple(arity: usize) -> Self
fn from_type(ty: &Type) -> Option<Self>
fn intersection(self, other: Self) -> Option<Self>
fn is_subset_of(self, other: Self) -> bool
fn join(self, other: Self) -> Self
fn of(member: FlagMember) -> Self
fn types(self, stdlib: &Stdlib) -> Vec<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

The set of builtin types a `Flag` type parameter ranges over.

Invariant: a domain is never empty. `of` and `join` are the only constructors and both
yield at least one member, so materialization may assume a member exists.

---
