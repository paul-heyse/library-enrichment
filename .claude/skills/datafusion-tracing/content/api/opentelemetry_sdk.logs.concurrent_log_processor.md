# `opentelemetry_sdk::logs::concurrent_log_processor`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.logs.concurrent_log_processor.json`](../model/opentelemetry_sdk.logs.concurrent_log_processor.json)

## SimpleConcurrentLogProcessor

`struct` · `opentelemetry_sdk::logs::concurrent_log_processor::SimpleConcurrentLogProcessor`

```rust
struct SimpleConcurrentLogProcessor<T: LogExporter>
```

**Implements**: `opentelemetry_sdk::logs::log_processor::LogProcessor`

**Derives**: Debug

**Methods** (1)

```rust
fn new(exporter: T) -> Self
```

**via `opentelemetry_sdk::logs::log_processor::LogProcessor`**

```rust
fn emit(&self, record: &mut SdkLogRecord, instrumentation: &InstrumentationScope)
fn event_enabled(&self, level: opentelemetry::logs::Severity, target: &str, name: Option<&str>) -> bool
fn force_flush(&self) -> OTelSdkResult
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

A concurrent log processor calls exporter's export method on each emit. This
processor does not buffer logs. Note: This invokes exporter's export method
on the current thread without synchronization. i.e multiple export() calls
can happen simultaneously from different threads. This is not a problem if
the exporter is designed to handle that. As of now, exporters in the
opentelemetry-rust project (stdout/otlp) are not thread-safe.
This is intended to be used when exporting to operating system
tracing facilities like Windows ETW, Linux TracePoints etc.

---
