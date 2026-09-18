# `arrow_flight::streams`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.streams.json`](../model/arrow_flight.streams.json)

## FallibleRequestStream

`struct` · `arrow_flight::streams::FallibleRequestStream`

Also reachable as `arrow_flight::sql::FallibleRequestStream`

```rust
struct FallibleRequestStream<T, E>
```

**Implements**: `futures_core::stream::Stream`

**Methods** (1)

```rust
fn new(sender: Sender<E>, fallible_stream: Pin<Box<dyn Stream<Item = std::result::Result<T, E>> + Send + 'static>>) -> Self
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(std::pin::Pin<&mut self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.streams.FallibleRequestStream.md).


Wrapper around a fallible stream (one that returns errors) that makes it infallible.

Any errors encountered in the stream are ignored are sent to the provided
oneshot sender.

This can be used to accept a stream of `Result<_>` from a client API and send
them to the remote server that wants only the successful results.

---
