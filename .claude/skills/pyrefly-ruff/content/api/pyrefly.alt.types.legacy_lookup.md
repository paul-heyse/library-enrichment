# `pyrefly::alt::types::legacy_lookup`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.alt.types.legacy_lookup.json`](../model/pyrefly.alt.types.legacy_lookup.json)

## LegacyTypeParameterLookup

`enum` · `pyrefly::alt::types::legacy_lookup::LegacyTypeParameterLookup`

```rust
enum LegacyTypeParameterLookup
```

**Variants**: `Parameter`, `NotParameter`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn parameter(&self) -> Option<&Quantified>
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

Python's legacy (pre-PEP 695) type variable syntax is not syntactic at all, it requires
name resolution of global variables plus multiple sets of rules for when a global that
is a type variable placeholder is allowed to be used as a type parameter.

This type represents the result of such a lookup: given a name appearing in a function or
a class, we either determine that the name is *not* a type variable and return the type
for the name, or we determine that it is one and create a `Quantified` that
represents that variable as a type parameter.

---
