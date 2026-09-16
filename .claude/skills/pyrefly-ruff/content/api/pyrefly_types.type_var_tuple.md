# `pyrefly_types::type_var_tuple`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.type_var_tuple.json`](../model/pyrefly_types.type_var_tuple.json)

## TypeVarTuple

`struct` · `pyrefly_types::type_var_tuple::TypeVarTuple`

```rust
struct TypeVarTuple
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn default(&self) -> Option<&Type>
fn new(name: Identifier, module: Module, default: Option<Type>) -> Self
fn qname(&self) -> &QName
fn to_type(&self, heap: &TypeHeap) -> Type
fn type_eq_inner(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool
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
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

Used to represent TypeVarTuple calls. Each TypeVarTuple is unique, so use the ArcId to separate them.

---
