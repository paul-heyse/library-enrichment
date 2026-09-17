# `opentelemetry_sdk::logs::export`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.logs.export.json`](../model/opentelemetry_sdk.logs.export.json)

## LogBatch

`struct` · `opentelemetry_sdk::logs::export::LogBatch`

Also reachable as `opentelemetry_sdk::logs::LogBatch`

```rust
struct LogBatch<'a>
```

**Derives**: Debug

**Methods** (2)

```rust
fn iter(&self) -> impl Iterator<Item = (&SdkLogRecord, &InstrumentationScope)>
fn new(data: &'a [(&'a SdkLogRecord, &'a InstrumentationScope)]) -> LogBatch<'a>
```

A batch of log records to be exported by a `LogExporter`.

The `LogBatch` struct holds a collection of log records along with their associated
instrumentation scopes. This structure is used to group log records together for efficient
export operations.

# Type Parameters
- `'a`: The lifetime of the references to the log records and instrumentation scopes.

---

## LogExporter

`trait` · `opentelemetry_sdk::logs::export::LogExporter`

Also reachable as `opentelemetry_sdk::logs::LogExporter`

```rust
trait LogExporter: Send + Sync + Debug
```

**Implementors** (2)

- `opentelemetry_otlp::logs::LogExporter`
- `opentelemetry_sdk::logs::in_memory_exporter::InMemoryLogExporter`

**Methods** (5)

```rust
fn event_enabled(&self, _level: Severity, _target: &str, _name: Option<&str>) -> bool
fn export(&self, batch: LogBatch<'_>) -> impl std::future::Future<Output = OTelSdkResult> + Send
fn set_resource(&mut self, _resource: &Resource)
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, _timeout: time::Duration) -> OTelSdkResult
```

`LogExporter` defines the interface that log exporters should implement.

---
