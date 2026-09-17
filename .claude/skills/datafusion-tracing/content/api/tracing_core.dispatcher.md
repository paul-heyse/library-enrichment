# `tracing_core::dispatcher`

Crate `tracing-core` · 8 public items · structured records in [`model/tracing_core.dispatcher.json`](../model/tracing_core.dispatcher.json)

## get_default

`function` · `tracing_core::dispatcher::get_default`

Also reachable as `tracing::dispatcher::get_default`

```rust
fn get_default<T, F>(f: F) -> T where F: FnMut(&Dispatch) -> T
```

Executes a closure with a reference to this thread's current [dispatcher].

Note that calls to `get_default` should not be nested; if this function is
called while inside of another `get_default`, that closure will be provided
with `Dispatch::none` rather than the previously set dispatcher.

[dispatcher]: super::dispatcher::Dispatch

---

## set_default

`function` · `tracing_core::dispatcher::set_default`

Also reachable as `tracing::dispatcher::set_default`

```rust
fn set_default(dispatcher: &Dispatch) -> DefaultGuard
```

Sets the dispatch as the default dispatch for the duration of the lifetime
of the returned DefaultGuard

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>

[`set_global_default`]: set_global_default

---

## set_global_default

`function` · `tracing_core::dispatcher::set_global_default`

Also reachable as `tracing::dispatcher::set_global_default`

```rust
fn set_global_default(dispatcher: Dispatch) -> Result<(), SetGlobalDefaultError>
```

Sets this dispatch as the global default for the duration of the entire program.
Will be used as a fallback if no thread-local dispatch has been set in a thread
(using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail.
Returns `Err` if the global default has already been set.

<div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
    <strong>Warning</strong>: In general, libraries should <em>not</em> call
    <code>set_global_default()</code>! Doing so will cause conflicts when
    executables that depend on the library try to set the default later.
</pre></div>

[span]: super::span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

---

## with_default

`function` · `tracing_core::dispatcher::with_default`

Also reachable as `tracing::dispatcher::with_default`

```rust
fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T
```

Sets this dispatch as the default for the duration of a closure.

The default dispatcher is used when creating a new [span] or
[`Event`].

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This function required the Rust standard library.
    <code>no_std</code> users should use <a href="fn.set_global_default.html">
    <code>set_global_default</code></a> instead.
</pre>

[span]: super::span
[`Subscriber`]: super::subscriber::Subscriber
[`Event`]: super::event::Event

---

## DefaultGuard

`struct` · `tracing_core::dispatcher::DefaultGuard`

Also reachable as `tracing::dispatcher::DefaultGuard`, `tracing::subscriber::DefaultGuard`

```rust
struct DefaultGuard
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A guard that resets the current default dispatcher to the prior
default dispatcher when dropped.

---

## Dispatch

`struct` · `tracing_core::dispatcher::Dispatch`

Also reachable as `tracing::Dispatch`, `tracing::dispatcher::Dispatch`, `tracing_core::Dispatch`

```rust
struct Dispatch
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default

**Methods** (17)

```rust
fn clone_span(&self, id: &span::Id) -> span::Id
fn current_span(&self) -> span::Current
fn downcast_ref<T: Any>(&self) -> Option<&T>
fn downgrade(&self) -> WeakDispatch
fn drop_span(&self, id: span::Id)
fn enabled(&self, metadata: &Metadata<'_>) -> bool
fn enter(&self, span: &span::Id)
fn event(&self, event: &Event<'_>)
fn exit(&self, span: &span::Id)
fn is<T: Any>(&self) -> bool
fn new<S>(subscriber: S) -> Self where S: Subscriber + Send + Sync + 'static
fn new_span(&self, span: &span::Attributes<'_>) -> span::Id
fn none() -> Self
fn record(&self, span: &span::Id, values: &span::Record<'_>)
fn record_follows_from(&self, span: &span::Id, follows: &span::Id)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> subscriber::Interest
fn try_close(&self, id: span::Id) -> bool
```

**via `core::convert::From`**

```rust
fn from(subscriber: S) -> Self
```

`Dispatch` trace data to a [`Subscriber`].

---

## SetGlobalDefaultError

`struct` · `tracing_core::dispatcher::SetGlobalDefaultError`

Also reachable as `tracing::dispatcher::SetGlobalDefaultError`, `tracing::subscriber::SetGlobalDefaultError`

```rust
struct SetGlobalDefaultError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Returned if setting the global dispatcher fails.

---

## WeakDispatch

`struct` · `tracing_core::dispatcher::WeakDispatch`

Also reachable as `tracing::dispatcher::WeakDispatch`

```rust
struct WeakDispatch
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn upgrade(&self) -> Option<Dispatch>
```

`WeakDispatch` is a version of [`Dispatch`] that holds a non-owning reference
to a [`Subscriber`].

The `Subscriber` may be accessed by calling [`WeakDispatch::upgrade`],
which returns an `Option<Dispatch>`. If all [`Dispatch`] clones that point
at the `Subscriber` have been dropped, [`WeakDispatch::upgrade`] will return
`None`. Otherwise, it will return `Some(Dispatch)`.

A `WeakDispatch` may be created from a [`Dispatch`] by calling the
[`Dispatch::downgrade`] method. The primary use for creating a
[`WeakDispatch`] is to allow a Subscriber` to hold a cyclical reference to
itself without creating a memory leak. See [here] for details.

This type is analogous to the [`std::sync::Weak`] type, but for a
[`Dispatch`] rather than an [`Arc`].

[`Arc`]: std::sync::Arc
[here]: Subscriber#avoiding-memory-leaks

---
