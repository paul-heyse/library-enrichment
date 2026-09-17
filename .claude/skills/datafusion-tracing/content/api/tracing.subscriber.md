# `tracing::subscriber`

Crate `tracing` · 3 public items · structured records in [`model/tracing.subscriber.json`](../model/tracing.subscriber.json)

## set_default

`function` · `tracing::subscriber::set_default`

```rust
fn set_default<S>(subscriber: S) -> DefaultGuard where S: Subscriber + Send + Sync + 'static
```

Sets the [`Subscriber`] as the default for the current thread for the
duration of the lifetime of the returned [`DefaultGuard`].

The default subscriber is used when creating a new [`Span`] or [`Event`].

[`Span`]: super::span::Span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event
[`DefaultGuard`]: super::dispatcher::DefaultGuard

---

## set_global_default

`function` · `tracing::subscriber::set_global_default`

```rust
fn set_global_default<S>(subscriber: S) -> Result<(), SetGlobalDefaultError> where S: Subscriber + Send + Sync + 'static
```

Sets this subscriber as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns whether the initialization was successful.

Note: Libraries should *NOT* call `set_global_default()`! That will cause conflicts when
executables try to set them later.

[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

---

## with_default

`function` · `tracing::subscriber::with_default`

```rust
fn with_default<T, S>(subscriber: S, f: impl FnOnce() -> T) -> T where S: Subscriber + Send + Sync + 'static
```

Sets this [`Subscriber`] as the default for the current thread for the
duration of a closure.

The default subscriber is used when creating a new [`Span`] or
[`Event`].


[`Span`]: super::span::Span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

---
