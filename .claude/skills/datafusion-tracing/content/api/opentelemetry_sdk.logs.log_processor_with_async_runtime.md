# `opentelemetry_sdk::logs::log_processor_with_async_runtime`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.logs.log_processor_with_async_runtime.json`](../model/opentelemetry_sdk.logs.log_processor_with_async_runtime.json)

## BatchLogProcessor

`struct` · `opentelemetry_sdk::logs::log_processor_with_async_runtime::BatchLogProcessor`

```rust
struct BatchLogProcessor<R: RuntimeChannel>
```

**Implements**: `opentelemetry_sdk::logs::log_processor::LogProcessor`

**Derives**: Debug

**Methods** (1)

```rust
fn builder<E>(exporter: E, runtime: R) -> BatchLogProcessorBuilder<E, R> where E: LogExporter
```

**via `opentelemetry_sdk::logs::log_processor::LogProcessor`**

```rust
fn emit(&self, record: &mut SdkLogRecord, instrumentation: &InstrumentationScope)
fn force_flush(&self) -> OTelSdkResult
fn set_resource(&mut self, resource: &Resource)
fn shutdown(&self) -> OTelSdkResult
```

A [`LogProcessor`] that asynchronously buffers log records and reports
them at a pre-configured interval.

---

## BatchLogProcessorBuilder

`struct` · `opentelemetry_sdk::logs::log_processor_with_async_runtime::BatchLogProcessorBuilder`

```rust
struct BatchLogProcessorBuilder<E, R>
```

**Derives**: Debug

**Methods** (2)

```rust
fn build(self) -> BatchLogProcessor<R>
fn with_batch_config(self, config: BatchConfig) -> Self
```

A builder for creating [`BatchLogProcessor`] instances.

---
