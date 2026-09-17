# `tracing_core::subscriber`

Crate `tracing-core` · 3 public items · structured records in [`model/tracing_core.subscriber.json`](../model/tracing_core.subscriber.json)

## Interest

`struct` · `tracing_core::subscriber::Interest`

Also reachable as `tracing::subscriber::Interest`, `tracing_core::Interest`

```rust
struct Interest
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn always() -> Self
fn is_always(&self) -> bool
fn is_never(&self) -> bool
fn is_sometimes(&self) -> bool
fn never() -> Self
fn sometimes() -> Self
```

Indicates a [`Subscriber`]'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their [`register_callsite`] methods
in order to determine whether that span should be enabled or disabled.

[`Subscriber`]: super::Subscriber
[`register_callsite`]: super::Subscriber::register_callsite

---

## NoSubscriber

`struct` · `tracing_core::subscriber::NoSubscriber`

Also reachable as `tracing::subscriber::NoSubscriber`

```rust
struct NoSubscriber
```

**Implements**: `tracing_core::subscriber::Subscriber`

**Derives**: Clone, Copy, Debug, Default

**Methods** (1)

```rust
const fn new() -> Self
```

**via `tracing_core::subscriber::Subscriber`**

```rust
fn enabled(&self, _metadata: &Metadata<'_>) -> bool
fn enter(&self, _span: &span::Id)
fn event(&self, _event: &Event<'_>)
fn exit(&self, _span: &span::Id)
fn new_span(&self, _: &span::Attributes<'_>) -> span::Id
fn record(&self, _span: &span::Id, _values: &span::Record<'_>)
fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id)
fn register_callsite(&self, _: &'static Metadata<'static>) -> Interest
```

A no-op [`Subscriber`].

[`NoSubscriber`] implements the [`Subscriber`] trait by never being enabled,
never being interested in any callsite, and dropping all spans and events.

---

## Subscriber

`trait` · `tracing_core::subscriber::Subscriber`

Also reachable as `tracing::Subscriber`, `tracing::subscriber::Subscriber`, `tracing_core::Subscriber`

```rust
trait Subscriber: 'static
```

**Implementors** (6)

- `alloc::boxed::Box`
- `alloc::sync::Arc`
- `tracing_core::subscriber::NoSubscriber`
- `tracing_subscriber::fmt::Subscriber`
- `tracing_subscriber::layer::layered::Layered`
- `tracing_subscriber::registry::sharded::Registry`

**Methods** (16)

```rust
fn clone_span(&self, id: &span::Id) -> span::Id
fn current_span(&self) -> span::Current
unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()>
fn drop_span(&self, _id: span::Id)
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

Trait representing the functions required to collect trace data.

Crates that provide implementations of methods for collecting or recording
trace data should implement the `Subscriber` interface. This trait is
intended to represent fundamental primitives for collecting trace events and
spans — other libraries may offer utility functions and types to make
subscriber implementations more modular or improve the ergonomics of writing
subscribers.

A subscriber is responsible for the following:
- Registering new spans as they are created, and providing them with span
  IDs. Implicitly, this means the subscriber may determine the strategy for
  determining span equality.
- Recording the attachment of field values and follows-from annotations to
  spans.
- Filtering spans and events, and determining when those filters must be
  invalidated.
- Observing spans as they are entered, exited, and closed, and events as
  they occur.

When a span is entered or exited, the subscriber is provided only with the
[ID] with which it tagged that span when it was created. This means
that it is up to the subscriber to determine whether and how span _data_ —
the fields and metadata describing the span — should be stored. The
[`new_span`] function is called when a new span is created, and at that
point, the subscriber _may_ choose to store the associated data if it will
be referenced again. However, if the data has already been recorded and will
not be needed by the implementations of `enter` and `exit`, the subscriber
may freely discard that data without allocating space to store it.

## Overriding default impls

Some trait methods on `Subscriber` have default implementations, either in
order to reduce the surface area of implementing `Subscriber`, or for
backward-compatibility reasons. However, many subscribers will likely want
to override these default implementations.

The following methods are likely of interest:

- [`register_callsite`] is called once for each callsite from which a span
  event may originate, and returns an [`Interest`] value describing whether or
  not the subscriber wishes to see events or spans from that callsite. By
  default, it calls [`enabled`], and returns `Interest::always()` if
  `enabled` returns true, or `Interest::never()` if enabled returns false.
  However, if the subscriber's interest can change dynamically at runtime,
  it may want to override this function to return `Interest::sometimes()`.
  Additionally, subscribers which wish to perform a behaviour once for each
  callsite, such as allocating storage for data related to that callsite,
  can perform it in `register_callsite`.

  See also the [documentation on the callsite registry][cs-reg] for details
  on [`register_callsite`].

- [`event_enabled`] is called once before every call to the [`event`]
  method. This can be used to implement filtering on events once their field
  values are known, but before any processing is done in the `event` method.
- [`clone_span`] is called every time a span ID is cloned, and [`try_close`]
  is called when a span ID is dropped. By default, these functions do
  nothing. However, they can be used to implement reference counting for
  spans, allowing subscribers to free storage for span data and to determine
  when a span has _closed_ permanently (rather than being exited).
  Subscribers which store per-span data or which need to track span closures
  should override these functions together.

[ID]: super::span::Id
[`new_span`]: Subscriber::new_span
[`register_callsite`]: Subscriber::register_callsite
[`enabled`]: Subscriber::enabled
[`clone_span`]: Subscriber::clone_span
[`try_close`]: Subscriber::try_close
[cs-reg]: crate::callsite#registering-callsites
[`event`]: Subscriber::event
[`event_enabled`]: Subscriber::event_enabled

---
