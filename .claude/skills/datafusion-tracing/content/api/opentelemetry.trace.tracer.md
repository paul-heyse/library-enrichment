# `opentelemetry::trace::tracer`

Crate `opentelemetry` · 4 public items · structured records in [`model/opentelemetry.trace.tracer.json`](../model/opentelemetry.trace.tracer.json)

## SamplingDecision

`enum` · `opentelemetry::trace::tracer::SamplingDecision`

Also reachable as `opentelemetry::trace::SamplingDecision`

```rust
enum SamplingDecision
```

**Variants**: `Drop`, `RecordOnly`, `RecordAndSample`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Decision about whether or not to sample

---

## SamplingResult

`struct` · `opentelemetry::trace::tracer::SamplingResult`

Also reachable as `opentelemetry::trace::SamplingResult`

```rust
struct SamplingResult
```

**Fields**: `decision`, `attributes`, `trace_state`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

The result of sampling logic for a given span.

---

## SpanBuilder

`struct` · `opentelemetry::trace::tracer::SpanBuilder`

Also reachable as `opentelemetry::trace::SpanBuilder`

```rust
struct SpanBuilder
```

**Fields**: `trace_id`, `span_id`, `span_kind`, `name`, `start_time`, `end_time`, `attributes`, `events`, `links`, `status`, `sampling_result`

**Derives**: Clone, Debug, Default

**Methods** (13)

```rust
fn from_name<T: Into<Cow<'static, str>>>(name: T) -> Self
fn start<T: Tracer>(self, tracer: &T) -> T::Span
fn start_with_context<T: Tracer>(self, tracer: &T, parent_cx: &Context) -> T::Span
fn with_attributes<I>(self, attributes: I) -> Self where I: IntoIterator<Item = KeyValue>
fn with_end_time<T: Into<SystemTime>>(self, end_time: T) -> Self
fn with_events(self, events: Vec<Event>) -> Self
fn with_kind(self, span_kind: SpanKind) -> Self
fn with_links(self, links: Vec<Link>) -> Self
fn with_sampling_result(self, sampling_result: SamplingResult) -> Self
fn with_span_id(self, span_id: SpanId) -> Self
fn with_start_time<T: Into<SystemTime>>(self, start_time: T) -> Self
fn with_status(self, status: Status) -> Self
fn with_trace_id(self, trace_id: TraceId) -> Self
```

`SpanBuilder` allows span attributes to be configured before the span
has started.

```
use opentelemetry::{
    global,
    trace::{TracerProvider, SpanBuilder, SpanKind, Tracer},
};

let tracer = global::tracer("example-tracer");

// The builder can be used to create a span directly with the tracer
let _span = tracer.build(SpanBuilder {
    name: "example-span-name".into(),
    span_kind: Some(SpanKind::Server),
    ..Default::default()
});

// Or used with builder pattern
let _span = tracer
    .span_builder("example-span-name")
    .with_kind(SpanKind::Server)
    .start(&tracer);
```

---

## Tracer

`trait` · `opentelemetry::trace::tracer::Tracer`

Also reachable as `opentelemetry::trace::Tracer`

```rust
trait Tracer
```

**Implementors** (3)

- `opentelemetry::global::trace::BoxedTracer`
- `opentelemetry::trace::noop::NoopTracer`
- `opentelemetry_sdk::trace::tracer::SdkTracer`

**Methods** (6)

```rust
fn build(&self, builder: SpanBuilder) -> Self::Span
fn build_with_context(&self, builder: SpanBuilder, parent_cx: &Context) -> Self::Span
fn in_span<T, F, N>(&self, name: N, f: F) -> T where F: FnOnce(Context) -> T, N: Into<Cow<'static, str>>, Self::Span: Send + Sync + 'static
fn span_builder<T>(&self, name: T) -> SpanBuilder where T: Into<Cow<'static, str>>
fn start<T>(&self, name: T) -> Self::Span where T: Into<Cow<'static, str>>
fn start_with_context<T>(&self, name: T, parent_cx: &Context) -> Self::Span where T: Into<Cow<'static, str>>
```

The interface for constructing [`Span`]s.

## In Synchronous Code

Spans can be created and nested manually:

```
use opentelemetry::{global, trace::{Span, Tracer, TraceContextExt}, Context};

let tracer = global::tracer("my-component");

let parent = tracer.start("foo");
let parent_cx = Context::current_with_span(parent);
let mut child = tracer.start_with_context("bar", &parent_cx);

// ...

child.end(); // explicitly end
drop(parent_cx) // or implicitly end on drop
```

Spans can also use the current thread's [`Context`] to track which span is active:

```
use opentelemetry::{global, trace::{SpanKind, Tracer}};

let tracer = global::tracer("my-component");

// Create simple spans with `in_span`
tracer.in_span("foo", |_foo_cx| {
    // parent span is active
    tracer.in_span("bar", |_bar_cx| {
        // child span is now the active span and associated with the parent span
    });
    // child has ended, parent now the active span again
});
// parent has ended, no active spans
```

Spans can also be marked as active, and the resulting guard allows for
greater control over when the span is no longer considered active.

```
use opentelemetry::{global, trace::{Span, Tracer, mark_span_as_active}};
let tracer = global::tracer("my-component");

let parent_span = tracer.start("foo");
let parent_active = mark_span_as_active(parent_span);

{
    let child = tracer.start("bar");
    let _child_active = mark_span_as_active(child);

    // do work in the context of the child span...

    // exiting the scope drops the guard, child is no longer active
}
// Parent is active span again

// Parent can be dropped manually, or allowed to go out of scope as well.
drop(parent_active);

// no active span
```

## In Asynchronous Code

If you are instrumenting code that make use of [`std::future::Future`] or
async/await, be sure to use the [`FutureExt`] trait. This is needed because
the following example _will not_ work:

```no_run
# use opentelemetry::{global, trace::{Tracer, mark_span_as_active}};
# let tracer = global::tracer("foo");
# let span = tracer.start("foo-span");
async {
    // Does not work
    let _g = mark_span_as_active(span);
    // ...
};
```

The context guard `_g` will not exit until the future generated by the
`async` block is complete. Since futures can be entered and exited
_multiple_ times without them completing, the span remains active for as
long as the future exists, rather than only when it is polled, leading to
very confusing and incorrect output.

In order to trace asynchronous code, the [`Future::with_context`] combinator
can be used:

```
# async fn run() -> Result<(), ()> {
use opentelemetry::{trace::FutureExt, Context};
let cx = Context::current();

let my_future = async {
    // ...
};

my_future
    .with_context(cx)
    .await;
# Ok(())
# }
```

[`Future::with_context`] attaches a context to the future, ensuring that the
context's lifetime is as long as the future's.

[`FutureExt`]: crate::trace::FutureExt
[`Future::with_context`]: crate::trace::FutureExt::with_context()
[`Context`]: crate::Context

---
