# `tracing_subscriber::filter::env::field`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.filter.env.field.json`](../model/tracing_subscriber.filter.env.field.json)

## BadName

`struct` · `tracing_subscriber::filter::env::field::BadName`

Also reachable as `tracing_subscriber::filter::BadFieldName`

```rust
struct BadName
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Indicates that a field name specified in a filter directive was invalid.

---
