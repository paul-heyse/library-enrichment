# `opentelemetry::global::trace`

Crate `opentelemetry` · 10 public items · structured records in [`model/opentelemetry.global.trace.json`](../model/opentelemetry.global.trace.json)

## set_tracer_provider

`function` · `opentelemetry::global::trace::set_tracer_provider`

Also reachable as `opentelemetry::global::set_tracer_provider`

```rust
fn set_tracer_provider<P, T, S>(new_provider: P) where S: trace::Span + Send + Sync + 'static, T: trace::Tracer<Span = S> + Send + Sync + 'static, P: trace::TracerProvider<Tracer = T> + Send + Sync + 'static
```

Sets the given [`TracerProvider`] instance as the current global provider.

Libraries should NOT call this function. It is intended for applications/executables.
[`TracerProvider`]: crate::trace::TracerProvider

---

## tracer

`function` · `opentelemetry::global::trace::tracer`

Also reachable as `opentelemetry::global::tracer`

```rust
fn tracer(name: impl Into<std::borrow::Cow<'static, str>>) -> BoxedTracer
```

Creates a named instance of [`Tracer`] via the configured [`GlobalTracerProvider`].

If the name is an empty string, the provider will use a default name.

This is a more convenient way of expressing `global::tracer_provider().tracer(name)`.

[`Tracer`]: crate::trace::Tracer

---

## tracer_provider

`function` · `opentelemetry::global::trace::tracer_provider`

Also reachable as `opentelemetry::global::tracer_provider`

```rust
fn tracer_provider() -> GlobalTracerProvider
```

Returns an instance of the currently configured global [`TracerProvider`] through
[`GlobalTracerProvider`].

[`TracerProvider`]: crate::trace::TracerProvider
[`GlobalTracerProvider`]: crate::global::GlobalTracerProvider

---

## tracer_with_scope

`function` · `opentelemetry::global::trace::tracer_with_scope`

Also reachable as `opentelemetry::global::tracer_with_scope`

```rust
fn tracer_with_scope(scope: InstrumentationScope) -> BoxedTracer
```

Creates a [`Tracer`] with the given instrumentation scope
via the configured [`GlobalTracerProvider`].

This is a simpler alternative to `global::tracer_provider().tracer_with_scope(...)`

# Example

```
use std::sync::Arc;
use opentelemetry::global::tracer_with_scope;
use opentelemetry::InstrumentationScope;
use opentelemetry::KeyValue;

let scope = InstrumentationScope::builder("io.opentelemetry")
    .with_version("0.17")
    .with_schema_url("https://opentelemetry.io/schema/1.2.0")
    .with_attributes(vec![(KeyValue::new("key", "value"))])
    .build();

let tracer = tracer_with_scope(scope);
```

[`Tracer`]: crate::trace::Tracer

---

## BoxedSpan

`struct` · `opentelemetry::global::trace::BoxedSpan`

Also reachable as `opentelemetry::global::BoxedSpan`

```rust
struct BoxedSpan
```

**Implements**: `opentelemetry::trace::span::Span`

**Derives**: Debug

**via `opentelemetry::trace::span::Span`**

```rust
fn add_event_with_timestamp<T>(&mut self, name: T, timestamp: SystemTime, attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_link(&mut self, span_context: trace::SpanContext, attributes: Vec<KeyValue>)
fn end_with_timestamp(&mut self, timestamp: SystemTime)
fn is_recording(&self) -> bool
fn set_attribute(&mut self, attribute: KeyValue)
fn set_status(&mut self, status: trace::Status)
fn span_context(&self) -> &trace::SpanContext
fn update_name<T>(&mut self, new_name: T) where T: Into<Cow<'static, str>>
```

Wraps the [`BoxedTracer`]'s [`Span`] so it can be used generically by
applications without knowing the underlying type.

[`Span`]: crate::trace::Span

---

## BoxedTracer

