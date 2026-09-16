# `pyrefly_types::literal`

Crate `pyrefly_types` · 4 public items · structured records in [`model/pyrefly_types.literal.json`](../model/pyrefly_types.literal.json)

## Lit

`enum` · `pyrefly_types::literal::Lit`

```rust
enum Lit
```

**Variants**: `Str`, `Int`, `Bool`, `Bytes`, `Enum`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (15)

```rust
fn as_index_i64(&self) -> Option<i64>
fn from_boolean_literal(x: &ExprBooleanLiteral) -> Self
fn from_bytes_literal(x: &ExprBytesLiteral) -> Option<Self>
fn from_fstring(x: &ExprFString) -> Option<Self>
fn from_int(x: &Int) -> Self
fn from_string_literal(x: &ExprStringLiteral) -> Option<Self>
fn general_class_type<'a>(&'a self, stdlib: &'a Stdlib) -> &'a ClassType
fn invert(&self) -> Option<Type>
fn is_string(&self) -> bool
fn negate(&self) -> Option<Type>
fn positive(&self) -> Option<Type>
fn to_explicit_type(self) -> Type
fn to_implicit_type(self) -> Type
fn to_string_escaped(&self, use_single_quotes_for_string: bool) -> String
fn write_escaped(&self, f: &mut impl fmt::Write, use_single_quotes_for_string: bool) -> fmt::Result
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

A literal value.

---

## LitStyle

`enum` · `pyrefly_types::literal::LitStyle`

```rust
enum LitStyle
```

**Variants**: `Implicit`, `Explicit`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

## LitEnum

`struct` · `pyrefly_types::literal::LitEnum`

```rust
struct LitEnum
```

**Fields**: `class`, `member`, `ty`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

---

## Literal

`struct` · `pyrefly_types::literal::Literal`

```rust
struct Literal
```

**Fields**: `value`, `style`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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
