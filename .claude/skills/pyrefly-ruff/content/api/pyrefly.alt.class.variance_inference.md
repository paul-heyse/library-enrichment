# `pyrefly::alt::class::variance_inference`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.class.variance_inference.json`](../model/pyrefly.alt.class.variance_inference.json)

## VarianceMap

`struct` · `pyrefly::alt::class::variance_inference::VarianceMap`

```rust
struct VarianceMap
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn empty() -> &'static Self
fn get(&self, parameter: &Name) -> Variance
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
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

## VarianceViolation

`struct` · `pyrefly::alt::class::variance_inference::VarianceViolation`

```rust
struct VarianceViolation
```

**Fields**: `range`, `var_name`, `position_variance`, `declared_variance`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn format_message(&self) -> String
```

---
