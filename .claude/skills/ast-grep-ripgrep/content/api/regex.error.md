# `regex::error`

Crate `regex` · 1 public items · structured records in [`model/regex.error.json`](../model/regex.error.json)

## Error

`enum` · `regex::error::Error`

Also reachable as `regex::Error`

```rust
enum Error
```

**Variants**: `Syntax`, `CompiledTooBig`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurred during parsing or compiling a regular expression.

---
