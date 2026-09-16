# `pyrefly_types::type_var::shape_extension`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.type_var.shape_extension.json`](../model/pyrefly_types.type_var.shape_extension.json)

## ShapeExtensionRestriction

`struct` · `pyrefly_types::type_var::shape_extension::ShapeExtensionRestriction`

Also reachable as `pyrefly_types::type_var::ShapeExtensionRestriction`

```rust
struct ShapeExtensionRestriction
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn accepts_specialization(&self, ty: &Type, is_str_subclass: impl FnMut(&Type) -> bool) -> bool
fn infer_default_as_value(&self) -> bool
fn kind_name(&self) -> &'static str
fn upper_bound(&self, stdlib: &Stdlib, heap: &TypeHeap) -> Type
fn upper_bound_class_names(&self) -> Vec<&'static str>
fn upper_bound_members(&self, stdlib: &Stdlib) -> Vec<Type>
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

A type-parameter restriction whose inference policy belongs to the experimental shape system.

---
