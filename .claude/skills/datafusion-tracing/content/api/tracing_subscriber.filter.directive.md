# `tracing_subscriber::filter::directive`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.filter.directive.json`](../model/tracing_subscriber.filter.directive.json)

## ParseError

`struct` · `tracing_subscriber::filter::directive::ParseError`

Also reachable as `tracing_subscriber::filter::ParseError`

```rust
struct ParseError
```

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(l: level::ParseError) -> Self
fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Indicates that a string could not be parsed as a filtering directive.

---
