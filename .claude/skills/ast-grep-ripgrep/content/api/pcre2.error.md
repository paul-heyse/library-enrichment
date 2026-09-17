# `pcre2::error`

Crate `pcre2` · 2 public items · structured records in [`model/pcre2.error.json`](../model/pcre2.error.json)

## ErrorKind

`enum` · `pcre2::error::ErrorKind`

Also reachable as `pcre2::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `Compile`, `JIT`, `Match`, `Info`, `Option`

**Derives**: Clone, Debug

The kind of an error indicates the type of operation that was attempted
that resulted in an error.

This enum may expand over time.

---

## Error

`struct` · `pcre2::error::Error`

Also reachable as `pcre2::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn code(&self) -> c_int
fn kind(&self) -> &ErrorKind
fn offset(&self) -> Option<usize>
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A PCRE2 error.

An error can occur during compilation or during matching. The kind of this
error indicates the type of operation being performed when the error
occurred.

---
