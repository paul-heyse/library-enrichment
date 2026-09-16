# `pyrefly::alt::types::abstract_class`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.alt.types.abstract_class.json`](../model/pyrefly.alt.types.abstract_class.json)

## AbstractClassMembers

`struct` · `pyrefly::alt::types::abstract_class::AbstractClassMembers`

```rust
struct AbstractClassMembers
```

**Fields**: `unimplemented_abstract_methods`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(unimplemented_abstract_methods: SmallSet<Name>) -> AbstractClassMembers
fn recursive() -> &'static Self
fn unimplemented_abstract_methods(&self) -> &SmallSet<Name>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
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

---
