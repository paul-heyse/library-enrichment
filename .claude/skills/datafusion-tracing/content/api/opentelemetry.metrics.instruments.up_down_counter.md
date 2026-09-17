# `opentelemetry::metrics::instruments::up_down_counter`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.metrics.instruments.up_down_counter.json`](../model/opentelemetry.metrics.instruments.up_down_counter.json)

## ObservableUpDownCounter

`struct` · `opentelemetry::metrics::instruments::up_down_counter::ObservableUpDownCounter`

Also reachable as `opentelemetry::metrics::ObservableUpDownCounter`

```rust
struct ObservableUpDownCounter<T>
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new() -> Self
```

An async instrument that records increasing or decreasing values.

---

## UpDownCounter

`struct` · `opentelemetry::metrics::instruments::up_down_counter::UpDownCounter`

Also reachable as `opentelemetry::metrics::UpDownCounter`

```rust
struct UpDownCounter<T>
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn add(&self, value: T, attributes: &[KeyValue])
fn new(inner: Arc<dyn SyncInstrument<T> + Send + Sync>) -> Self
```

An instrument that records increasing or decreasing values.

[`UpDownCounter`] can be cloned to create multiple handles to the same instrument. If a [`UpDownCounter`] needs to be shared,
users are recommended to clone the [`UpDownCounter`] instead of creating duplicate [`UpDownCounter`]s for the same metric. Creating
duplicate [`UpDownCounter`]s for the same metric could lower SDK performance.

---
