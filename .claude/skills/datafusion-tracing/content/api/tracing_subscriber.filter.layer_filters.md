# `tracing_subscriber::filter::layer_filters`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.filter.layer_filters.json`](../model/tracing_subscriber.filter.layer_filters.json)

## FilterId

`struct` · `tracing_subscriber::filter::layer_filters::FilterId`

Also reachable as `tracing_subscriber::filter::FilterId`

```rust
struct FilterId
```

**Implements**: `core::fmt::Binary`

**Derives**: Clone, Copy, Debug

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Uniquely identifies an individual [`Filter`] instance in the context of
a [`Subscriber`].

When adding a [`Filtered`] [`Layer`] to a [`Subscriber`], the [`Subscriber`]
generates a `FilterId` for that [`Filtered`] layer. The [`Filtered`] layer
will then use the generated ID to query whether a particular span was
previously enabled by that layer's [`Filter`].

**Note**: Currently, the [`Registry`] type provided by this crate is the
**only** [`Subscriber`] implementation capable of participating in per-layer
filtering. Therefore, the `FilterId` type cannot currently be constructed by
code outside of `tracing-subscriber`. In the future, new APIs will be added to `tracing-subscriber` to
allow non-Registry [`Subscriber`]s to also participate in per-layer
filtering. When those APIs are added, subscribers will be responsible
for generating and assigning `FilterId`s.

[`Filter`]: crate::layer::Filter
[`Subscriber`]: tracing_core::Subscriber
[`Layer`]: crate::layer::Layer
[`Registry`]: crate::registry::Registry

---

## Filtered

`struct` · `tracing_subscriber::filter::layer_filters::Filtered`

Also reachable as `tracing_subscriber::filter::Filtered`

```rust
struct Filtered<L, F, S>
```

**Implements**: `tracing_subscriber::layer::Layer`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn filter(&self) -> &F
fn filter_mut(&mut self) -> &mut F
fn inner(&self) -> &L
fn inner_mut(&mut self) -> &mut L
fn new(layer: L, filter: F) -> Self
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn enabled(&self, metadata: &Metadata<'_>, cx: Context<'_, S>) -> bool
fn event_enabled(&self, event: &Event<'_>, cx: Context<'_, S>) -> bool
fn on_close(&self, id: span::Id, cx: Context<'_, S>)
fn on_enter(&self, id: &span::Id, cx: Context<'_, S>)
fn on_event(&self, event: &Event<'_>, cx: Context<'_, S>)
fn on_exit(&self, id: &span::Id, cx: Context<'_, S>)
fn on_follows_from(&self, span: &span::Id, follows: &span::Id, cx: Context<'_, S>)
fn on_id_change(&self, old: &span::Id, new: &span::Id, cx: Context<'_, S>)
fn on_layer(&mut self, subscriber: &mut S)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, cx: Context<'_, S>)
fn on_record(&self, span: &span::Id, values: &span::Record<'_>, cx: Context<'_, S>)
fn on_register_dispatch(&self, subscriber: &Dispatch)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
```

A [`Layer`] that wraps an inner [`Layer`] and adds a [`Filter`] which
controls what spans and events are enabled for that layer.

This is returned by the [`Layer::with_filter`] method. See the
[documentation on per-layer filtering][plf] for details.

[`Filter`]: crate::layer::Filter
[plf]: crate::layer#per-layer-filtering

---

## FilterExt

`trait` · `tracing_subscriber::filter::layer_filters::FilterExt`

Also reachable as `tracing_subscriber::filter::FilterExt`

```rust
trait FilterExt<S>: layer::Filter<S>
```

**Methods** (4)

```rust
fn and<B>(self, other: B) -> combinator::And<Self, B, S> where Self: Sized, B: layer::Filter<S>
fn boxed(self) -> Box<dyn layer::Filter<S> + Send + Sync + 'static> where Self: Sized + Send + Sync + 'static
fn not(self) -> combinator::Not<Self, S> where Self: Sized
fn or<B>(self, other: B) -> combinator::Or<Self, B, S> where Self: Sized, B: layer::Filter<S>
```

Extension trait adding [combinators] for combining [`Filter`].

[combinators]: crate::filter::combinator
[`Filter`]: crate::layer::Filter

---
