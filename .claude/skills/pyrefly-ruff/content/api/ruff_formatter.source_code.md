# `ruff_formatter::source_code`

Crate `ruff_formatter` · 2 public items · structured records in [`model/ruff_formatter.source_code.json`](../model/ruff_formatter.source_code.json)

## SourceCode

`struct` · `ruff_formatter::source_code::SourceCode`

Also reachable as `ruff_formatter::SourceCode`

```rust
struct SourceCode<'a>
```

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn as_str(&self) -> &'a str
fn new(text: &'a str) -> Self
fn slice(self, range: TextRange) -> SourceCodeSlice
```

The source code of a document that gets formatted

---

## SourceCodeSlice

`struct` · `ruff_formatter::source_code::SourceCodeSlice`

Also reachable as `ruff_formatter::SourceCodeSlice`

```rust
struct SourceCodeSlice
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn text<'a>(&self, code: SourceCode<'a>) -> &'a str
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A slice into the source text of a document.

It only stores the range in production builds for a more compact representation, but it
keeps the original text in debug builds for better developer experience.

---
