# `opentelemetry::metrics::instruments::counter`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.metrics.instruments.counter.json`](../model/opentelemetry.metrics.instruments.counter.json)

## Counter

`struct` · `opentelemetry::metrics::instruments::counter::Counter`

Also reachable as `opentelemetry::metrics::Counter`

```rust
struct Counter<T>
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn add(&self, value: T, attributes: &[KeyValue])
fn new(inner: Arc<dyn SyncInstrument<T> + Send + Sync>) -> Self
```

An instrument that records increasing values.

[`Counter`] can be cloned to create multiple handles to the same instrument. If a [`Counter`] needs to be shared,
users are recommended to clone the [`Counter`] instead of creating duplicate [`Counter`]s for the same metric. Creating
duplicate [`Counter`]s for the same metric could lower SDK performance.

---

## ObservableCounter

`struct` · `opentelemetry::metrics::instruments::counter::ObservableCounter`

Also reachable as `opentelemetry::metrics::ObservableCounter`

```rust
struct ObservableCounter<T>
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new() -> Self
```

An async instrument that records increasing values.

---
