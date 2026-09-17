# `opentelemetry::trace::context`

Crate `opentelemetry` · 4 public items · structured records in [`model/opentelemetry.trace.context.json`](../model/opentelemetry.trace.context.json)

## get_active_span

`function` · `opentelemetry::trace::context::get_active_span`

Also reachable as `opentelemetry::trace::get_active_span`

```rust
fn get_active_span<F, T>(f: F) -> T where F: FnOnce(SpanRef<'_>) -> T
```

Executes a closure with a reference to this thread's current span.

# Examples

```
use opentelemetry::{global, trace::{Span, Tracer}, KeyValue};
use opentelemetry::trace::get_active_span;

fn my_function() {
    // start an active span in one function
    global::tracer("my-component").in_span("span-name", |_cx| {
        // anything happening in functions we call can still access the active span...
        my_other_function();
    })
}

fn my_other_function() {
    // call methods on the current span from
    get_active_span(|span| {
        span.add_event("An event!", vec![KeyValue::new("happened", true)]);
    })
}
```

---

## mark_span_as_active

`function` · `opentelemetry::trace::context::mark_span_as_active`

Also reachable as `opentelemetry::trace::mark_span_as_active`

```rust
fn mark_span_as_active<T: trace::Span + Send + Sync + 'static>(span: T) -> ContextGuard
```

Mark a given `Span` as active.

The `Tracer` MUST provide a way to update its active `Span`, and MAY provide convenience
methods to manage a `Span`'s lifetime and the scope in which a `Span` is active. When an
active `Span` is made inactive, the previously-active `Span` SHOULD be made active. A `Span`
maybe finished (i.e. have a non-null end time) but still be active. A `Span` may be active
on one thread after it has been made inactive on another.

# Examples

```
use opentelemetry::{global, trace::{Span, Tracer}, KeyValue};
use opentelemetry::trace::{get_active_span, mark_span_as_active};

fn my_function() {
    let tracer = global::tracer("my-component-a");
    // start an active span in one function
    let span = tracer.start("span-name");
    let _guard = mark_span_as_active(span);
    // anything happening in functions we call can still access the active span...
    my_other_function();
}

fn my_other_function() {
    // call methods on the current span from
    get_active_span(|span| {
        span.add_event("An event!".to_string(), vec![KeyValue::new("happened", true)]);
    });
}
```

---

## SpanRef

`struct` · `opentelemetry::trace::context::SpanRef`

Also reachable as `opentelemetry::trace::SpanRef`

```rust
struct SpanRef<'a>
```

**Derives**: Debug

**Methods** (12)

```rust
fn add_event<T>(&self, name: T, attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_event_with_timestamp<T>(&self, name: T, timestamp: std::time::SystemTime, attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_link(&self, span_context: SpanContext, attributes: Vec<KeyValue>)
fn end(&self)
fn end_with_timestamp(&self, timestamp: std::time::SystemTime)
fn is_recording(&self) -> bool
fn record_error(&self, err: &dyn Error)
fn set_attribute(&self, attribute: KeyValue)
fn set_attributes(&self, attributes: impl IntoIterator<Item = KeyValue>)
fn set_status(&self, status: Status)
fn span_context(&self) -> &SpanContext
fn update_name<T>(&self, new_name: T) where T: Into<Cow<'static, str>>
```

A reference to the currently active span in this context.

---

## TraceContextExt

`trait` · `opentelemetry::trace::context::TraceContextExt`

Also reachable as `opentelemetry::trace::TraceContextExt`

```rust
trait TraceContextExt
```

**Implementors** (1)

- `opentelemetry::context::Context`

**Methods** (5)

```rust
fn current_with_span<T: trace::Span + Send + Sync + 'static>(span: T) -> Self
fn has_active_span(&self) -> bool
fn span(&self) -> SpanRef<'_>
fn with_remote_span_context(&self, span_context: trace::SpanContext) -> Self
fn with_span<T: trace::Span + Send + Sync + 'static>(&self, span: T) -> Self
```

Methods for storing and retrieving trace data in a [`Context`].

See [`Context`] for examples of setting and retrieving the current context.

---
