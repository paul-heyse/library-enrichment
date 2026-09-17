# `tracing_subscriber::layer::context`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.layer.context.json`](../model/tracing_subscriber.layer.context.json)

## Context

`struct` · `tracing_subscriber::layer::context::Context`

Also reachable as `tracing_subscriber::layer::Context`

```rust
struct Context<'a, S>
```

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn current_span(&self) -> span::Current
fn enabled(&self, metadata: &Metadata<'_>) -> bool
fn event(&self, event: &Event<'_>)
fn event_scope(&self, event: &Event<'_>) -> Option<registry::Scope<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn event_span(&self, event: &Event<'_>) -> Option<SpanRef<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn exists(&self, id: &span::Id) -> bool where S: for<'lookup> LookupSpan<'lookup>
fn lookup_current(&self) -> Option<registry::SpanRef<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn metadata(&self, id: &span::Id) -> Option<&'static Metadata<'static>> where S: for<'lookup> LookupSpan<'lookup>
fn span(&self, id: &span::Id) -> Option<registry::SpanRef<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn span_scope(&self, id: &span::Id) -> Option<registry::Scope<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
```

Represents information about the current context provided to [`Layer`]s by the
wrapped [`Subscriber`].

To access [stored data] keyed by a span ID, implementors of the `Layer`
trait should ensure that the `Subscriber` type parameter is *also* bound by the
[`LookupSpan`]:

```rust
use tracing::Subscriber;
use tracing_subscriber::{Layer, registry::LookupSpan};

pub struct MyLayer;

impl<S> Layer<S> for MyLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    // ...
}
```

[`Layer`]: super::Layer
[`Subscriber`]: tracing_core::Subscriber
[stored data]: crate::registry::SpanRef
[`LookupSpan`]: crate::registry::LookupSpan

---
