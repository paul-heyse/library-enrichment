# `opentelemetry_sdk::logs::log_processor`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.logs.log_processor.json`](../model/opentelemetry_sdk.logs.log_processor.json)

## LogProcessor

`trait` · `opentelemetry_sdk::logs::log_processor::LogProcessor`

Also reachable as `opentelemetry_sdk::logs::LogProcessor`

```rust
trait LogProcessor: Send + Sync + Debug
```

**Implementors** (4)

- `opentelemetry_sdk::logs::batch_log_processor::BatchLogProcessor`
- `opentelemetry_sdk::logs::concurrent_log_processor::SimpleConcurrentLogProcessor`
- `opentelemetry_sdk::logs::log_processor_with_async_runtime::BatchLogProcessor`
- `opentelemetry_sdk::logs::simple_log_processor::SimpleLogProcessor`

**Methods** (6)

```rust
fn emit(&self, data: &mut SdkLogRecord, instrumentation: &InstrumentationScope)
fn event_enabled(&self, _level: Severity, _target: &str, _name: Option<&str>) -> bool
fn force_flush(&self) -> OTelSdkResult
fn set_resource(&mut self, _resource: &Resource)
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
```

The interface for plugging into a [`SdkLogger`].

[`SdkLogger`]: crate::logs::SdkLogger

---
