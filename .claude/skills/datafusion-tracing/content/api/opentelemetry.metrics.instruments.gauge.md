# `opentelemetry::metrics::instruments::gauge`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.metrics.instruments.gauge.json`](../model/opentelemetry.metrics.instruments.gauge.json)

## Gauge

`struct` · `opentelemetry::metrics::instruments::gauge::Gauge`

Also reachable as `opentelemetry::metrics::Gauge`

```rust
struct Gauge<T>
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(inner: Arc<dyn SyncInstrument<T> + Send + Sync>) -> Self
fn record(&self, value: T, attributes: &[KeyValue])
```

An instrument that records independent values

[`Gauge`] can be cloned to create multiple handles to the same instrument. If a [`Gauge`] needs to be shared,
users are recommended to clone the [`Gauge`] instead of creating duplicate [`Gauge`]s for the same metric. Creating
duplicate [`Gauge`]s for the same metric could lower SDK performance.

---

## ObservableGauge

`struct` · `opentelemetry::metrics::instruments::gauge::ObservableGauge`

Also reachable as `opentelemetry::metrics::ObservableGauge`

```rust
struct ObservableGauge<T>
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new() -> Self
```

An async instrument that records independent readings.

---
