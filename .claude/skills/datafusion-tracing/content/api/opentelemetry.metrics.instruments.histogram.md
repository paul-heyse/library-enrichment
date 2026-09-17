# `opentelemetry::metrics::instruments::histogram`

Crate `opentelemetry` · 1 public items · structured records in [`model/opentelemetry.metrics.instruments.histogram.json`](../model/opentelemetry.metrics.instruments.histogram.json)

## Histogram

`struct` · `opentelemetry::metrics::instruments::histogram::Histogram`

Also reachable as `opentelemetry::metrics::Histogram`

```rust
struct Histogram<T>
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(inner: Arc<dyn SyncInstrument<T> + Send + Sync>) -> Self
fn record(&self, value: T, attributes: &[KeyValue])
```

An instrument that records a distribution of values.

[`Histogram`] can be cloned to create multiple handles to the same instrument. If a [`Histogram`] needs to be shared,
users are recommended to clone the [`Histogram`] instead of creating duplicate [`Histogram`]s for the same metric. Creating
duplicate [`Histogram`]s for the same metric could lower SDK performance.

---
