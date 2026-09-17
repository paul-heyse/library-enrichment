# `tracing_futures`

Crate `tracing-futures` · 4 public items · structured records in [`model/tracing_futures.json`](../model/tracing_futures.json)

## Instrumented

`struct` · `tracing_futures::Instrumented`

```rust
struct Instrumented<T>
```

**Implements**: `core::future::future::Future`, `futures::future::Executor`, `futures::future::Future`, `futures::sink::Sink`, `futures::stream::Stream`, `futures_core::stream::Stream`, `futures_sink::Sink`, `futures_task::spawn::LocalSpawn`, `futures_task::spawn::Spawn`, `tokio_executor::executor::Executor`, `tokio_executor::typed::TypedExecutor`

**Derives**: Clone, Debug, Unpin

**Methods** (13)

```rust
fn block_on<F, R, E>(&mut self, future: F) -> Result<R, E> where F: 'static + Future<Item = R, Error = E>, R: 'static, E: 'static
fn block_on<F, R, E>(&mut self, future: F) -> Result<R, E> where F: Send + 'static + Future<Item = R, Error = E>, R: Send + 'static, E: Send + 'static
fn executor(&self) -> Instrumented<TaskExecutor>
fn handle(&self) -> Instrumented<current_thread::Handle>
fn inner(&self) -> &T
fn inner_mut(&mut self) -> &mut T
fn inner_pin_mut(Pin<&mut self>) -> Pin<&mut T>
fn inner_pin_ref(Pin<&self>) -> Pin<&T>
fn into_inner(self) -> T
fn span(&self) -> &Span
fn span_mut(&mut self) -> &mut Span
fn spawn<F>(&mut self, future: F) -> &mut Self where F: Future<Item = (), Error = ()> + 'static
fn spawn<F>(&mut self, future: F) -> &mut Self where F: Future<Item = (), Error = ()> + Send + 'static
```

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> stdlib::task::Poll<Self::Output>
```

**via `futures::future::Executor`**

```rust
fn execute(&self, future: F) -> Result<(), ExecuteError<F>>
```

**via `futures::future::Future`**

```rust
fn poll(&mut self) -> futures_01::Poll<Self::Item, Self::Error>
```

**via `futures::sink::Sink`**

```rust
fn poll_complete(&mut self) -> futures_01::Poll<(), Self::SinkError>
fn start_send(&mut self, item: Self::SinkItem) -> futures_01::StartSend<Self::SinkItem, Self::SinkError>
```

**via `futures::stream::Stream`**

```rust
fn poll(&mut self) -> futures_01::Poll<Option<Self::Item>, Self::Error>
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> futures::task::Poll<Option<Self::Item>>
```

**via `futures_sink::Sink`**

```rust
fn poll_close(Pin<&mut self>, cx: &mut Context<'_>) -> futures::task::Poll<Result<(), Self::Error>>
fn poll_flush(Pin<&mut self>, cx: &mut Context<'_>) -> futures::task::Poll<Result<(), Self::Error>>
fn poll_ready(Pin<&mut self>, cx: &mut Context<'_>) -> futures::task::Poll<Result<(), Self::Error>>
fn start_send(Pin<&mut self>, item: I) -> Result<(), Self::Error>
```

**via `futures_task::spawn::LocalSpawn`**

```rust
fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>
fn status_local(&self) -> Result<(), SpawnError>
```

**via `futures_task::spawn::Spawn`**

```rust
fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError>
fn status(&self) -> Result<(), SpawnError>
```

**via `tokio_executor::executor::Executor`**

```rust
fn spawn(&mut self, future: Box<dyn Future<Error = (), Item = ()> + Send + 'static>) -> Result<(), SpawnError>
```

**via `tokio_executor::typed::TypedExecutor`**

```rust
fn spawn(&mut self, future: F) -> Result<(), SpawnError>
fn status(&self) -> Result<(), SpawnError>
```

A future, stream, sink, or executor that has been instrumented with a `tracing` span.

---

## WithDispatch

`struct` · `tracing_futures::WithDispatch`

```rust
struct WithDispatch<T>
```

**Implements**: `core::future::future::Future`, `futures::future::Executor`, `futures::future::Future`, `futures_task::spawn::LocalSpawn`, `futures_task::spawn::Spawn`, `tokio_executor::executor::Executor`, `tokio_executor::typed::TypedExecutor`

**Derives**: Clone, Debug, Unpin

**Methods** (13)

```rust
fn block_on<F, R, E>(&mut self, future: F) -> Result<R, E> where F: Send + 'static + Future<Item = R, Error = E>, R: Send + 'static, E: Send + 'static
fn block_on<F, R, E>(&mut self, future: F) -> Result<R, E> where F: 'static + Future<Item = R, Error = E>, R: 'static, E: 'static
fn dispatch(&self) -> &Dispatch
fn executor(&self) -> WithDispatch<TaskExecutor>
fn handle(&self) -> WithDispatch<current_thread::Handle>
fn inner(&self) -> &T
fn inner_mut(&mut self) -> &mut T
fn inner_pin_mut(Pin<&mut self>) -> Pin<&mut T>
fn inner_pin_ref(Pin<&self>) -> Pin<&T>
fn into_inner(self) -> T
fn spawn<F>(&mut self, future: F) -> &mut Self where F: Future<Item = (), Error = ()> + Send + 'static
fn spawn<F>(&mut self, future: F) -> &mut Self where F: Future<Item = (), Error = ()> + 'static
fn with_dispatch<U>(&self, inner: U) -> WithDispatch<U>
```

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> stdlib::task::Poll<Self::Output>
```

**via `futures::future::Executor`**

```rust
fn execute(&self, future: F) -> Result<(), ExecuteError<F>>
```

**via `futures::future::Future`**

```rust
fn poll(&mut self) -> futures_01::Poll<Self::Item, Self::Error>
```

**via `futures_task::spawn::LocalSpawn`**

```rust
fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError>
fn status_local(&self) -> Result<(), SpawnError>
```

**via `futures_task::spawn::Spawn`**

```rust
fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError>
fn status(&self) -> Result<(), SpawnError>
```

**via `tokio_executor::executor::Executor`**

```rust
fn spawn(&mut self, future: Box<dyn Future<Error = (), Item = ()> + Send + 'static>) -> Result<(), SpawnError>
```

**via `tokio_executor::typed::TypedExecutor`**

```rust
fn spawn(&mut self, future: F) -> Result<(), SpawnError>
fn status(&self) -> Result<(), SpawnError>
```

A future, stream, sink, or executor that has been instrumented with a
`tracing` subscriber.

---

## Instrument

`trait` · `tracing_futures::Instrument`

```rust
trait Instrument: Sized
```

**Methods** (2)

```rust
fn in_current_span(self) -> Instrumented<Self>
fn instrument(self, span: Span) -> Instrumented<Self>
```

Extension trait allowing futures, streams, sinks, and executors to be
instrumented with a `tracing` [span].

[span]: https://docs.rs/tracing/latest/tracing/span/index.html

---

## WithSubscriber

`trait` · `tracing_futures::WithSubscriber`

```rust
trait WithSubscriber: Sized
```

**Methods** (2)

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>
```

Extension trait allowing futures, streams, and sinks to be instrumented with
a `tracing` [`Subscriber`].

[`Subscriber`]: https://docs.rs/tracing/latest/tracing/subscriber/trait.Subscriber.html

---
