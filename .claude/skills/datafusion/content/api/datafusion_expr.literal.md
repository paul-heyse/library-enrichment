# `datafusion_expr::literal`

Crate `datafusion-expr` · 5 public items · structured records in [`model/datafusion_expr.literal.json`](../model/datafusion_expr.literal.json)

## lit

`function` · `datafusion_expr::literal::lit`

Also reachable as `datafusion::logical_expr::lit`, `datafusion::prelude::lit`, `datafusion_expr::lit`

```rust
fn lit<T: Literal>(n: T) -> Expr
```

Create a literal expression

---

## lit_timestamp_nano

`function` · `datafusion_expr::literal::lit_timestamp_nano`

Also reachable as `datafusion::logical_expr::lit_timestamp_nano`, `datafusion::prelude::lit_timestamp_nano`, `datafusion_expr::lit_timestamp_nano`

```rust
fn lit_timestamp_nano<T: TimestampLiteral>(n: T) -> Expr
```

Create a literal timestamp expression

---

## lit_with_metadata

`function` · `datafusion_expr::literal::lit_with_metadata`

Also reachable as `datafusion::logical_expr::lit_with_metadata`, `datafusion_expr::lit_with_metadata`

```rust
fn lit_with_metadata<T: Literal>(n: T, metadata: Option<datafusion_common::metadata::FieldMetadata>) -> Expr
```

---

## Literal

`trait` · `datafusion_expr::literal::Literal`

Also reachable as `datafusion::logical_expr::Literal`, `datafusion_expr::Literal`

```rust
trait Literal
```

**Implementors** (11)

- `alloc::string::String`
- `alloc::vec::Vec`
- `core::num::nonzero::NonZeroI16`
- `core::num::nonzero::NonZeroI32`
- `core::num::nonzero::NonZeroI64`
- `core::num::nonzero::NonZeroI8`
- `core::num::nonzero::NonZeroU16`
- `core::num::nonzero::NonZeroU32`
- `core::num::nonzero::NonZeroU64`
- `core::num::nonzero::NonZeroU8`
- `datafusion_common::scalar::ScalarValue`

**Methods** (1)

```rust
fn lit(&self) -> Expr
```

Trait for converting a type to a [`Literal`] literal expression.

---

## TimestampLiteral

`trait` · `datafusion_expr::literal::TimestampLiteral`

Also reachable as `datafusion::logical_expr::TimestampLiteral`, `datafusion_expr::TimestampLiteral`

```rust
trait TimestampLiteral
```

**Methods** (1)

```rust
fn lit_timestamp_nano(&self) -> Expr
```

Trait for converting a type to a literal timestamp

---
