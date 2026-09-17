# `tracing::instrument`

Crate `tracing` · 4 public items · structured records in [`model/tracing.instrument.json`](../model/tracing.instrument.json)

## Instrumented

`struct` · `tracing::instrument::Instrumented`

```rust
struct Instrumented<T>
```

**Implements**: `core::future::future::Future`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Unpin

**Methods** (7)

```rust
fn inner(&self) -> &T
fn inner_mut(&mut self) -> &mut T
fn inner_pin_mut(Pin<&mut self>) -> Pin<&mut T>
fn inner_pin_ref(Pin<&self>) -> Pin<&T>
fn into_inner(self) -> T
fn span(&self) -> &Span
fn span_mut(&mut self) -> &mut Span
```

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A [`Future`] that has been instrumented with a `tracing` [`Span`].

This type is returned by the [`Instrument`] extension trait. See that
trait's documentation for details.

[`Future`]: std::future::Future
[`Span`]: crate::Span

---

## WithDispatch

`struct` · `tracing::instrument::WithDispatch`

```rust
struct WithDispatch<T>
```

**Implements**: `core::future::future::Future`

**Derives**: Clone, Debug, Unpin

**Methods** (6)

```rust
fn dispatcher(&self) -> &Dispatch
fn inner(&self) -> &T
fn inner_mut(&mut self) -> &mut T
fn inner_pin_mut(Pin<&mut self>) -> Pin<&mut T>
fn inner_pin_ref(Pin<&self>) -> Pin<&T>
fn into_inner(self) -> T
```

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

A [`Future`] that has been instrumented with a `tracing` [`Subscriber`].

This type is returned by the [`WithSubscriber`] extension trait. See that
trait's documentation for details.

[`Future`]: std::future::Future
[`Subscriber`]: crate::Subscriber

---

## Instrument

`trait` · `tracing::instrument::Instrument`

Also reachable as `tracing::Instrument`

```rust
trait Instrument: Sized
```

**Methods** (2)

```rust
fn in_current_span(self) -> Instrumented<Self>
fn instrument(self, span: Span) -> Instrumented<Self>
```

Attaches spans to a [`std::future::Future`].

Extension trait allowing futures to be
instrumented with a `tracing` [span].

[span]: super::Span

---

## WithSubscriber

`trait` · `tracing::instrument::WithSubscriber`

```rust
trait WithSubscriber: Sized
```

**Methods** (2)

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>
```

Extension trait allowing futures to be instrumented with
a `tracing` [`Subscriber`](crate::Subscriber).

---
