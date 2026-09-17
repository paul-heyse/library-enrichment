# `opentelemetry::trace::noop`

Crate `opentelemetry` · 4 public items · structured records in [`model/opentelemetry.trace.noop.json`](../model/opentelemetry.trace.noop.json)

## NoopSpan

`struct` · `opentelemetry::trace::noop::NoopSpan`

```rust
struct NoopSpan
```

**Implements**: `opentelemetry::trace::span::Span`

**Derives**: Clone, Debug

**via `opentelemetry::trace::span::Span`**

```rust
fn add_event<T>(&mut self, _name: T, _attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_event_with_timestamp<T>(&mut self, _name: T, _timestamp: SystemTime, _attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_link(&mut self, _span_context: trace::SpanContext, _attributes: Vec<KeyValue>)
fn end_with_timestamp(&mut self, _timestamp: SystemTime)
fn is_recording(&self) -> bool
fn set_attribute(&mut self, _attribute: KeyValue)
fn set_status(&mut self, _status: trace::Status)
fn span_context(&self) -> &trace::SpanContext
fn update_name<T>(&mut self, _new_name: T) where T: Into<Cow<'static, str>>
```

A no-op instance of a `Span`.

---

## NoopTextMapPropagator

`struct` · `opentelemetry::trace::noop::NoopTextMapPropagator`

```rust
struct NoopTextMapPropagator
```

**Implements**: `opentelemetry::propagation::text_map_propagator::TextMapPropagator`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `opentelemetry::propagation::text_map_propagator::TextMapPropagator`**

```rust
fn extract_with_context(&self, _cx: &Context, _extractor: &dyn Extractor) -> Context
fn fields(&self) -> FieldIter<'_>
fn inject_context(&self, _cx: &Context, _injector: &mut dyn Injector)
```

A no-op instance of an [`TextMapPropagator`].

[`TextMapPropagator`]: crate::propagation::TextMapPropagator

---

## NoopTracer

`struct` · `opentelemetry::trace::noop::NoopTracer`

```rust
struct NoopTracer
```

**Implements**: `opentelemetry::trace::tracer::Tracer`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `opentelemetry::trace::tracer::Tracer`**

```rust
fn build_with_context(&self, _builder: trace::SpanBuilder, parent_cx: &Context) -> Self::Span
```

A no-op instance of a `Tracer`.

---

## NoopTracerProvider

`struct` · `opentelemetry::trace::noop::NoopTracerProvider`

```rust
struct NoopTracerProvider
```

**Implements**: `opentelemetry::trace::tracer_provider::TracerProvider`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `opentelemetry::trace::tracer_provider::TracerProvider`**

```rust
fn tracer_with_scope(&self, _scope: InstrumentationScope) -> Self::Tracer
```

A no-op instance of a `TracerProvider`.

---