`struct` · `opentelemetry::global::trace::BoxedTracer`

Also reachable as `opentelemetry::global::BoxedTracer`

```rust
struct BoxedTracer
```

**Implements**: `opentelemetry::trace::tracer::Tracer`

**Derives**: Debug

**Methods** (1)

```rust
fn new(tracer: Box<dyn ObjectSafeTracer + Send + Sync>) -> Self
```

**via `opentelemetry::trace::tracer::Tracer`**

```rust
fn build_with_context(&self, builder: trace::SpanBuilder, parent_cx: &Context) -> Self::Span
```

Wraps the [`GlobalTracerProvider`]'s [`Tracer`] so it can be used generically by
applications without knowing the underlying type.

[`Tracer`]: crate::trace::Tracer
[`GlobalTracerProvider`]: crate::global::GlobalTracerProvider

---

## GlobalTracerProvider

`struct` · `opentelemetry::global::trace::GlobalTracerProvider`

Also reachable as `opentelemetry::global::GlobalTracerProvider`

```rust
struct GlobalTracerProvider
```

**Implements**: `opentelemetry::trace::tracer_provider::TracerProvider`

**Derives**: Clone, Debug

**via `opentelemetry::trace::tracer_provider::TracerProvider`**

```rust
fn tracer_with_scope(&self, scope: InstrumentationScope) -> Self::Tracer
```

Represents the globally configured [`TracerProvider`] instance for this
application. This allows generic tracing through the returned
[`BoxedTracer`] instances.

[`TracerProvider`]: crate::trace::TracerProvider

---

## ObjectSafeSpan

`trait` · `opentelemetry::global::trace::ObjectSafeSpan`

Also reachable as `opentelemetry::global::ObjectSafeSpan`

```rust
trait ObjectSafeSpan
```

**Methods** (9)

```rust
fn add_event_with_timestamp(&mut self, name: Cow<'static, str>, timestamp: SystemTime, attributes: Vec<KeyValue>)
fn add_link(&mut self, span_context: SpanContext, attributes: Vec<KeyValue>)
fn end(&mut self)
fn end_with_timestamp(&mut self, timestamp: SystemTime)
fn is_recording(&self) -> bool
fn set_attribute(&mut self, attribute: KeyValue)
fn set_status(&mut self, status: Status)
fn span_context(&self) -> &SpanContext
fn update_name(&mut self, new_name: Cow<'static, str>)
```

Allows a specific [`crate::trace::Span`] to be used generically by [`BoxedSpan`]
instances by mirroring the interface and boxing the return types.

---

## ObjectSafeTracer

`trait` · `opentelemetry::global::trace::ObjectSafeTracer`

Also reachable as `opentelemetry::global::ObjectSafeTracer`

```rust
trait ObjectSafeTracer
```

**Methods** (1)

```rust
fn build_with_context_boxed(&self, builder: trace::SpanBuilder, parent_cx: &Context) -> Box<dyn ObjectSafeSpan + Send + Sync>
```

Allows a specific [`Tracer`] to be used generically by [`BoxedTracer`]
instances by mirroring the interface and boxing the return types.

[`Tracer`]: crate::trace::Tracer

---

## ObjectSafeTracerProvider

`trait` · `opentelemetry::global::trace::ObjectSafeTracerProvider`

Also reachable as `opentelemetry::global::ObjectSafeTracerProvider`

```rust
trait ObjectSafeTracerProvider
```

**Methods** (1)

```rust
fn boxed_tracer(&self, scope: InstrumentationScope) -> Box<dyn ObjectSafeTracer + Send + Sync>
```

Allows a specific [`TracerProvider`] to be used generically by the
[`GlobalTracerProvider`] by mirroring the interface and boxing the return types.

[`TracerProvider`]: crate::trace::TracerProvider
[`GlobalTracerProvider`]: crate::global::GlobalTracerProvider

---
