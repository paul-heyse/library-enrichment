# `ruff_server::edit`

Crate `ruff_server` · 1 public items · structured records in [`model/ruff_server.edit.json`](../model/ruff_server.edit.json)

## PositionEncoding

`enum` · `ruff_server::edit::PositionEncoding`

Also reachable as `ruff_server::PositionEncoding`

```rust
enum PositionEncoding
```

**Variants**: `UTF16`, `UTF32`, `UTF8`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &PositionEncodingKind) -> Result<Self, Self::Error>
```

A convenient enumeration for supported text encodings. Can be converted to [`lsp_types::PositionEncodingKind`].

---
