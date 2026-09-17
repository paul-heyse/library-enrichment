# `opentelemetry_sdk::trace::span_processor_with_async_runtime`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.span_processor_with_async_runtime.json`](../model/opentelemetry_sdk.trace.span_processor_with_async_runtime.json)

## BatchSpanProcessor

`struct` · `opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor`

```rust
struct BatchSpanProcessor<R: RuntimeChannel>
```

**Implements**: `opentelemetry_sdk::trace::span_processor::SpanProcessor`

**Derives**: Debug

**Methods** (1)

```rust
fn builder<E>(exporter: E, runtime: R) -> BatchSpanProcessorBuilder<E, R> where E: SpanExporter
```

**via `opentelemetry_sdk::trace::span_processor::SpanProcessor`**

```rust
fn force_flush(&self) -> OTelSdkResult
fn on_end(&self, span: SpanData)
fn on_start(&self, _span: &mut Span, _cx: &Context)
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
```

A [`SpanProcessor`] that asynchronously buffers finished spans and reports
them at a preconfigured interval.

Batch span processors need to run a background task to collect and send
spans. Different runtimes need different ways to handle the background task.

Note: Configuring an opentelemetry `Runtime` that's not compatible with the
underlying runtime can cause deadlocks (see tokio section).

### Use with Tokio

Tokio currently offers two different schedulers. One is
`current_thread_scheduler`, the other is `multiple_thread_scheduler`. Both
of them default to use batch span processors to install span exporters.

Tokio's `current_thread_scheduler` can cause the program to hang forever if
blocking work is scheduled with other tasks in the same runtime. To avoid
this, be sure to enable the `rt-tokio-current-thread` feature in this crate
if you are using that runtime (e.g. users of actix-web), and blocking tasks
will then be scheduled on a different thread.

# Examples

This processor can be configured with an [`executor`] of your choice to
batch and upload spans asynchronously when they end. If you have added a
library like [`tokio`], you can pass in their respective
`spawn` and `interval` functions to have batching performed in those
contexts.

```
# #[cfg(feature="tokio")]
# {
use opentelemetry::global;
use opentelemetry_sdk::{runtime, testing::trace::NoopSpanExporter, trace};
use opentelemetry_sdk::trace::BatchConfigBuilder;
use std::time::Duration;
use opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor;

#[tokio::main]
async fn main() {
    // Configure your preferred exporter
    let exporter = NoopSpanExporter::new();

    // Create a batch span processor using an exporter and a runtime
    let batch = BatchSpanProcessor::builder(exporter, runtime::Tokio)
        .with_batch_config(BatchConfigBuilder::default().with_max_queue_size(4096).build())
        .build();

    // Then use the `with_batch_exporter` method to have the provider export spans in batches.
    let provider = trace::SdkTracerProvider::builder()
        .with_span_processor(batch)
        .build();

    let _ = global::set_tracer_provider(provider);
}
# }
```

[`executor`]: https://docs.rs/futures/0.3/futures/executor/index.html
[`tokio`]: https://tokio.rs

---

## BatchSpanProcessorBuilder

`struct` · `opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessorBuilder`

```rust
struct BatchSpanProcessorBuilder<E, R>
```

**Derives**: Debug

**Methods** (2)

```rust
fn build(self) -> BatchSpanProcessor<R>
fn with_batch_config(self, config: BatchConfig) -> Self
```

A builder for creating [`BatchSpanProcessor`] instances.

---
