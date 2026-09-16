# `pyrefly::binding::expr`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.binding.expr.json`](../model/pyrefly.binding.expr.json)

## Usage

`enum` · `pyrefly::binding::expr::Usage`

```rust
enum Usage
```

**Variants**: `CurrentIdx`, `NonPinningValue`, `StaticTypeInformation`, `TypeAliasRhs`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn current_idx(&self) -> Option<Idx<Key>>
fn is_static(&self) -> bool
fn may_pin_partial_type(&self) -> bool
fn non_pinning_value_from(other: &Self) -> Self
```

Looking up names in an expression requires knowing the identity of the binding
we are computing for usage tracking.

There are some cases - particularly in type declaration contexts like annotations,
type variable declarations, and match patterns - that we want to skip for usage
tracking.

---
