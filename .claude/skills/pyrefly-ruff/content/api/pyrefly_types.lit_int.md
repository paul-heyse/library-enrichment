# `pyrefly_types::lit_int`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.lit_int.json`](../model/pyrefly_types.lit_int.json)

## LitInt

`struct` · `pyrefly_types::lit_int::LitInt`

```rust
struct LitInt
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn as_bool(&self) -> bool
fn as_i64(&self) -> Option<i64>
fn from_ast(x: &Int) -> Self
fn invert(&self) -> LitInt
fn negate(&self) -> Self
fn new(x: i64) -> Self
fn to_explicit_type(self) -> Type
fn to_implicit_type(self) -> Type
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
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
