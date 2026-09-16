# `ruff_formatter::format_element::document`

Crate `ruff_formatter` · 2 public items · structured records in [`model/ruff_formatter.format_element.document.json`](../model/ruff_formatter.format_element.document.json)

## DisplayDocument

`struct` · `ruff_formatter::format_element::document::DisplayDocument`

Also reachable as `ruff_formatter::prelude::document::DisplayDocument`

```rust
struct DisplayDocument<'a>
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## Document

`struct` · `ruff_formatter::format_element::document::Document`

Also reachable as `ruff_formatter::prelude::Document`, `ruff_formatter::prelude::document::Document`

```rust
struct Document
```

**Implements**: `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn display<'a>(&'a self, source_code: SourceCode<'a>) -> DisplayDocument<'a>
```

**via `core::convert::From`**

```rust
fn from(elements: Vec<FormatElement>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A formatted document.

---
