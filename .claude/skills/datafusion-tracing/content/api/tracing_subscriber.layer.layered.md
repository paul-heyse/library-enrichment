# `tracing_subscriber::layer::layered`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.layer.layered.json`](../model/tracing_subscriber.layer.layered.json)

## Layered

`struct` · `tracing_subscriber::layer::layered::Layered`

Also reachable as `tracing_subscriber::layer::Layered`

```rust
struct Layered<L, I, S = I>
```

**Implements**: `tracing_core::subscriber::Subscriber`, `tracing_subscriber::layer::Layer`, `tracing_subscriber::registry::LookupSpan`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn downcast_ref<T: Any>(&self) -> Option<&T>
fn is<T: Any>(&self) -> bool
```

**via `tracing_core::subscriber::Subscriber`**

```rust
fn clone_span(&self, old: &span::Id) -> span::Id
fn current_span(&self) -> span::Current
fn drop_span(&self, id: span::Id)
fn enabled(&self, metadata: &Metadata<'_>) -> bool
fn enter(&self, span: &span::Id)
fn event(&self, event: &Event<'_>)
fn event_enabled(&self, event: &Event<'_>) -> bool
fn exit(&self, span: &span::Id)
fn max_level_hint(&self) -> Option<LevelFilter>
fn new_span(&self, span: &span::Attributes<'_>) -> span::Id
fn on_register_dispatch(&self, subscriber: &Dispatch)
fn record(&self, span: &span::Id, values: &span::Record<'_>)
fn record_follows_from(&self, span: &span::Id, follows: &span::Id)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
fn try_close(&self, id: span::Id) -> bool
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool
fn event_enabled(&self, event: &Event<'_>, ctx: Context<'_, S>) -> bool
fn on_close(&self, id: span::Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_follows_from(&self, span: &span::Id, follows: &span::Id, ctx: Context<'_, S>)
fn on_id_change(&self, old: &span::Id, new: &span::Id, ctx: Context<'_, S>)
fn on_layer(&mut self, subscriber: &mut S)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)
fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)
fn on_register_dispatch(&self, subscriber: &Dispatch)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
```

**via `tracing_subscriber::registry::LookupSpan`**

```rust
fn register_filter(&mut self) -> FilterId
fn span_data(&'a self, id: &span::Id) -> Option<Self::Data>
```

A [`Subscriber`] composed of a `Subscriber` wrapped by one or more
[`Layer`]s.

[`Layer`]: crate::Layer
[`Subscriber`]: tracing_core::Subscriber

---
