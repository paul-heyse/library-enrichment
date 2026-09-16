# `pyrefly::binding::base_class`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.binding.base_class.json`](../model/pyrefly.binding.base_class.json)

## BaseClass

`enum` · `pyrefly::binding::base_class::BaseClass`

```rust
enum BaseClass
```

**Variants**: `TypedDict`, `Generic`, `BaseClassExpr`, `InvalidExpr`, `NamedTuple`, `SynthesizedBase`, `TypeOf`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn is_generic(&self) -> bool
fn is_typed_dict(&self) -> bool
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Helper type used to share part of the logic needed for the
binding-level work of finding legacy type parameters versus the type-level
work of computing inheritance information and the MRO.

---

## BaseClassExpr

`enum` · `pyrefly::binding::base_class::BaseClassExpr`

```rust
enum BaseClassExpr
```

**Variants**: `Name`, `Attribute`, `Subscript`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn from_expr(expr: &Expr) -> Option<Self>
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

We only recognize a small subset of syntactical forms for expression that may appear on base classes

---

## BaseClassGenericKind

`enum` · `pyrefly::binding::base_class::BaseClassGenericKind`

```rust
enum BaseClassGenericKind
```

**Variants**: `Generic`, `Protocol`

**Derives**: Clone, Debug

---

## BaseClassGeneric

`struct` · `pyrefly::binding::base_class::BaseClassGeneric`

```rust
struct BaseClassGeneric
```

**Fields**: `kind`, `args`, `range`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---
