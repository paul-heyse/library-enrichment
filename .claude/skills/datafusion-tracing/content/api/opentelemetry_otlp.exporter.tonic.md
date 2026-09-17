# `opentelemetry_otlp::exporter::tonic`

Crate `opentelemetry-otlp` · 4 public items · structured records in [`model/opentelemetry_otlp.exporter.tonic.json`](../model/opentelemetry_otlp.exporter.tonic.json)

## TonicConfig

`struct` · `opentelemetry_otlp::exporter::tonic::TonicConfig`

Also reachable as `opentelemetry_otlp::TonicConfig`

```rust
struct TonicConfig
```

**Derives**: Debug, Default

Configuration for [tonic]

[tonic]: https://github.com/hyperium/tonic

---

## TonicExporterBuilder

`struct` · `opentelemetry_otlp::exporter::tonic::TonicExporterBuilder`

Also reachable as `opentelemetry_otlp::TonicExporterBuilder`

```rust
struct TonicExporterBuilder
```

**Implements**: `opentelemetry_otlp::exporter::HasExportConfig`, `opentelemetry_otlp::exporter::tonic::HasTonicConfig`

**Derives**: Debug, Default

**via `opentelemetry_otlp::exporter::HasExportConfig`**

```rust
fn export_config(&mut self) -> &mut ExportConfig
```

**via `opentelemetry_otlp::exporter::tonic::HasTonicConfig`**

```rust
fn tonic_config(&mut self) -> &mut TonicConfig
```

Configuration for the [tonic] OTLP GRPC exporter.

It allows you to
- add additional metadata
- set tls config (via the  `tls` feature)
- specify custom [channel]s

[tonic]: <https://github.com/hyperium/tonic>
[channel]: tonic::transport::Channel

## Examples

```no_run
# #[cfg(feature="metrics")]
use opentelemetry_sdk::metrics::Temporality;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// Create a span exporter you can use to when configuring tracer providers
# #[cfg(feature="trace")]
let span_exporter = opentelemetry_otlp::SpanExporter::builder().with_tonic().build()?;

// Create a metric exporter you can use when configuring meter providers
# #[cfg(feature="metrics")]
let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
    .with_tonic()
    .with_temporality(Temporality::default())
    .build()?;

// Create a log exporter you can use when configuring logger providers
# #[cfg(feature="logs")]
let log_exporter = opentelemetry_otlp::LogExporter::builder().with_tonic().build()?;
# Ok(())
# }
```

---

## HasTonicConfig

`trait` · `opentelemetry_otlp::exporter::tonic::HasTonicConfig`

Also reachable as `opentelemetry_otlp::HasTonicConfig`

```rust
trait HasTonicConfig
```

**Implementors** (4)

- `opentelemetry_otlp::exporter::tonic::TonicExporterBuilder`
- `opentelemetry_otlp::logs::LogExporterBuilder`
- `opentelemetry_otlp::metric::MetricExporterBuilder`
- `opentelemetry_otlp::span::SpanExporterBuilder`

**Methods** (1)

```rust
fn tonic_config(&mut self) -> &mut TonicConfig
```

Expose interface for modifying [TonicConfig] fields within the exporter builders.

---

## WithTonicConfig

`trait` · `opentelemetry_otlp::exporter::tonic::WithTonicConfig`

Also reachable as `opentelemetry_otlp::WithTonicConfig`

```rust
trait WithTonicConfig
```

**Methods** (5)

```rust
fn with_channel(self, channel: tonic::transport::Channel) -> Self
fn with_compression(self, compression: Compression) -> Self
fn with_interceptor<I>(self, interceptor: I) -> Self where I: tonic::service::Interceptor + Clone + Send + Sync + 'static
fn with_metadata(self, metadata: MetadataMap) -> Self
fn with_tls_config(self, tls_config: ClientTlsConfig) -> Self
```

Expose methods to override [TonicConfig].

This trait will be implemented for every struct that implemented [`HasTonicConfig`] trait.

## Examples
```
# #[cfg(all(feature = "trace", feature = "grpc-tonic"))]
# {
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
let exporter_builder = opentelemetry_otlp::SpanExporter::builder()
    .with_tonic()
    .with_compression(opentelemetry_otlp::Compression::Gzip);
# }
```

---
