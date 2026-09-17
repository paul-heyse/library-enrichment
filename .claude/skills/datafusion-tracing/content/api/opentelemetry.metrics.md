# `opentelemetry::metrics`

Crate `opentelemetry` · 1 public items · structured records in [`model/opentelemetry.metrics.json`](../model/opentelemetry.metrics.json)

## InstrumentProvider

`trait` · `opentelemetry::metrics::InstrumentProvider`

```rust
trait InstrumentProvider
```

**Methods** (16)

```rust
fn f64_counter(&self, _builder: InstrumentBuilder<'_, Counter<f64>>) -> Counter<f64>
fn f64_gauge(&self, _builder: InstrumentBuilder<'_, Gauge<f64>>) -> Gauge<f64>
fn f64_histogram(&self, _builder: HistogramBuilder<'_, Histogram<f64>>) -> Histogram<f64>
fn f64_observable_counter(&self, _builder: AsyncInstrumentBuilder<'_, ObservableCounter<f64>, f64>) -> ObservableCounter<f64>
fn f64_observable_gauge(&self, _builder: AsyncInstrumentBuilder<'_, ObservableGauge<f64>, f64>) -> ObservableGauge<f64>
fn f64_observable_up_down_counter(&self, _builder: AsyncInstrumentBuilder<'_, ObservableUpDownCounter<f64>, f64>) -> ObservableUpDownCounter<f64>
fn f64_up_down_counter(&self, _builder: InstrumentBuilder<'_, UpDownCounter<f64>>) -> UpDownCounter<f64>
fn i64_gauge(&self, _builder: InstrumentBuilder<'_, Gauge<i64>>) -> Gauge<i64>
fn i64_observable_gauge(&self, _builder: AsyncInstrumentBuilder<'_, ObservableGauge<i64>, i64>) -> ObservableGauge<i64>
fn i64_observable_up_down_counter(&self, _builder: AsyncInstrumentBuilder<'_, ObservableUpDownCounter<i64>, i64>) -> ObservableUpDownCounter<i64>
fn i64_up_down_counter(&self, _builder: InstrumentBuilder<'_, UpDownCounter<i64>>) -> UpDownCounter<i64>
fn u64_counter(&self, _builder: InstrumentBuilder<'_, Counter<u64>>) -> Counter<u64>
fn u64_gauge(&self, _builder: InstrumentBuilder<'_, Gauge<u64>>) -> Gauge<u64>
fn u64_histogram(&self, _builder: HistogramBuilder<'_, Histogram<u64>>) -> Histogram<u64>
fn u64_observable_counter(&self, _builder: AsyncInstrumentBuilder<'_, ObservableCounter<u64>, u64>) -> ObservableCounter<u64>
fn u64_observable_gauge(&self, _builder: AsyncInstrumentBuilder<'_, ObservableGauge<u64>, u64>) -> ObservableGauge<u64>
```

SDK implemented trait for creating instruments

---
