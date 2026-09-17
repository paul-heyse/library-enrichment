# `opentelemetry_sdk::trace::span_processor`

Crate `opentelemetry_sdk` · 6 public items · structured records in [`model/opentelemetry_sdk.trace.span_processor.json`](../model/opentelemetry_sdk.trace.span_processor.json)

## BatchConfig

`struct` · `opentelemetry_sdk::trace::span_processor::BatchConfig`

Also reachable as `opentelemetry_sdk::trace::BatchConfig`

```rust
struct BatchConfig
```

**Derives**: Debug, Default

Batch span processor configuration.
Use [`BatchConfigBuilder`] to configure your own instance of [`BatchConfig`].

---

## BatchConfigBuilder

`struct` · `opentelemetry_sdk::trace::span_processor::BatchConfigBuilder`

Also reachable as `opentelemetry_sdk::trace::BatchConfigBuilder`

```rust
struct BatchConfigBuilder
```

**Derives**: Debug, Default

**Methods** (6)

```rust
fn build(self) -> BatchConfig
fn with_max_concurrent_exports(self, max_concurrent_exports: usize) -> Self
fn with_max_export_batch_size(self, max_export_batch_size: usize) -> Self
fn with_max_export_timeout(self, max_export_timeout: Duration) -> Self
fn with_max_queue_size(self, max_queue_size: usize) -> Self
fn with_scheduled_delay(self, scheduled_delay: Duration) -> Self
```

A builder for creating [`BatchConfig`] instances.

---

## BatchSpanProcessor

`struct` · `opentelemetry_sdk::trace::span_processor::BatchSpanProcessor`

Also reachable as `opentelemetry_sdk::trace::BatchSpanProcessor`

```rust
struct BatchSpanProcessor
```

**Implements**: `opentelemetry_sdk::trace::span_processor::SpanProcessor`

**Derives**: Debug

**Methods** (2)

```rust
fn builder<E>(exporter: E) -> BatchSpanProcessorBuilder<E> where E: SpanExporter + Send + 'static
fn new<E>(exporter: E, config: BatchConfig) -> Self where E: SpanExporter + Send + 'static
```

**via `opentelemetry_sdk::trace::span_processor::SpanProcessor`**

```rust
fn force_flush(&self) -> OTelSdkResult
fn on_end(&self, span: SpanData)
fn on_start(&self, _span: &mut Span, _cx: &Context)
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

The `BatchSpanProcessor` collects finished spans in a buffer and exports them
in batches to the configured `SpanExporter`. This processor is ideal for
high-throughput environments, as it minimizes the overhead of exporting spans
individually. It uses a **dedicated background thread** to manage and export spans
asynchronously, ensuring that the application's main execution flow is not blocked.

This processor supports the following configurations:
- **Queue size**: Maximum number of spans that can be buffered.
- **Batch size**: Maximum number of spans to include in a single export.
- **Scheduled delay**: Frequency at which the batch is exported.

When using this processor with the OTLP Exporter, the following exporter
features are supported:
- `grpc-tonic`: Requires `TracerProvider` to be created within a tokio runtime.
- `reqwest-blocking-client`: Works with a regular `main` or `tokio::main`.

In other words, other clients like `reqwest` and `hyper` are not supported.

`BatchSpanProcessor` buffers spans in memory and exports them in batches. An
export is triggered when `max_export_batch_size` is reached or every
`scheduled_delay` milliseconds. Users can explicitly trigger an export using
the `force_flush` method. Shutdown also triggers an export of all buffered
spans and is recommended to be called before the application exits to ensure
all buffered spans are exported.

**Warning**: When using tokio's current-thread runtime, `shutdown()`, which
is a blocking call ,should not be called from your main thread. This can
cause deadlock. Instead, call `shutdown()` from a separate thread or use
tokio's `spawn_blocking`.

---

## BatchSpanProcessorBuilder

`struct` · `opentelemetry_sdk::trace::span_processor::BatchSpanProcessorBuilder`

Also reachable as `opentelemetry_sdk::trace::BatchSpanProcessorBuilder`

```rust
struct BatchSpanProcessorBuilder<E> where E: SpanExporter + Send + 'static
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn build(self) -> BatchSpanProcessor
fn with_batch_config(self, config: BatchConfig) -> Self
```

Builder for `BatchSpanProcessorDedicatedThread`.

---

## SimpleSpanProcessor

`struct` · `opentelemetry_sdk::trace::span_processor::SimpleSpanProcessor`

Also reachable as `opentelemetry_sdk::trace::SimpleSpanProcessor`

```rust
struct SimpleSpanProcessor<T: SpanExporter>
```

**Implements**: `opentelemetry_sdk::trace::span_processor::SpanProcessor`

**Derives**: Debug

**Methods** (1)

```rust
fn new(exporter: T) -> Self
```

**via `opentelemetry_sdk::trace::span_processor::SpanProcessor`**

```rust
fn force_flush(&self) -> OTelSdkResult
fn on_end(&self, span: SpanData)
fn on_start(&self, _span: &mut Span, _cx: &Context)
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

A [SpanProcessor] that passes finished spans to the configured
`SpanExporter`, as soon as they are finished, without any batching. This is
typically useful for debugging and testing. For scenarios requiring higher
performance/throughput, consider using [BatchSpanProcessor].
Spans are exported synchronously
in the same thread that emits the log record.
When using this processor with the OTLP Exporter, the following exporter
features are supported:
- `grpc-tonic`: This requires TracerProvider to be created within a tokio
  runtime. Spans can be emitted from any thread, including tokio runtime
  threads.
- `reqwest-blocking-client`: TracerProvider may be created anywhere, but
  spans must be emitted from a non-tokio runtime thread.
- `reqwest-client`: TracerProvider may be created anywhere, but spans must be
  emitted from a tokio runtime thread.

---

## SpanProcessor

`trait` · `opentelemetry_sdk::trace::span_processor::SpanProcessor`

Also reachable as `opentelemetry_sdk::trace::SpanProcessor`

```rust
trait SpanProcessor: Send + Sync + std::fmt::Debug
```

**Implementors** (3)

- `opentelemetry_sdk::trace::span_processor::BatchSpanProcessor`
- `opentelemetry_sdk::trace::span_processor::SimpleSpanProcessor`
- `opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor`

**Methods** (6)

```rust
fn force_flush(&self) -> OTelSdkResult
fn on_end(&self, span: SpanData)
fn on_start(&self, span: &mut Span, cx: &Context)
fn set_resource(&mut self, _resource: &Resource)
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

`SpanProcessor` is an interface which allows hooks for span start and end
method invocations. The span processors are invoked only when is_recording
is true.

---
