# `opentelemetry_otlp::logs`

Crate `opentelemetry-otlp` · 6 public items · structured records in [`model/opentelemetry_otlp.logs.json`](../model/opentelemetry_otlp.logs.json)

## OTEL_EXPORTER_OTLP_LOGS_COMPRESSION

`constant` · `opentelemetry_otlp::logs::OTEL_EXPORTER_OTLP_LOGS_COMPRESSION`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_LOGS_COMPRESSION`

```rust
const OTEL_EXPORTER_OTLP_LOGS_COMPRESSION: &str = "OTEL_EXPORTER_OTLP_LOGS_COMPRESSION"
```

Compression algorithm to use, defaults to none.

---

## OTEL_EXPORTER_OTLP_LOGS_ENDPOINT

`constant` · `opentelemetry_otlp::logs::OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`

```rust
const OTEL_EXPORTER_OTLP_LOGS_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT"
```

Target to which the exporter is going to send logs

---

## OTEL_EXPORTER_OTLP_LOGS_HEADERS

`constant` · `opentelemetry_otlp::logs::OTEL_EXPORTER_OTLP_LOGS_HEADERS`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_LOGS_HEADERS`

```rust
const OTEL_EXPORTER_OTLP_LOGS_HEADERS: &str = "OTEL_EXPORTER_OTLP_LOGS_HEADERS"
```

Key-value pairs to be used as headers associated with gRPC or HTTP requests
for sending logs.
Example: `k1=v1,k2=v2`
Note: this is only supported for HTTP.

---

## OTEL_EXPORTER_OTLP_LOGS_TIMEOUT

`constant` · `opentelemetry_otlp::logs::OTEL_EXPORTER_OTLP_LOGS_TIMEOUT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_LOGS_TIMEOUT`

```rust
const OTEL_EXPORTER_OTLP_LOGS_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_LOGS_TIMEOUT"
```

Maximum time the OTLP exporter will wait for each batch logs export.

---

## LogExporter

`struct` · `opentelemetry_otlp::logs::LogExporter`

Also reachable as `opentelemetry_otlp::LogExporter`

```rust
struct LogExporter
```

**Implements**: `opentelemetry_sdk::logs::export::LogExporter`

**Derives**: Debug

**Methods** (1)

```rust
fn builder() -> LogExporterBuilder<NoExporterBuilderSet>
```

**via `opentelemetry_sdk::logs::export::LogExporter`**

```rust
async fn export(&self, batch: LogBatch<'_>) -> OTelSdkResult
fn set_resource(&mut self, resource: &opentelemetry_sdk::Resource)
fn shutdown_with_timeout(&self, _timeout: time::Duration) -> OTelSdkResult
```

OTLP exporter that sends log data

---

## LogExporterBuilder

`struct` · `opentelemetry_otlp::logs::LogExporterBuilder`

Also reachable as `opentelemetry_otlp::LogExporterBuilder`

```rust
struct LogExporterBuilder<C>
```

**Implements**: `opentelemetry_otlp::exporter::HasExportConfig`, `opentelemetry_otlp::exporter::http::HasHttpConfig`, `opentelemetry_otlp::exporter::tonic::HasTonicConfig`

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn build(self) -> Result<LogExporter, ExporterBuildError>
fn build(self) -> Result<LogExporter, ExporterBuildError>
fn new() -> Self
fn with_http(self) -> LogExporterBuilder<HttpExporterBuilderSet>
fn with_tonic(self) -> LogExporterBuilder<TonicExporterBuilderSet>
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

Builder for creating a new [LogExporter].

---
