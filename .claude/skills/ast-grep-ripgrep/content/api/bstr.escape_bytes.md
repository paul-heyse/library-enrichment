# `bstr::escape_bytes`

Crate `bstr` · 1 public items · structured records in [`model/bstr.escape_bytes.json`](../model/bstr.escape_bytes.json)

## EscapeBytes

`struct` · `bstr::escape_bytes::EscapeBytes`

Also reachable as `bstr::EscapeBytes`

```rust
struct EscapeBytes<'a>
```

**Implements**: `core::fmt::Display`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<char>
```

An iterator of `char` values that represent an escaping of arbitrary bytes.

The lifetime parameter `'a` refers to the lifetime of the bytes being
escaped.

This iterator is created by the
[`ByteSlice::escape_bytes`](crate::ByteSlice::escape_bytes) method.

---
