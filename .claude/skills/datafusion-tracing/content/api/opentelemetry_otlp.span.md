# `opentelemetry_otlp::span`

Crate `opentelemetry-otlp` · 6 public items · structured records in [`model/opentelemetry_otlp.span.json`](../model/opentelemetry_otlp.span.json)

## OTEL_EXPORTER_OTLP_TRACES_COMPRESSION

`constant` · `opentelemetry_otlp::span::OTEL_EXPORTER_OTLP_TRACES_COMPRESSION`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_COMPRESSION`

```rust
const OTEL_EXPORTER_OTLP_TRACES_COMPRESSION: &str = "OTEL_EXPORTER_OTLP_TRACES_COMPRESSION"
```

Compression algorithm to use, defaults to none.

---

## OTEL_EXPORTER_OTLP_TRACES_ENDPOINT

`constant` · `opentelemetry_otlp::span::OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`

```rust
const OTEL_EXPORTER_OTLP_TRACES_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT"
```

Target to which the exporter is going to send spans, defaults to https://localhost:4317/v1/traces.
Learn about the relationship between this constant and default/metrics/logs at
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#endpoint-urls-for-otlphttp>

---

## OTEL_EXPORTER_OTLP_TRACES_HEADERS

`constant` · `opentelemetry_otlp::span::OTEL_EXPORTER_OTLP_TRACES_HEADERS`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_HEADERS`

```rust
const OTEL_EXPORTER_OTLP_TRACES_HEADERS: &str = "OTEL_EXPORTER_OTLP_TRACES_HEADERS"
```

Key-value pairs to be used as headers associated with gRPC or HTTP requests
for sending spans.
Example: `k1=v1,k2=v2`
Note: this is only supported for HTTP.

---

## OTEL_EXPORTER_OTLP_TRACES_TIMEOUT

`constant` · `opentelemetry_otlp::span::OTEL_EXPORTER_OTLP_TRACES_TIMEOUT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_TIMEOUT`

```rust
const OTEL_EXPORTER_OTLP_TRACES_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_TRACES_TIMEOUT"
```

Max waiting time for the backend to process each spans batch, defaults to 10s.

---

## SpanExporter

`struct` · `opentelemetry_otlp::span::SpanExporter`

Also reachable as `opentelemetry_otlp::SpanExporter`

```rust
struct SpanExporter
```

**Implements**: `opentelemetry_sdk::trace::export::SpanExporter`

**Derives**: Debug

**Methods** (1)

```rust
fn builder() -> SpanExporterBuilder<NoExporterBuilderSet>
```

**via `opentelemetry_sdk::trace::export::SpanExporter`**

```rust
async fn export(&self, batch: Vec<SpanData>) -> OTelSdkResult
fn set_resource(&mut self, resource: &opentelemetry_sdk::Resource)
```

OTLP exporter that sends tracing data

---

## SpanExporterBuilder

`struct` · `opentelemetry_otlp::span::SpanExporterBuilder`

Also reachable as `opentelemetry_otlp::SpanExporterBuilder`

```rust
struct SpanExporterBuilder<C>
```

**Implements**: `opentelemetry_otlp::exporter::HasExportConfig`, `opentelemetry_otlp::exporter::http::HasHttpConfig`, `opentelemetry_otlp::exporter::tonic::HasTonicConfig`

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn build(self) -> Result<SpanExporter, ExporterBuildError>
fn build(self) -> Result<SpanExporter, ExporterBuildError>
fn new() -> Self
fn with_http(self) -> SpanExporterBuilder<HttpExporterBuilderSet>
fn with_tonic(self) -> SpanExporterBuilder<TonicExporterBuilderSet>
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

OTLP span exporter builder

---
