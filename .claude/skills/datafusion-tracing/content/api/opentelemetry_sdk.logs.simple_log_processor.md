# `opentelemetry_sdk::logs::simple_log_processor`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.logs.simple_log_processor.json`](../model/opentelemetry_sdk.logs.simple_log_processor.json)

## SimpleLogProcessor

`struct` · `opentelemetry_sdk::logs::simple_log_processor::SimpleLogProcessor`

Also reachable as `opentelemetry_sdk::logs::SimpleLogProcessor`

```rust
struct SimpleLogProcessor<T: LogExporter>
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
fn shutdown(&self) -> OTelSdkResult
```

A [`LogProcessor`] designed for testing and debugging purpose, that immediately
exports log records as they are emitted. Log records are exported synchronously
in the same thread that emits the log record.
When using this processor with the OTLP Exporter, the following exporter
features are supported:
- `grpc-tonic`: This requires LoggerProvider to be created within a tokio
  runtime. Logs can be emitted from any thread, including tokio runtime
  threads.
- `reqwest-blocking-client`: LoggerProvider may be created anywhere, but
  logs must be emitted from a non-tokio runtime thread.
- `reqwest-client`: LoggerProvider may be created anywhere, but logs must be
  emitted from a tokio runtime thread.

## Example

### Using a SimpleLogProcessor

```rust
use opentelemetry_sdk::logs::{SimpleLogProcessor, SdkLoggerProvider, LogExporter};
use opentelemetry::global;
use opentelemetry_sdk::logs::InMemoryLogExporter;

let exporter = InMemoryLogExporter::default(); // Replace with an actual exporter
let provider = SdkLoggerProvider::builder()
    .with_simple_exporter(exporter)
    .build();

```

---
