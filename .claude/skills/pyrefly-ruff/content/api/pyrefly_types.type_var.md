# `pyrefly_types::type_var`

Crate `pyrefly_types` · 4 public items · structured records in [`model/pyrefly_types.type_var.json`](../model/pyrefly_types.type_var.json)

## PreInferenceVariance

`enum` · `pyrefly_types::type_var::PreInferenceVariance`

```rust
enum PreInferenceVariance
```

**Variants**: `Covariant`, `Contravariant`, `Invariant`, `Undefined`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

## Restriction

`enum` · `pyrefly_types::type_var::Restriction`

```rust
enum Restriction
```

**Variants**: `Constraints`, `Bound`, `ShapeExtension`, `Unrestricted`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn flag(domain: FlagDomain) -> Self
fn flag_domain(&self) -> Option<FlagDomain>
fn index() -> Self
fn is_flag(&self) -> bool
fn is_index(&self) -> bool
fn is_restricted(&self) -> bool
fn uses_direct_value_source(&self) -> bool
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

## Variance

`enum` · `pyrefly_types::type_var::Variance`

```rust
enum Variance
```

**Variants**: `Covariant`, `Contravariant`, `Invariant`, `Bivariant`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn compose(self, other: Variance) -> Variance
fn inv(self) -> Variance
fn union(self, other: Variance) -> Variance
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

## TypeVar

`struct` · `pyrefly_types::type_var::TypeVar`

```rust
struct TypeVar
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (10)

```rust
fn default(&self) -> Option<&Type>
fn kind(&self) -> QuantifiedKind
fn new(name: Identifier, module: Module, restriction: Restriction, default: Option<Type>, variance: PreInferenceVariance) -> Self
fn new_with_kind(name: Identifier, module: Module, kind: QuantifiedKind, restriction: Restriction, default: Option<Type>, variance: PreInferenceVariance) -> Self
fn qname(&self) -> &QName
fn restriction(&self) -> &Restriction
fn to_type(&self, heap: &TypeHeap) -> Type
fn type_eq_inner(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool
fn upper_bound(&self, stdlib: &Stdlib, heap: &TypeHeap) -> Type
fn variance(&self) -> PreInferenceVariance
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

Used to represent TypeVar calls. Each TypeVar is unique, so use the ArcId to separate them.

---
