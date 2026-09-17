# `opentelemetry_otlp::exporter`

Crate `opentelemetry-otlp` · 13 public items · structured records in [`model/opentelemetry_otlp.exporter.json`](../model/opentelemetry_otlp.exporter.json)

## OTEL_EXPORTER_OTLP_COMPRESSION

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_COMPRESSION`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_COMPRESSION`

```rust
const OTEL_EXPORTER_OTLP_COMPRESSION: &str = "OTEL_EXPORTER_OTLP_COMPRESSION"
```

Compression algorithm to use, defaults to none.

---

## OTEL_EXPORTER_OTLP_ENDPOINT

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_ENDPOINT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_ENDPOINT`

```rust
const OTEL_EXPORTER_OTLP_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT"
```

Target to which the exporter is going to send signals, defaults to https://localhost:4317.
Learn about the relationship between this constant and metrics/spans/logs at
<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#endpoint-urls-for-otlphttp>

---

## OTEL_EXPORTER_OTLP_ENDPOINT_DEFAULT

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_ENDPOINT_DEFAULT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_ENDPOINT_DEFAULT`

```rust
const OTEL_EXPORTER_OTLP_ENDPOINT_DEFAULT: &str = OTEL_EXPORTER_OTLP_HTTP_ENDPOINT_DEFAULT
```

Default target to which the exporter is going to send signals.

---

## OTEL_EXPORTER_OTLP_HEADERS

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_HEADERS`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_HEADERS`

```rust
const OTEL_EXPORTER_OTLP_HEADERS: &str = "OTEL_EXPORTER_OTLP_HEADERS"
```

Key-value pairs to be used as headers associated with gRPC or HTTP requests
Example: `k1=v1,k2=v2`
Note: as of now, this is only supported for HTTP requests.

---

## OTEL_EXPORTER_OTLP_PROTOCOL

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_PROTOCOL`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_PROTOCOL`

```rust
const OTEL_EXPORTER_OTLP_PROTOCOL: &str = "OTEL_EXPORTER_OTLP_PROTOCOL"
```

Protocol the exporter will use. Either `http/protobuf` or `grpc`.

---

## OTEL_EXPORTER_OTLP_PROTOCOL_DEFAULT

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_PROTOCOL_DEFAULT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_PROTOCOL_DEFAULT`

```rust
const OTEL_EXPORTER_OTLP_PROTOCOL_DEFAULT: &str = OTEL_EXPORTER_OTLP_PROTOCOL_HTTP_JSON
```

Default protocol, using http-json.

---

## OTEL_EXPORTER_OTLP_TIMEOUT

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_TIMEOUT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT`

```rust
const OTEL_EXPORTER_OTLP_TIMEOUT: &str = "OTEL_EXPORTER_OTLP_TIMEOUT"
```

Max waiting time for the backend to process each signal batch, defaults to 10 seconds.

---

## OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT

`constant` · `opentelemetry_otlp::exporter::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT`

Also reachable as `opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT`

```rust
const OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT: std::time::Duration = _
```

Default max waiting time for the backend to process each signal batch.

---

## Compression

`enum` · `opentelemetry_otlp::exporter::Compression`

Also reachable as `opentelemetry_otlp::Compression`

```rust
enum Compression
```

**Variants**: `Gzip`, `Zstd`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private226::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private226::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The compression algorithm to use when sending data.

---

## ExporterBuildError

`enum` · `opentelemetry_otlp::exporter::ExporterBuildError`

Also reachable as `opentelemetry_otlp::ExporterBuildError`

```rust
enum ExporterBuildError
```

**Variants**: `ThreadSpawnFailed`, `NoHttpClient`, `UnsupportedCompressionAlgorithm`, `InvalidUri`, `InternalFailure`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Errors that can occur while building an exporter.

---

## ExportConfig

`struct` · `opentelemetry_otlp::exporter::ExportConfig`

Also reachable as `opentelemetry_otlp::ExportConfig`

```rust
struct ExportConfig
```

**Fields**: `endpoint`, `protocol`, `timeout`

**Derives**: Debug, Default

Configuration for the OTLP exporter.

---

## HasExportConfig

`trait` · `opentelemetry_otlp::exporter::HasExportConfig`

Also reachable as `opentelemetry_otlp::HasExportConfig`

```rust
trait HasExportConfig
```

**Implementors** (5)

- `opentelemetry_otlp::exporter::http::HttpExporterBuilder`
- `opentelemetry_otlp::exporter::tonic::TonicExporterBuilder`
- `opentelemetry_otlp::logs::LogExporterBuilder`
- `opentelemetry_otlp::metric::MetricExporterBuilder`
- `opentelemetry_otlp::span::SpanExporterBuilder`

**Methods** (1)

```rust
fn export_config(&mut self) -> &mut ExportConfig
```

Provide access to the [ExportConfig] field within the exporter builders.

---

## WithExportConfig

`trait` · `opentelemetry_otlp::exporter::WithExportConfig`

Also reachable as `opentelemetry_otlp::WithExportConfig`

```rust
trait WithExportConfig
```

**Methods** (4)

```rust
fn with_endpoint<T: Into<String>>(self, endpoint: T) -> Self
fn with_export_config(self, export_config: ExportConfig) -> Self
fn with_protocol(self, protocol: Protocol) -> Self
fn with_timeout(self, timeout: Duration) -> Self
```

Expose methods to override [ExportConfig].

This trait will be implemented for every struct that implemented [`HasExportConfig`] trait.

## Examples
```
# #[cfg(all(feature = "trace", feature = "grpc-tonic"))]
# {
use crate::opentelemetry_otlp::WithExportConfig;
let exporter_builder = opentelemetry_otlp::SpanExporter::builder()
    .with_tonic()
    .with_endpoint("http://localhost:7201");
# }
```

---
