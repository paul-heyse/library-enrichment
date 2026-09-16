# `pyrefly_types::annotation`

Crate `pyrefly_types` · 2 public items · structured records in [`model/pyrefly_types.annotation.json`](../model/pyrefly_types.annotation.json)

## Qualifier

`enum` · `pyrefly_types::annotation::Qualifier`

```rust
enum Qualifier
```

**Variants**: `Required`, `NotRequired`, `ReadOnly`, `ClassVar`, `Final`, `InitVar`, `Annotated`, `TypeAlias`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
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

---

## Annotation

`struct` · `pyrefly_types::annotation::Annotation`

```rust
struct Annotation
```

**Fields**: `qualifiers`, `ty`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn get_type(&self) -> &Type
fn has_qualifier(&self, qualifier: &Qualifier) -> bool
fn is_class_var(&self) -> bool
fn is_final(&self) -> bool
fn is_init_var(&self) -> bool
fn new_type(ty: Type) -> Self
fn substitute_with(self, substitution: Substitution<'_>) -> Self
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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---
