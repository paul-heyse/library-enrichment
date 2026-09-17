# `opentelemetry_sdk::trace::span`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.trace.span.json`](../model/opentelemetry_sdk.trace.span.json)

## Span

`struct` · `opentelemetry_sdk::trace::span::Span`

Also reachable as `opentelemetry_sdk::trace::Span`

```rust
struct Span
```

**Implements**: `core::ops::drop::Drop`, `opentelemetry::trace::span::Span`

**Derives**: Debug

**Methods** (1)

```rust
fn exported_data(&self) -> Option<trace::SpanData>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `opentelemetry::trace::span::Span`**

```rust
fn add_event_with_timestamp<T>(&mut self, name: T, timestamp: SystemTime, attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_link(&mut self, span_context: SpanContext, attributes: Vec<KeyValue>)
fn end_with_timestamp(&mut self, timestamp: SystemTime)
fn is_recording(&self) -> bool
fn set_attribute(&mut self, attribute: KeyValue)
fn set_status(&mut self, status: Status)
fn span_context(&self) -> &SpanContext
fn update_name<T>(&mut self, new_name: T) where T: Into<Cow<'static, str>>
```

Single operation within a trace.

---
