# `grep_pcre2::error`

Crate `grep-pcre2` · 2 public items · structured records in [`model/grep_pcre2.error.json`](../model/grep_pcre2.error.json)

## ErrorKind

`enum` · `grep_pcre2::error::ErrorKind`

Also reachable as `grep_pcre2::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `Regex`

**Derives**: Clone, Debug

The kind of an error that can occur.

---

## Error

`struct` · `grep_pcre2::error::Error`

Also reachable as `grep_pcre2::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn kind(&self) -> &ErrorKind
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
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
