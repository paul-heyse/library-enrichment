# `opentelemetry_sdk::trace::events`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.trace.events.json`](../model/opentelemetry_sdk.trace.events.json)

## SpanEvents

`struct` · `opentelemetry_sdk::trace::events::SpanEvents`

Also reachable as `opentelemetry_sdk::trace::SpanEvents`

```rust
struct SpanEvents
```

**Fields**: `events`, `dropped_count`

**Implements**: `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Stores span events along with dropped count.

---
