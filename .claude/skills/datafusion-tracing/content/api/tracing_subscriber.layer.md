# `tracing_subscriber::layer`

Crate `tracing-subscriber` · 4 public items · structured records in [`model/tracing_subscriber.layer.json`](../model/tracing_subscriber.layer.json)

## Identity

`struct` · `tracing_subscriber::layer::Identity`

```rust
struct Identity
```

**Implements**: `tracing_subscriber::layer::Layer`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

A layer that does nothing.

---

## Filter

`trait` · `tracing_subscriber::layer::Filter`

```rust
trait Filter<S>
```

**Implementors** (12)

- `alloc::boxed::Box`
- `alloc::sync::Arc`
- `core::option::Option`
- `tracing_core::metadata::LevelFilter`
- `tracing_subscriber::filter::env::EnvFilter`
- `tracing_subscriber::filter::filter_fn::DynFilterFn`
- `tracing_subscriber::filter::filter_fn::FilterFn`
- `tracing_subscriber::filter::layer_filters::combinator::And`
- `tracing_subscriber::filter::layer_filters::combinator::Not`
- `tracing_subscriber::filter::layer_filters::combinator::Or`
- `tracing_subscriber::filter::targets::Targets`
- `tracing_subscriber::reload::Layer`

**Methods** (9)

```rust
fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest
fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool
fn event_enabled(&self, event: &Event<'_>, cx: &Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
fn on_close(&self, id: span::Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)
fn on_record(&self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)
```

A per-[`Layer`] filter that determines whether a span or event is enabled
for an individual layer.

See [the module-level documentation][plf] for details on using [`Filter`]s.

[plf]: crate::layer#per-layer-filtering

---

## Layer

`trait` · `tracing_subscriber::layer::Layer`

Also reachable as `tracing_subscriber::Layer`, `tracing_subscriber::prelude::__tracing_subscriber_Layer`

```rust
trait Layer<S> where S: Subscriber, Self: 'static
```

**Implementors** (15)

- `alloc::boxed::Box`
- `alloc::vec::Vec`
- `core::option::Option`
- `tracing_core::metadata::LevelFilter`
- `tracing_opentelemetry::layer::OpenTelemetryLayer`
- `tracing_opentelemetry::metrics::MetricsLayer`
- `tracing_subscriber::filter::env::EnvFilter`
- `tracing_subscriber::filter::filter_fn::DynFilterFn`
- `tracing_subscriber::filter::filter_fn::FilterFn`
- `tracing_subscriber::filter::layer_filters::Filtered`
- `tracing_subscriber::filter::targets::Targets`
- `tracing_subscriber::fmt::fmt_layer::Layer`
- `tracing_subscriber::layer::Identity`
- `tracing_subscriber::layer::layered::Layered`
- `tracing_subscriber::reload::Layer`

**Methods** (17)

```rust
fn and_then<L>(self, layer: L) -> Layered<L, Self, S> where L: Layer<S>, Self: Sized
fn boxed(self) -> Box<dyn Layer<S> + Send + Sync + 'static> where Self: Sized + Layer<S> + Send + Sync + 'static, S: Subscriber
fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool
fn event_enabled(&self, _event: &Event<'_>, _ctx: Context<'_, S>) -> bool
fn on_close(&self, _id: span::Id, _ctx: Context<'_, S>)
fn on_enter(&self, _id: &span::Id, _ctx: Context<'_, S>)
fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>)
fn on_exit(&self, _id: &span::Id, _ctx: Context<'_, S>)
fn on_follows_from(&self, _span: &span::Id, _follows: &span::Id, _ctx: Context<'_, S>)
fn on_id_change(&self, _old: &span::Id, _new: &span::Id, _ctx: Context<'_, S>)
fn on_layer(&mut self, subscriber: &mut S)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)
fn on_record(&self, _span: &span::Id, _values: &span::Record<'_>, _ctx: Context<'_, S>)
fn on_register_dispatch(&self, subscriber: &Dispatch)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
fn with_filter<F>(self, filter: F) -> filter::Filtered<Self, F, S> where Self: Sized, F: Filter<S>
fn with_subscriber(self, inner: S) -> Layered<Self, S> where Self: Sized
```

A composable handler for `tracing` events.

A `Layer` implements a behavior for recording or collecting traces that can
be composed together with other `Layer`s to build a [`Subscriber`]. See the
[module-level documentation](crate::layer) for details.

[`Subscriber`]: tracing_core::Subscriber

---

## SubscriberExt

`trait` · `tracing_subscriber::layer::SubscriberExt`

Also reachable as `tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt`

```rust
trait SubscriberExt: Subscriber + sealed::Sealed
```

**Methods** (1)

```rust
fn with<L>(self, layer: L) -> Layered<L, Self> where L: Layer<Self>, Self: Sized
```

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

---
