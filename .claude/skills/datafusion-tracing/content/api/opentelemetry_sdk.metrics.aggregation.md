# `opentelemetry_sdk::metrics::aggregation`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.metrics.aggregation.json`](../model/opentelemetry_sdk.metrics.aggregation.json)

## Aggregation

`enum` · `opentelemetry_sdk::metrics::aggregation::Aggregation`

Also reachable as `opentelemetry_sdk::metrics::Aggregation`

```rust
enum Aggregation
```

**Variants**: `Drop`, `Default`, `Sum`, `LastValue`, `ExplicitBucketHistogram`, `Base2ExponentialHistogram`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

The way recorded measurements are summarized.

---
