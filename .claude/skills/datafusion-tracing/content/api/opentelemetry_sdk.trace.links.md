# `opentelemetry_sdk::trace::links`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.trace.links.json`](../model/opentelemetry_sdk.trace.links.json)

## SpanLinks

`struct` · `opentelemetry_sdk::trace::links::SpanLinks`

Also reachable as `opentelemetry_sdk::trace::SpanLinks`

```rust
struct SpanLinks
```

**Fields**: `links`, `dropped_count`

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

Stores span links along with dropped count.

---
