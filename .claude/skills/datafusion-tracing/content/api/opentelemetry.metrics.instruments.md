# `opentelemetry::metrics::instruments`

Crate `opentelemetry` · 6 public items · structured records in [`model/opentelemetry.metrics.instruments.json`](../model/opentelemetry.metrics.instruments.json)

## AsyncInstrumentBuilder

`struct` · `opentelemetry::metrics::instruments::AsyncInstrumentBuilder`

Also reachable as `opentelemetry::metrics::AsyncInstrumentBuilder`

```rust
struct AsyncInstrumentBuilder<'a, I, M>
```

**Fields**: `instrument_provider`, `name`, `description`, `unit`, `callbacks`

**Derives**: Debug

**Methods** (10)

```rust
fn build(self) -> ObservableGauge<u64>
fn build(self) -> ObservableUpDownCounter<i64>
fn build(self) -> ObservableCounter<f64>
fn build(self) -> ObservableGauge<i64>
fn build(self) -> ObservableCounter<u64>
fn build(self) -> ObservableGauge<f64>
fn build(self) -> ObservableUpDownCounter<f64>
fn with_callback<F>(self, callback: F) -> Self where F: Fn(&dyn AsyncInstrument<M>) + Send + Sync + 'static
fn with_description<S: Into<Cow<'static, str>>>(self, description: S) -> Self
fn with_unit<S: Into<Cow<'static, str>>>(self, unit: S) -> Self
```

Configuration for building an async instrument.

---

## HistogramBuilder

`struct` · `opentelemetry::metrics::instruments::HistogramBuilder`

Also reachable as `opentelemetry::metrics::HistogramBuilder`

```rust
struct HistogramBuilder<'a, T>
```

**Fields**: `instrument_provider`, `name`, `description`, `unit`, `boundaries`

**Derives**: Debug

**Methods** (5)

```rust
fn build(self) -> Histogram<u64>
fn build(self) -> Histogram<f64>
fn with_boundaries(self, boundaries: Vec<f64>) -> Self
fn with_description<S: Into<Cow<'static, str>>>(self, description: S) -> Self
fn with_unit<S: Into<Cow<'static, str>>>(self, unit: S) -> Self
```

Configuration for building a Histogram.

---

## InstrumentBuilder

`struct` · `opentelemetry::metrics::instruments::InstrumentBuilder`

Also reachable as `opentelemetry::metrics::InstrumentBuilder`

```rust
struct InstrumentBuilder<'a, T>
```

**Fields**: `instrument_provider`, `name`, `description`, `unit`

**Derives**: Debug

**Methods** (9)

```rust
fn build(self) -> Gauge<f64>
fn build(self) -> UpDownCounter<f64>
fn build(self) -> Gauge<u64>
fn build(self) -> UpDownCounter<i64>
fn build(self) -> Counter<f64>
fn build(self) -> Gauge<i64>
fn build(self) -> Counter<u64>
fn with_description<S: Into<Cow<'static, str>>>(self, description: S) -> Self
fn with_unit<S: Into<Cow<'static, str>>>(self, unit: S) -> Self
```

Configuration for building a sync instrument.

---

## AsyncInstrument

`trait` · `opentelemetry::metrics::instruments::AsyncInstrument`

Also reachable as `opentelemetry::metrics::AsyncInstrument`

```rust
trait AsyncInstrument<T>: Send + Sync
```

**Methods** (1)

```rust
fn observe(&self, measurement: T, attributes: &[KeyValue])
```

An SDK implemented instrument that records measurements via callback.

---

## SyncInstrument

`trait` · `opentelemetry::metrics::instruments::SyncInstrument`

Also reachable as `opentelemetry::metrics::SyncInstrument`

```rust
trait SyncInstrument<T>: Send + Sync
```

**Methods** (1)

```rust
fn measure(&self, measurement: T, attributes: &[KeyValue])
```

An SDK implemented instrument that records measurements synchronously.

---

## Callback

`type_alias` · `opentelemetry::metrics::instruments::Callback`

Also reachable as `opentelemetry::metrics::Callback`

```rust
type Callback<T> = Box<dyn Fn(&dyn AsyncInstrument<T>) + Send + Sync>
```

A function registered with a [Meter] that makes observations for the
instruments it is registered with.

The async instrument parameter is used to record measurement observations
for these instruments.

The function needs to complete in a finite amount of time.

---
