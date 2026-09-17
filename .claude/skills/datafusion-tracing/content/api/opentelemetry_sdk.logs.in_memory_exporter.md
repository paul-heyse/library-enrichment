# `opentelemetry_sdk::logs::in_memory_exporter`

Crate `opentelemetry_sdk` · 4 public items · structured records in [`model/opentelemetry_sdk.logs.in_memory_exporter.json`](../model/opentelemetry_sdk.logs.in_memory_exporter.json)

## InMemoryLogExporter

`struct` · `opentelemetry_sdk::logs::in_memory_exporter::InMemoryLogExporter`

Also reachable as `opentelemetry_sdk::logs::InMemoryLogExporter`

```rust
struct InMemoryLogExporter
```

**Implements**: `opentelemetry_sdk::logs::export::LogExporter`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn get_emitted_logs(&self) -> Result<Vec<LogDataWithResource>, InMemoryExporterError>
fn is_shutdown_called(&self) -> bool
fn reset(&self)
```

**via `opentelemetry_sdk::logs::export::LogExporter`**

```rust
async fn export(&self, batch: LogBatch<'_>) -> OTelSdkResult
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&self, _timeout: time::Duration) -> OTelSdkResult
```

 An in-memory logs exporter that stores logs data in memory.

 This exporter is useful for testing and debugging purposes.
 It stores logs in a `Vec<OwnedLogData>`. Logs can be retrieved using
 `get_emitted_logs` method.

 # Example
 ```no_run
# use opentelemetry_sdk::logs::{BatchLogProcessor, SdkLoggerProvider};
# use opentelemetry_sdk::runtime;
# use opentelemetry_sdk::logs::InMemoryLogExporter;

# #[tokio::main]
# async fn main() {
    // Create an InMemoryLogExporter
    let exporter: InMemoryLogExporter = InMemoryLogExporter::default();
    //Create a LoggerProvider and register the exporter
    let logger_provider = SdkLoggerProvider::builder()
        .with_log_processor(BatchLogProcessor::builder(exporter.clone()).build())
        .build();
    // Setup Log Appenders and emit logs. (Not shown here)
    logger_provider.force_flush();
    let emitted_logs = exporter.get_emitted_logs().unwrap();
    for log in emitted_logs {
        println!("{:?}", log);
    }
# }
 ```

---

## InMemoryLogExporterBuilder

`struct` · `opentelemetry_sdk::logs::in_memory_exporter::InMemoryLogExporterBuilder`

Also reachable as `opentelemetry_sdk::logs::InMemoryLogExporterBuilder`

```rust
struct InMemoryLogExporterBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn build(&self) -> InMemoryLogExporter
fn new() -> Self
```

Builder for [`InMemoryLogExporter`].
 # Example

 ```no_run
# use opentelemetry_sdk::logs::{InMemoryLogExporter, InMemoryLogExporterBuilder};
# use opentelemetry_sdk::logs::{BatchLogProcessor, SdkLoggerProvider};
# use opentelemetry_sdk::runtime;

# #[tokio::main]
# async fn main() {
    //Create an InMemoryLogExporter
    let exporter: InMemoryLogExporter = InMemoryLogExporterBuilder::default().build();
    //Create a LoggerProvider and register the exporter
    let logger_provider = SdkLoggerProvider::builder()
        .with_log_processor(BatchLogProcessor::builder(exporter.clone()).build())
        .build();
    // Setup Log Appenders and emit logs. (Not shown here)
    logger_provider.force_flush();
    let emitted_logs = exporter.get_emitted_logs().unwrap();
    for log in emitted_logs {
        println!("{:?}", log);
    }
# }

 ```

---

## LogDataWithResource

`struct` · `opentelemetry_sdk::logs::in_memory_exporter::LogDataWithResource`

```rust
struct LogDataWithResource
```

**Fields**: `record`, `instrumentation`, `resource`

**Derives**: Clone, Debug

`LogDataWithResource` associates a [`SdkLogRecord`] with a [`Resource`] and
[`InstrumentationScope`].

---

## OwnedLogData

`struct` · `opentelemetry_sdk::logs::in_memory_exporter::OwnedLogData`

```rust
struct OwnedLogData
```

**Fields**: `record`, `instrumentation`

**Derives**: Clone, Debug

`OwnedLogData` represents a single log event without resource context.

---
