# `ra_ap_syntax::syntax_error`

Crate `ra_ap_syntax` · 1 public items · structured records in [`model/ra_ap_syntax.syntax_error.json`](../model/ra_ap_syntax.syntax_error.json)

## SyntaxError

`struct` · `ra_ap_syntax::syntax_error::SyntaxError`

Also reachable as `ra_ap_syntax::SyntaxError`

```rust
struct SyntaxError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn new(message: impl Into<String>, range: TextRange) -> Self
fn new_at_offset(message: impl Into<String>, offset: TextSize) -> Self
fn range(&self) -> TextRange
fn with_range(self, range: TextRange) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Represents the result of unsuccessful tokenization, parsing
or tree validation.

---
