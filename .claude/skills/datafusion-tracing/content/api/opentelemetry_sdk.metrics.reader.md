# `opentelemetry_sdk::metrics::reader`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.metrics.reader.json`](../model/opentelemetry_sdk.metrics.reader.json)

## MetricReader

`trait` · `opentelemetry_sdk::metrics::reader::MetricReader`

```rust
trait MetricReader: fmt::Debug + Send + Sync + 'static
```

**Implementors** (3)

- `opentelemetry_sdk::metrics::manual_reader::ManualReader`
- `opentelemetry_sdk::metrics::periodic_reader::PeriodicReader`
- `opentelemetry_sdk::metrics::periodic_reader_with_async_runtime::PeriodicReader`

**Methods** (6)

```rust
fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult
fn force_flush(&self) -> OTelSdkResult
fn register_pipeline(&self, pipeline: Weak<Pipeline>)
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
fn temporality(&self, kind: InstrumentKind) -> Temporality
```

The interface used between the SDK and an exporter.

Control flow is bi-directional through the `MetricReader`, since the SDK
initiates `force_flush` and `shutdown` while the reader initiates
collection. The `register_pipeline` method here informs the metric reader
that it can begin reading, signaling the start of bi-directional control
flow.

Typically, push-based exporters that are periodic will implement
`MetricExporter` themselves and construct a `PeriodicReader` to satisfy this
interface.

Pull-based exporters will typically implement `MetricReader` themselves,
since they read on demand.

---
