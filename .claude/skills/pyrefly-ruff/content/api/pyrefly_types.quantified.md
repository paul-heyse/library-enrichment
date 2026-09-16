# `pyrefly_types::quantified`

Crate `pyrefly_types` · 5 public items · structured records in [`model/pyrefly_types.quantified.json`](../model/pyrefly_types.quantified.json)

## QuantifiedKind

`enum` · `pyrefly_types::quantified::QuantifiedKind`

```rust
enum QuantifiedKind
```

**Variants**: `TypeVar`, `IntVar`, `ParamSpec`, `TypeVarTuple`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

## QuantifiedOrigin

`enum` · `pyrefly_types::quantified::QuantifiedOrigin`

```rust
enum QuantifiedOrigin
```

**Variants**: `ScopedLegacy`, `Pep695`, `Synthetic`, `MapIntTuplesParameter`, `NormalizedMapIntTuplesParameter`

**Implements**: `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn synthetic() -> Self
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

Discriminator for the origin of a `Quantified`, making collisions structurally impossible
between quantifieds that share the same anchor range but have different origins.

---

## AnchorIndex

`struct` · `pyrefly_types::quantified::AnchorIndex`

```rust
struct AnchorIndex
```

**Fields**: `range`, `index`

**Implements**: `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn first(range: TextRange) -> Self
fn new(range: TextRange, index: u32) -> Self
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

A source range plus an index that disambiguates multiple quantifieds sharing the same range.
Index 0 is used when a range produces exactly one quantified; higher indices are used when
a single range produces several (e.g. multiple type parameters anchored to the same scope).

---

## Quantified

`struct` · `pyrefly_types::quantified::Quantified`

```rust
struct Quantified
```

**Fields**: `name`, `kind`, `default`, `restriction`, `owner`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (24)

```rust
fn as_gradual_type(&self) -> Type
fn as_gradual_type_helper(kind: QuantifiedKind, default: Option<&Type>) -> Type
fn class_type<'a>(&self, stdlib: &'a Stdlib) -> &'a ClassType
fn default(&self) -> Option<&Type>
fn display_name_with_prefix(&self) -> impl Display + '_
fn display_with_bounds(&self) -> impl Display + '_
fn from_type_var(tv: &TypeVar, identity: QuantifiedIdentity) -> Self
fn identity(&self) -> &QuantifiedIdentity
fn is_param_spec(&self) -> bool
fn is_type_var(&self) -> bool
fn is_type_var_tuple(&self) -> bool
fn kind(&self) -> QuantifiedKind
fn name(&self) -> &Name
fn new(identity: QuantifiedIdentity, name: Name, kind: QuantifiedKind, default: Option<Type>, restriction: Restriction, variance: PreInferenceVariance) -> Self
fn param_spec(name: Name, identity: QuantifiedIdentity, default: Option<Type>) -> Self
fn restriction(&self) -> &Restriction
fn to_type(self, heap: &TypeHeap) -> Type
fn to_value(self) -> Type
fn type_var(name: Name, identity: QuantifiedIdentity, default: Option<Type>, restriction: Restriction, variance: PreInferenceVariance) -> Self
fn type_var_tuple(name: Name, identity: QuantifiedIdentity, default: Option<Type>) -> Self
fn upper_bound(&self, stdlib: &Stdlib, heap: &TypeHeap) -> Type
fn variance(&self) -> PreInferenceVariance
fn with_owner(self, owner: Name) -> Self
fn with_restriction(self, restriction: Restriction) -> Self
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

## QuantifiedIdentity

`struct` · `pyrefly_types::quantified::QuantifiedIdentity`

```rust
struct QuantifiedIdentity
```

**Fields**: `module`, `anchor`, `origin`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(module: ModuleName, anchor: AnchorIndex, origin: QuantifiedOrigin) -> Self
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
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

Deterministic identity for a `Quantified`, derived from source locations rather than
allocation order. Two quantifieds are the same iff their identity is the same.

Globally unique because `module` distinguishes cross-module collisions, `anchor` pins
the source location and index, and `origin` prevents collisions between quantifieds
of different kinds at the same anchor.

---
