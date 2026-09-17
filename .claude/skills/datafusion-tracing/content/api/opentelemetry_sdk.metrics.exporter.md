# `opentelemetry_sdk::metrics::exporter`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.metrics.exporter.json`](../model/opentelemetry_sdk.metrics.exporter.json)

## PushMetricExporter

`trait` · `opentelemetry_sdk::metrics::exporter::PushMetricExporter`

```rust
trait PushMetricExporter: Send + Sync + 'static
```

**Implementors** (2)

- `opentelemetry_otlp::metric::MetricExporter`
- `opentelemetry_sdk::metrics::in_memory_exporter::InMemoryMetricExporter`

**Methods** (5)

```rust
fn export(&self, metrics: &ResourceMetrics) -> impl std::future::Future<Output = OTelSdkResult> + Send
fn force_flush(&self) -> OTelSdkResult
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
fn temporality(&self) -> Temporality
```

Exporter handles the delivery of metric data to external receivers.

This is the final component in the metric push pipeline.

---
