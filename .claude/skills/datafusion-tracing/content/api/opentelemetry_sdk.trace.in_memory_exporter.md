# `opentelemetry_sdk::trace::in_memory_exporter`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.in_memory_exporter.json`](../model/opentelemetry_sdk.trace.in_memory_exporter.json)

## InMemorySpanExporter

`struct` · `opentelemetry_sdk::trace::in_memory_exporter::InMemorySpanExporter`

Also reachable as `opentelemetry_sdk::trace::InMemorySpanExporter`

```rust
struct InMemorySpanExporter
```

**Implements**: `opentelemetry_sdk::trace::export::SpanExporter`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn get_finished_spans(&self) -> Result<Vec<SpanData>, InMemoryExporterError>
fn is_shutdown_called(&self) -> bool
fn reset(&self)
```

**via `opentelemetry_sdk::trace::export::SpanExporter`**

```rust
async fn export(&self, batch: Vec<SpanData>) -> OTelSdkResult
fn set_resource(&mut self, resource: &Resource)
fn shutdown_with_timeout(&mut self, _timeout: Duration) -> OTelSdkResult
```

 An in-memory span exporter that stores span data in memory.

 This exporter is useful for testing and debugging purposes. It stores
 span data in a `Vec<SpanData>`. Spans can be retrieved
 using the `get_finished_spans` method.
 # Example
 ```
# use opentelemetry::trace::{SpanKind, TraceContextExt};
# use opentelemetry::{global, trace::Tracer, Context};
# use opentelemetry_sdk::propagation::TraceContextPropagator;
# use opentelemetry_sdk::runtime;
# use opentelemetry_sdk::trace::InMemorySpanExporterBuilder;
# use opentelemetry_sdk::trace::{BatchSpanProcessor, SdkTracerProvider};

# #[tokio::main]
# async fn main() {
     let exporter = InMemorySpanExporterBuilder::new().build();
     let provider = SdkTracerProvider::builder()
         .with_span_processor(BatchSpanProcessor::builder(exporter.clone()).build())
         .build();

     global::set_tracer_provider(provider.clone());

     let tracer = global::tracer("example/in_memory_exporter");
     let span = tracer
         .span_builder("say hello")
         .with_kind(SpanKind::Server)
         .start(&tracer);

     let cx = Context::current_with_span(span);
     cx.span().add_event("handling this...", Vec::new());
     cx.span().end();

     if let Err(e) = provider.force_flush() {
         println!("{:?}", e)
     }
     let spans = exporter.get_finished_spans().unwrap();
     for span in spans {
         println!("{:?}", span)
     }
# }
 ```

---

## InMemorySpanExporterBuilder

`struct` · `opentelemetry_sdk::trace::in_memory_exporter::InMemorySpanExporterBuilder`

Also reachable as `opentelemetry_sdk::trace::InMemorySpanExporterBuilder`

```rust
struct InMemorySpanExporterBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn build(&self) -> InMemorySpanExporter
fn new() -> Self
```

 Builder for [`InMemorySpanExporter`].
 # Example
 ```
# use opentelemetry_sdk::trace::InMemorySpanExporterBuilder;

 let exporter = InMemorySpanExporterBuilder::new().build();
 ```

---
