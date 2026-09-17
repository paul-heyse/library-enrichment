# `opentelemetry_sdk::logs::batch_log_processor`

Crate `opentelemetry_sdk` · 4 public items · structured records in [`model/opentelemetry_sdk.logs.batch_log_processor.json`](../model/opentelemetry_sdk.logs.batch_log_processor.json)

## BatchConfig

`struct` · `opentelemetry_sdk::logs::batch_log_processor::BatchConfig`

Also reachable as `opentelemetry_sdk::logs::BatchConfig`

```rust
struct BatchConfig
```

**Derives**: Debug, Default

Batch log processor configuration.
Use [`BatchConfigBuilder`] to configure your own instance of [`BatchConfig`].

---

## BatchConfigBuilder

`struct` · `opentelemetry_sdk::logs::batch_log_processor::BatchConfigBuilder`

Also reachable as `opentelemetry_sdk::logs::BatchConfigBuilder`

```rust
struct BatchConfigBuilder
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn build(self) -> BatchConfig
fn with_max_export_batch_size(self, max_export_batch_size: usize) -> Self
fn with_max_export_timeout(self, max_export_timeout: Duration) -> Self
fn with_max_queue_size(self, max_queue_size: usize) -> Self
fn with_scheduled_delay(self, scheduled_delay: Duration) -> Self
```

A builder for creating [`BatchConfig`] instances.

---

## BatchLogProcessor

`struct` · `opentelemetry_sdk::logs::batch_log_processor::BatchLogProcessor`

Also reachable as `opentelemetry_sdk::logs::BatchLogProcessor`

```rust
struct BatchLogProcessor
```

**Implements**: `opentelemetry_sdk::logs::log_processor::LogProcessor`

**Derives**: Debug

**Methods** (1)

```rust
fn builder<E>(exporter: E) -> BatchLogProcessorBuilder<E> where E: LogExporter
```

**via `opentelemetry_sdk::logs::log_processor::LogProcessor`**

```rust
fn emit(&self, record: &mut SdkLogRecord, instrumentation: &InstrumentationScope)
fn force_flush(&self) -> OTelSdkResult
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

The `BatchLogProcessor` collects finished logs in a buffer and exports them
in batches to the configured `LogExporter`. This processor is ideal for
high-throughput environments, as it minimizes the overhead of exporting logs
individually. It uses a **dedicated background thread** to manage and export logs
asynchronously, ensuring that the application's main execution flow is not blocked.

This processor supports the following configurations:
- **Queue size**: Maximum number of log records that can be buffered.
- **Batch size**: Maximum number of log records to include in a single export.
- **Scheduled delay**: Frequency at which the batch is exported.

When using this processor with the OTLP Exporter, the following exporter
features are supported:
- `grpc-tonic`: Requires `LoggerProvider` to be created within a tokio runtime.
- `reqwest-blocking-client`: Works with a regular `main` or `tokio::main`.

In other words, other clients like `reqwest` and `hyper` are not supported.

`BatchLogProcessor` buffers logs in memory and exports them in batches. An
export is triggered when `max_export_batch_size` is reached or every
`scheduled_delay` milliseconds. Users can explicitly trigger an export using
the `force_flush` method. Shutdown also triggers an export of all buffered
logs and is recommended to be called before the application exits to ensure
all buffered logs are exported.

**Warning**: When using tokio's current-thread runtime, `shutdown()`, which
is a blocking call ,should not be called from your main thread. This can
cause deadlock. Instead, call `shutdown()` from a separate thread or use
tokio's `spawn_blocking`.


### Using a BatchLogProcessor:

```rust
use opentelemetry_sdk::logs::{BatchLogProcessor, BatchConfigBuilder, SdkLoggerProvider};
use opentelemetry::global;
use std::time::Duration;
use opentelemetry_sdk::logs::InMemoryLogExporter;

let exporter = InMemoryLogExporter::default(); // Replace with an actual exporter
let processor = BatchLogProcessor::builder(exporter)
    .with_batch_config(
        BatchConfigBuilder::default()
            .with_max_queue_size(2048)
            .with_max_export_batch_size(512)
            .with_scheduled_delay(Duration::from_secs(5))
            .build(),
    )
    .build();

let provider = SdkLoggerProvider::builder()
    .with_log_processor(processor)
    .build();

---

## BatchLogProcessorBuilder

`struct` · `opentelemetry_sdk::logs::batch_log_processor::BatchLogProcessorBuilder`

Also reachable as `opentelemetry_sdk::logs::BatchLogProcessorBuilder`

```rust
struct BatchLogProcessorBuilder<E>
```

**Derives**: Debug

**Methods** (2)

```rust
fn build(self) -> BatchLogProcessor
fn with_batch_config(self, config: BatchConfig) -> Self
```


A builder for creating [`BatchLogProcessor`] instances.

---
