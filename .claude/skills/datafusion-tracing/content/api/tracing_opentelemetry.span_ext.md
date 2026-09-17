# `tracing_opentelemetry::span_ext`

Crate `tracing-opentelemetry` · 2 public items · structured records in [`model/tracing_opentelemetry.span_ext.json`](../model/tracing_opentelemetry.span_ext.json)

## SetParentError

`enum` · `tracing_opentelemetry::span_ext::SetParentError`

```rust
enum SetParentError
```

**Variants**: `LayerNotFound`, `AlreadyStarted`, `SpanDisabled`

---

## OpenTelemetrySpanExt

`trait` · `tracing_opentelemetry::span_ext::OpenTelemetrySpanExt`

Also reachable as `tracing_opentelemetry::OpenTelemetrySpanExt`

```rust
trait OpenTelemetrySpanExt
```

**Implementors** (1)

- `tracing::span::Span`

**Methods** (8)

```rust
fn add_event(&self, name: impl Into<Cow<'static, str>>, attributes: Vec<KeyValue>)
fn add_event_with_timestamp(&self, name: impl Into<Cow<'static, str>>, timestamp: SystemTime, attributes: Vec<KeyValue>)
fn add_link(&self, cx: SpanContext)
fn add_link_with_attributes(&self, cx: SpanContext, attributes: Vec<KeyValue>)
fn context(&self) -> Context
fn set_attribute(&self, key: impl Into<Key>, value: impl Into<Value>)
fn set_parent(&self, cx: Context) -> Result<(), SetParentError>
fn set_status(&self, status: Status)
```

Utility functions to allow tracing [`Span`]s to accept and return
[OpenTelemetry] [`Context`]s.

[`Span`]: tracing::Span
[OpenTelemetry]: https://opentelemetry.io
[`Context`]: opentelemetry::Context

---
