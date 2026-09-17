# `grep_regex::error`

Crate `grep-regex` · 2 public items · structured records in [`model/grep_regex.error.json`](../model/grep_regex.error.json)

## ErrorKind

`enum` · `grep_regex::error::ErrorKind`

Also reachable as `grep_regex::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `Regex`, `NotAllowed`, `InvalidLineTerminator`, `Banned`

**Derives**: Clone, Debug

The kind of an error that can occur.

---

## Error

`struct` · `grep_regex::error::Error`

Also reachable as `grep_regex::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn kind(&self) -> &ErrorKind
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that can occur in this crate.

Generally, this error corresponds to problems building a regular
expression, whether it's in parsing, compilation or a problem with
guaranteeing a configured optimization.

---
