# `opentelemetry_otlp::exporter::http`

Crate `opentelemetry-otlp` · 4 public items · structured records in [`model/opentelemetry_otlp.exporter.http.json`](../model/opentelemetry_otlp.exporter.http.json)

## HttpConfig

`struct` · `opentelemetry_otlp::exporter::http::HttpConfig`

```rust
struct HttpConfig
```

Configuration of the http transport

---

## HttpExporterBuilder

`struct` · `opentelemetry_otlp::exporter::http::HttpExporterBuilder`

Also reachable as `opentelemetry_otlp::HttpExporterBuilder`

```rust
struct HttpExporterBuilder
```

**Implements**: `opentelemetry_otlp::exporter::HasExportConfig`, `opentelemetry_otlp::exporter::http::HasHttpConfig`

**Derives**: Debug, Default

**Methods** (3)

```rust
fn build_log_exporter(self) -> Result<LogExporter, ExporterBuildError>
fn build_metrics_exporter(self, temporality: opentelemetry_sdk::metrics::Temporality) -> Result<MetricExporter, ExporterBuildError>
fn build_span_exporter(self) -> Result<SpanExporter, ExporterBuildError>
```

**via `opentelemetry_otlp::exporter::HasExportConfig`**

```rust
fn export_config(&mut self) -> &mut ExportConfig
```

**via `opentelemetry_otlp::exporter::http::HasHttpConfig`**

```rust
fn http_client_config(&mut self) -> &mut HttpConfig
```

Configuration for the OTLP HTTP exporter.

## Examples

```no_run
# #[cfg(feature="metrics")]
use opentelemetry_sdk::metrics::Temporality;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Create a span exporter you can use when configuring tracer providers
# #[cfg(feature="trace")]
let span_exporter = opentelemetry_otlp::SpanExporter::builder().with_http().build()?;

// Create a metrics exporter you can use when configuring meter providers
# #[cfg(feature="metrics")]
let metrics_exporter = opentelemetry_otlp::MetricExporter::builder()
    .with_http()
    .with_temporality(Temporality::default())
    .build()?;

// Create a log exporter you can use when configuring logger providers
# #[cfg(feature="logs")]
let log_exporter = opentelemetry_otlp::LogExporter::builder().with_http().build()?;
# Ok(())
# }
```

---

## HasHttpConfig

`trait` · `opentelemetry_otlp::exporter::http::HasHttpConfig`

Also reachable as `opentelemetry_otlp::HasHttpConfig`

```rust
trait HasHttpConfig
```

**Implementors** (4)

- `opentelemetry_otlp::exporter::http::HttpExporterBuilder`
- `opentelemetry_otlp::logs::LogExporterBuilder`
- `opentelemetry_otlp::metric::MetricExporterBuilder`
- `opentelemetry_otlp::span::SpanExporterBuilder`

**Methods** (1)

```rust
fn http_client_config(&mut self) -> &mut HttpConfig
```

Expose interface for modifying builder config.

---

## WithHttpConfig

`trait` · `opentelemetry_otlp::exporter::http::WithHttpConfig`

Also reachable as `opentelemetry_otlp::WithHttpConfig`

```rust
trait WithHttpConfig
```

**Methods** (3)

```rust
fn with_compression(self, compression: Compression) -> Self
fn with_headers(self, headers: HashMap<String, String>) -> Self
fn with_http_client<T: HttpClient + 'static>(self, client: T) -> Self
```

This trait will be implemented for every struct that implemented [`HasHttpConfig`] trait.

## Examples
```
# #[cfg(all(feature = "trace", feature = "grpc-tonic"))]
# {
use crate::opentelemetry_otlp::WithHttpConfig;
let exporter_builder = opentelemetry_otlp::SpanExporter::builder()
    .with_http()
    .with_headers(std::collections::HashMap::new());
# }
```

---
