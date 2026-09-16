# `ruff_python_codegen::stylist`

Crate `ruff_python_codegen` · 2 public items · structured records in [`model/ruff_python_codegen.stylist.json`](../model/ruff_python_codegen.stylist.json)

## Indentation

`struct` · `ruff_python_codegen::stylist::Indentation`

Also reachable as `ruff_python_codegen::Indentation`

```rust
struct Indentation
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_char(&self) -> char
fn as_str(&self) -> &str
const fn new(indentation: String) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

The indentation style used in Python source code.

---

## Stylist

`struct` · `ruff_python_codegen::stylist::Stylist`

Also reachable as `ruff_python_codegen::Stylist`

```rust
struct Stylist<'a>
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn from_tokens(tokens: &Tokens, source: &'a str) -> Self
fn indentation(&self) -> &Indentation
fn into_owned(self) -> Stylist<'static>
fn line_ending(&self) -> LineEnding
fn quote(&self) -> Quote
```

---
