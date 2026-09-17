# `regex_syntax::error`

Crate `regex-syntax` · 1 public items · structured records in [`model/regex_syntax.error.json`](../model/regex_syntax.error.json)

## Error

`enum` · `regex_syntax::error::Error`

Also reachable as `regex_syntax::Error`

```rust
enum Error
```

**Variants**: `Parse`, `Translate`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(err: ast::Error) -> Error
fn from(err: hir::Error) -> Error
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

This error type encompasses any error that can be returned by this crate.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

---
