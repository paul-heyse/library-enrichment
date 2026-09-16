# `ruff_python_literal::escape`

Crate `ruff_python_literal` · 6 public items · structured records in [`model/ruff_python_literal.escape.json`](../model/ruff_python_literal.escape.json)

## AsciiEscape

`struct` · `ruff_python_literal::escape::AsciiEscape`

```rust
struct AsciiEscape<'a>
```

**Implements**: `ruff_python_literal::escape::Escape`

**Methods** (5)

```rust
fn bytes_repr<'r>(&'a self, triple_quotes: TripleQuotes) -> BytesRepr<'r, 'a>
fn new(source: &'a [u8], layout: EscapeLayout) -> Self
fn new_repr(source: &'a [u8]) -> Self
fn repr_layout(source: &[u8], preferred_quote: Quote) -> EscapeLayout
fn with_preferred_quote(source: &'a [u8], quote: Quote) -> Self
```

**via `ruff_python_literal::escape::Escape`**

```rust
fn layout(&self) -> &EscapeLayout
fn source_len(&self) -> usize
fn write_body_slow(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
fn write_source(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
```

---

## BytesRepr

`struct` · `ruff_python_literal::escape::BytesRepr`

```rust
struct BytesRepr<'r, 'a>
```

**Implements**: `core::fmt::Display`

**Methods** (2)

```rust
fn to_string(&self) -> Option<String>
fn write(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## EscapeLayout

`struct` · `ruff_python_literal::escape::EscapeLayout`

```rust
struct EscapeLayout
```

**Fields**: `quote`, `len`

---

## StrRepr

`struct` · `ruff_python_literal::escape::StrRepr`

```rust
struct StrRepr<'r, 'a>
```

**Implements**: `core::fmt::Display`

**Methods** (2)

```rust
fn to_string(&self) -> Option<String>
fn write(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## UnicodeEscape

`struct` · `ruff_python_literal::escape::UnicodeEscape`

```rust
struct UnicodeEscape<'a>
```

**Implements**: `ruff_python_literal::escape::Escape`

**Methods** (4)

```rust
fn new_repr(source: &'a str) -> Self
fn repr_layout(source: &str, preferred_quote: Quote) -> EscapeLayout
fn str_repr<'r>(&'a self, triple_quotes: TripleQuotes) -> StrRepr<'r, 'a>
fn with_preferred_quote(source: &'a str, quote: Quote) -> Self
```

**via `ruff_python_literal::escape::Escape`**

```rust
fn layout(&self) -> &EscapeLayout
fn source_len(&self) -> usize
fn write_body_slow(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
fn write_source(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
```

---

## Escape

`trait` · `ruff_python_literal::escape::Escape`

```rust
trait Escape
```

**Implementors** (2)

- `ruff_python_literal::escape::AsciiEscape`
- `ruff_python_literal::escape::UnicodeEscape`

**Methods** (6)

```rust
fn changed(&self) -> bool
fn layout(&self) -> &EscapeLayout
fn source_len(&self) -> usize
fn write_body(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
fn write_body_slow(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
fn write_source(&self, formatter: &mut impl std::fmt::Write) -> std::fmt::Result
```

---
