# `tracing_subscriber::filter::layer_filters::combinator`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.filter.layer_filters.combinator.json`](../model/tracing_subscriber.filter.layer_filters.combinator.json)

## And

`struct` · `tracing_subscriber::filter::layer_filters::combinator::And`

Also reachable as `tracing_subscriber::filter::combinator::And`

```rust
struct And<A, B, S>
```

**Implements**: `tracing_subscriber::layer::Filter`

**Derives**: Clone, Debug

**via `tracing_subscriber::layer::Filter`**

```rust
fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest
fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool
fn event_enabled(&self, event: &tracing_core::Event<'_>, cx: &Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
fn on_close(&self, id: Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &Id, ctx: Context<'_, S>)
fn on_exit(&self, id: &Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

Combines two [`Filter`]s so that spans and events are enabled if and only if
*both* filters return `true`.

This type is typically returned by the [`FilterExt::and`] method. See that
method's documentation for details.

[`Filter`]: crate::layer::Filter
[`FilterExt::and`]: crate::filter::FilterExt::and

---

## Not

`struct` · `tracing_subscriber::filter::layer_filters::combinator::Not`

Also reachable as `tracing_subscriber::filter::combinator::Not`

```rust
struct Not<A, S>
```

**Implements**: `tracing_subscriber::layer::Filter`

**Derives**: Clone, Debug

**via `tracing_subscriber::layer::Filter`**

```rust
fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest
fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool
fn event_enabled(&self, event: &tracing_core::Event<'_>, cx: &Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
fn on_close(&self, id: Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &Id, ctx: Context<'_, S>)
fn on_exit(&self, id: &Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

Inverts the result of a [`Filter`].

If the wrapped filter would enable a span or event, it will be disabled. If
it would disable a span or event, that span or event will be enabled.

This type is typically returned by the [`FilterExt::not`] method. See that
method's documentation for details.

[`Filter`]: crate::layer::Filter
[`FilterExt::not`]: crate::filter::FilterExt::not

---

## Or

`struct` · `tracing_subscriber::filter::layer_filters::combinator::Or`

Also reachable as `tracing_subscriber::filter::combinator::Or`

```rust
struct Or<A, B, S>
```

**Implements**: `tracing_subscriber::layer::Filter`

**Derives**: Clone, Debug

**via `tracing_subscriber::layer::Filter`**

```rust
fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest
fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool
fn event_enabled(&self, event: &tracing_core::Event<'_>, cx: &Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
fn on_close(&self, id: Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &Id, ctx: Context<'_, S>)
fn on_exit(&self, id: &Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

Combines two [`Filter`]s so that spans and events are enabled if *either* filter
returns `true`.

This type is typically returned by the [`FilterExt::or`] method. See that
method's documentation for details.

[`Filter`]: crate::layer::Filter
[`FilterExt::or`]: crate::filter::FilterExt::or

---
