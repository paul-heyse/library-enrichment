# `tracing_opentelemetry::layer`

Crate `tracing-opentelemetry` · 2 public items · structured records in [`model/tracing_opentelemetry.layer.json`](../model/tracing_opentelemetry.layer.json)

## layer

`function` · `tracing_opentelemetry::layer::layer`

Also reachable as `tracing_opentelemetry::layer`

```rust
fn layer<S>() -> OpenTelemetryLayer<S, noop::NoopTracer> where S: Subscriber + for<'span> LookupSpan<'span>
```

Construct a layer to track spans via [OpenTelemetry].

[OpenTelemetry]: https://opentelemetry.io

# Examples

```rust,no_run
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

// Use the tracing subscriber `Registry`, or any other subscriber
// that impls `LookupSpan`
let subscriber = Registry::default().with(tracing_opentelemetry::layer());
# drop(subscriber);
```

---

## OpenTelemetryLayer

`struct` · `tracing_opentelemetry::layer::OpenTelemetryLayer`

Also reachable as `tracing_opentelemetry::OpenTelemetryLayer`

```rust
struct OpenTelemetryLayer<S, T>
```

**Implements**: `tracing_subscriber::layer::Layer`

**Derives**: Default

**Methods** (12)

```rust
fn new(tracer: T) -> Self
fn with_context_activation(self, context_activation: bool) -> Self
fn with_error_events_to_exceptions(self, error_events_to_exceptions: bool) -> Self
fn with_error_events_to_status(self, error_events_to_status: bool) -> Self
fn with_error_fields_to_exceptions(self, error_fields_to_exceptions: bool) -> Self
fn with_error_records_to_exceptions(self, error_records_to_exceptions: bool) -> Self
fn with_level(self, level: bool) -> Self
fn with_location(self, location: bool) -> Self
fn with_target(self, target: bool) -> Self
fn with_threads(self, threads: bool) -> Self
fn with_tracer<Tracer>(self, tracer: Tracer) -> OpenTelemetryLayer<S, Tracer> where Tracer: otel::Tracer + 'static, Tracer::Span: Send + Sync
fn with_tracked_inactivity(self, tracked_inactivity: bool) -> Self
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn on_close(&self, id: span::Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_follows_from(&self, id: &Id, follows: &Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

An [OpenTelemetry] propagation layer for use in a project that uses
[tracing].

[OpenTelemetry]: https://opentelemetry.io
[tracing]: https://github.com/tokio-rs/tracing

---
