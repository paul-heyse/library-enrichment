# `opentelemetry_otlp::metric`

Crate `opentelemetry-otlp` · 6 public items · structured records in [`model/opentelemetry_otlp.metric.json`](../model/opentelemetry_otlp.metric.json)

## OTEL_EXPORTER_OTLP_METRICS_COMPRESSION

`constant` · `opentelemetry_otlp::metric::OTEL_EXPORTER_OTLP_METRICS_COMPRESSION`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_METRICS_COMPRESSION`

```rust
const OTEL_EXPORTER_OTLP_METRICS_COMPRESSION: &str = "OTEL_EXPORTER_OTLP_METRICS_COMPRESSION"
```

Compression algorithm to use, defaults to none.

---

## OTEL_EXPORTER_OTLP_METRICS_ENDPOINT

`constant` · `opentelemetry_otlp::metric::OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`

```rust
const OTEL_EXPORTER_OTLP_METRICS_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_METRICS_ENDPOINT"
```

Target to which the exporter is going to send metrics, defaults to https://localhost:4317/v1/metrics.
Learn about the relationship between this constant and default/spans/logs at
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#endpoint-urls-for-otlphttp>

---

## OTEL_EXPORTER_OTLP_METRICS_HEADERS

`constant` · `opentelemetry_otlp::metric::OTEL_EXPORTER_OTLP_METRICS_HEADERS`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_METRICS_HEADERS`

```rust
const OTEL_EXPORTER_OTLP_METRICS_HEADERS: &str = "OTEL_EXPORTER_OTLP_METRICS_HEADERS"
```

Key-value pairs to be used as headers associated with gRPC or HTTP requests
for sending metrics.
Example: `k1=v1,k2=v2`
Note: this is only supported for HTTP.

---

## OTEL_EXPORTER_OTLP_METRICS_TIMEOUT

`constant` · `opentelemetry_otlp::metric::OTEL_EXPORTER_OTLP_METRICS_TIMEOUT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_METRICS_TIMEOUT`

```rust
const OTEL_EXPORTER_OTLP_METRICS_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_METRICS_TIMEOUT"
```

Max waiting time for the backend to process each metrics batch, defaults to 10s.

---

## MetricExporter

`struct` · `opentelemetry_otlp::metric::MetricExporter`

Also reachable as `opentelemetry_otlp::MetricExporter`

```rust
struct MetricExporter
```

**Implements**: `opentelemetry_sdk::metrics::exporter::PushMetricExporter`

**Derives**: Debug

**Methods** (1)

```rust
fn builder() -> MetricExporterBuilder<NoExporterBuilderSet>
```

**via `opentelemetry_sdk::metrics::exporter::PushMetricExporter`**

```rust
async fn export(&self, metrics: &ResourceMetrics) -> OTelSdkResult
fn force_flush(&self) -> OTelSdkResult
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, _timeout: std::time::Duration) -> OTelSdkResult
fn temporality(&self) -> Temporality
```

Export metrics in OTEL format.

---

## MetricExporterBuilder

`struct` · `opentelemetry_otlp::metric::MetricExporterBuilder`

Also reachable as `opentelemetry_otlp::MetricExporterBuilder`

```rust
struct MetricExporterBuilder<C>
```

**Implements**: `opentelemetry_otlp::exporter::HasExportConfig`, `opentelemetry_otlp::exporter::http::HasHttpConfig`, `opentelemetry_otlp::exporter::tonic::HasTonicConfig`

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn build(self) -> Result<MetricExporter, ExporterBuildError>
fn build(self) -> Result<MetricExporter, ExporterBuildError>
fn new() -> Self
fn with_http(self) -> MetricExporterBuilder<HttpExporterBuilderSet>
fn with_temporality(self, temporality: Temporality) -> MetricExporterBuilder<C>
fn with_tonic(self) -> MetricExporterBuilder<TonicExporterBuilderSet>
```

**via `opentelemetry_otlp::exporter::HasExportConfig`**

```rust
fn export_config(&mut self) -> &mut ExportConfig
fn export_config(&mut self) -> &mut ExportConfig
```

**via `opentelemetry_otlp::exporter::http::HasHttpConfig`**

```rust
fn http_client_config(&mut self) -> &mut exporter::http::HttpConfig
```

**via `opentelemetry_otlp::exporter::tonic::HasTonicConfig`**

```rust
fn tonic_config(&mut self) -> &mut TonicConfig
```

A builder for creating a new [MetricExporter].

---
