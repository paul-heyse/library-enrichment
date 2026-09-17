# `opentelemetry_sdk::trace::provider`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.provider.json`](../model/opentelemetry_sdk.trace.provider.json)

## SdkTracerProvider

`struct` · `opentelemetry_sdk::trace::provider::SdkTracerProvider`

Also reachable as `opentelemetry_sdk::trace::SdkTracerProvider`

```rust
struct SdkTracerProvider
```

**Implements**: `opentelemetry::trace::tracer_provider::TracerProvider`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn builder() -> TracerProviderBuilder
fn force_flush(&self) -> OTelSdkResult
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

**via `opentelemetry::trace::tracer_provider::TracerProvider`**

```rust
fn tracer(&self, name: impl Into<Cow<'static, str>>) -> Self::Tracer
fn tracer_with_scope(&self, scope: InstrumentationScope) -> Self::Tracer
```

Creator and registry of named [`SdkTracer`] instances.

`TracerProvider` is a container holding pointers to `SpanProcessor` and other components.
Cloning a `TracerProvider` instance and dropping it will not stop span processing. To stop span processing, users
must either call the `shutdown` method explicitly or allow the last reference to the `TracerProvider`
to be dropped. When the last reference is dropped, the shutdown process will be automatically triggered
to ensure proper cleanup.

---

## TracerProviderBuilder

`struct` · `opentelemetry_sdk::trace::provider::TracerProviderBuilder`

Also reachable as `opentelemetry_sdk::trace::TracerProviderBuilder`

```rust
struct TracerProviderBuilder
```

**Derives**: Debug, Default

**Methods** (13)

```rust
fn build(self) -> SdkTracerProvider
fn with_batch_exporter<T: SpanExporter + 'static>(self, exporter: T) -> Self
fn with_id_generator<T: IdGenerator + 'static>(self, id_generator: T) -> Self
fn with_max_attributes_per_event(self, max_attributes: u32) -> Self
fn with_max_attributes_per_link(self, max_attributes: u32) -> Self
fn with_max_attributes_per_span(self, max_attributes: u32) -> Self
fn with_max_events_per_span(self, max_events: u32) -> Self
fn with_max_links_per_span(self, max_links: u32) -> Self
fn with_resource(self, resource: Resource) -> Self
fn with_sampler<T: trace::ShouldSample + 'static>(self, sampler: T) -> Self
fn with_simple_exporter<T: SpanExporter + 'static>(self, exporter: T) -> Self
fn with_span_limits(self, span_limits: SpanLimits) -> Self
fn with_span_processor<T: SpanProcessor + 'static>(self, processor: T) -> Self
```

Builder for provider attributes.

---
