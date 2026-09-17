# `tracing_subscriber::filter::env::directive`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.filter.env.directive.json`](../model/tracing_subscriber.filter.env.directive.json)

## Directive

`struct` · `tracing_subscriber::filter::env::directive::Directive`

Also reachable as `tracing_subscriber::filter::Directive`

```rust
struct Directive
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(level: Level) -> Self
fn from(level: LevelFilter) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(from: &str) -> Result<Self, Self::Err>
```

A single filtering directive.

---
