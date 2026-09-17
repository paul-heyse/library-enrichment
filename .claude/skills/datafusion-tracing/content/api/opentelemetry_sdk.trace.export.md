# `opentelemetry_sdk::trace::export`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.export.json`](../model/opentelemetry_sdk.trace.export.json)

## SpanData

`struct` · `opentelemetry_sdk::trace::export::SpanData`

Also reachable as `opentelemetry_sdk::trace::SpanData`

```rust
struct SpanData
```

**Fields**: `span_context`, `parent_span_id`, `parent_span_is_remote`, `span_kind`, `name`, `start_time`, `end_time`, `attributes`, `dropped_attributes_count`, `events`, `links`, `status`, `instrumentation_scope`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

`SpanData` contains all the information collected by a `Span` and can be used
by exporters as a standard input.

---

## SpanExporter

`trait` · `opentelemetry_sdk::trace::export::SpanExporter`

Also reachable as `opentelemetry_sdk::trace::SpanExporter`

```rust
trait SpanExporter: Send + Sync + Debug
```

**Implementors** (2)

- `opentelemetry_otlp::span::SpanExporter`
- `opentelemetry_sdk::trace::in_memory_exporter::InMemorySpanExporter`

**Methods** (5)

```rust
fn export(&self, batch: Vec<SpanData>) -> impl std::future::Future<Output = OTelSdkResult> + Send
fn force_flush(&mut self) -> OTelSdkResult
fn set_resource(&mut self, _resource: &Resource)
fn shutdown(&mut self) -> OTelSdkResult
fn shutdown_with_timeout(&mut self, _timeout: Duration) -> OTelSdkResult
```

`SpanExporter` defines the interface that protocol-specific exporters must
implement so that they can be plugged into OpenTelemetry SDK and support
sending of telemetry data.

The goal of the interface is to minimize burden of implementation for
protocol-dependent telemetry exporters. The protocol exporter is expected to
be primarily a simple telemetry data encoder and transmitter.

---
