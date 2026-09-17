# `tracing_subscriber::reload`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.reload.json`](../model/tracing_subscriber.reload.json)

## Error

`struct` · `tracing_subscriber::reload::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn is_dropped(&self) -> bool
fn is_poisoned(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Indicates that an error occurred when reloading a layer.

---

## Handle

`struct` · `tracing_subscriber::reload::Handle`

```rust
struct Handle<L, S>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn clone_current(&self) -> Option<L> where L: Clone
fn modify(&self, f: impl FnOnce(&mut L)) -> Result<(), Error>
fn reload(&self, new_value: impl Into<L>) -> Result<(), Error>
fn with_current<T>(&self, f: impl FnOnce(&L) -> T) -> Result<T, Error>
```

Allows reloading the state of an associated [`Layer`](crate::layer::Layer).

---

## Layer

`struct` · `tracing_subscriber::reload::Layer`

```rust
struct Layer<L, S>
```

**Implements**: `tracing_subscriber::layer::Filter`, `tracing_subscriber::layer::Layer`

**Derives**: Debug

**Methods** (2)

```rust
fn handle(&self) -> Handle<L, S>
fn new(inner: L) -> (Self, Handle<L, S>)
```

**via `tracing_subscriber::layer::Filter`**

```rust
fn callsite_enabled(&self, metadata: &'static Metadata<'static>) -> Interest
fn enabled(&self, metadata: &Metadata<'_>, ctx: &layer::Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
fn on_close(&self, id: span::Id, ctx: layer::Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: layer::Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: layer::Context<'_, S>)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>)
fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: layer::Context<'_, S>)
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn enabled(&self, metadata: &Metadata<'_>, ctx: layer::Context<'_, S>) -> bool
fn event_enabled(&self, event: &Event<'_>, ctx: layer::Context<'_, S>) -> bool
fn on_close(&self, id: span::Id, ctx: layer::Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: layer::Context<'_, S>)
fn on_event(&self, event: &Event<'_>, ctx: layer::Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: layer::Context<'_, S>)
fn on_follows_from(&self, span: &span::Id, follows: &span::Id, ctx: layer::Context<'_, S>)
fn on_id_change(&self, old: &span::Id, new: &span::Id, ctx: layer::Context<'_, S>)
fn on_layer(&mut self, subscriber: &mut S)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>)
fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: layer::Context<'_, S>)
fn on_register_dispatch(&self, subscriber: &Dispatch)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
```

Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.

---
