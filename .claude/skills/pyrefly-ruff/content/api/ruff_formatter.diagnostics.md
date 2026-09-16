# `ruff_formatter::diagnostics`

Crate `ruff_formatter` · 4 public items · structured records in [`model/ruff_formatter.diagnostics.json`](../model/ruff_formatter.diagnostics.json)

## ActualStart

`enum` · `ruff_formatter::diagnostics::ActualStart`

Also reachable as `ruff_formatter::ActualStart`

```rust
enum ActualStart
```

**Variants**: `Content`, `Start`, `End`, `EndOfDocument`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## FormatError

`enum` · `ruff_formatter::diagnostics::FormatError`

Also reachable as `ruff_formatter::FormatError`

```rust
enum FormatError
```

**Variants**: `SyntaxError`, `RangeError`, `InvalidDocument`, `PoorLayout`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn syntax_error(message: &'static str) -> Self
```

**via `core::convert::From`**

```rust
fn from(error: PrintError) -> Self
fn from(error: &PrintError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Series of errors encountered during formatting

---

## InvalidDocumentError

`enum` · `ruff_formatter::diagnostics::InvalidDocumentError`

Also reachable as `ruff_formatter::InvalidDocumentError`

```rust
enum InvalidDocumentError
```

**Variants**: `StartEndTagMismatch`, `StartTagMissing`, `ExpectedStart`, `UnknownGroupId`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## PrintError

`enum` · `ruff_formatter::diagnostics::PrintError`

Also reachable as `ruff_formatter::PrintError`

```rust
enum PrintError
```

**Variants**: `InvalidDocument`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---
