# `opentelemetry_sdk::runtime`

Crate `opentelemetry_sdk` · 6 public items · structured records in [`model/opentelemetry_sdk.runtime.json`](../model/opentelemetry_sdk.runtime.json)

## TrySendError

`enum` · `opentelemetry_sdk::runtime::TrySendError`

```rust
enum TrySendError
```

**Variants**: `ChannelFull`, `ChannelClosed`, `Other`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error returned by a [`TrySend`] implementation.

---

## Tokio

`struct` · `opentelemetry_sdk::runtime::Tokio`

```rust
struct Tokio
```

**Implements**: `opentelemetry_sdk::runtime::Runtime`, `opentelemetry_sdk::runtime::RuntimeChannel`

**Derives**: Clone, Debug

**via `opentelemetry_sdk::runtime::Runtime`**

```rust
fn delay(&self, duration: Duration) -> impl Future<Output = ()> + Send + 'static
fn spawn<F>(&self, future: F) where F: Future<Output = ()> + Send + 'static
```

**via `opentelemetry_sdk::runtime::RuntimeChannel`**

```rust
fn batch_message_channel<T: Debug + Send>(&self, capacity: usize) -> (Self::Sender<T>, Self::Receiver<T>)
```

Runtime implementation, which works with Tokio's multi thread runtime.

---

## TokioCurrentThread

`struct` · `opentelemetry_sdk::runtime::TokioCurrentThread`

```rust
struct TokioCurrentThread
```

**Implements**: `opentelemetry_sdk::runtime::Runtime`, `opentelemetry_sdk::runtime::RuntimeChannel`

**Derives**: Clone, Debug

**via `opentelemetry_sdk::runtime::Runtime`**

```rust
fn delay(&self, duration: Duration) -> impl Future<Output = ()> + Send + 'static
fn spawn<F>(&self, future: F) where F: Future<Output = ()> + Send + 'static
```

**via `opentelemetry_sdk::runtime::RuntimeChannel`**

```rust
fn batch_message_channel<T: Debug + Send>(&self, capacity: usize) -> (Self::Sender<T>, Self::Receiver<T>)
```

Runtime implementation, which works with Tokio's current thread runtime.

---

## Runtime

`trait` · `opentelemetry_sdk::runtime::Runtime`

```rust
trait Runtime: Clone + Send + Sync + 'static
```

**Implementors** (2)

- `opentelemetry_sdk::runtime::Tokio`
- `opentelemetry_sdk::runtime::TokioCurrentThread`

**Methods** (2)

```rust
fn delay(&self, duration: Duration) -> impl Future<Output = ()> + Send + 'static
fn spawn<F>(&self, future: F) where F: Future<Output = ()> + Send + 'static
```

A runtime is an abstraction of an async runtime like [Tokio]. It allows
OpenTelemetry to work with any current and hopefully future runtime implementations.

[Tokio]: https://crates.io/crates/tokio

# Note

OpenTelemetry expects a *multithreaded* runtime because its types can move across threads.
For this reason, this trait requires the `Send` and `Sync` bounds. Single-threaded runtimes
can implement this trait in a way that spawns the tasks on the same thread as the calling code.

---

## RuntimeChannel

`trait` · `opentelemetry_sdk::runtime::RuntimeChannel`

```rust
trait RuntimeChannel: Runtime
```

**Implementors** (2)

- `opentelemetry_sdk::runtime::Tokio`
- `opentelemetry_sdk::runtime::TokioCurrentThread`

**Methods** (1)

```rust
fn batch_message_channel<T: Debug + Send>(&self, capacity: usize) -> (Self::Sender<T>, Self::Receiver<T>)
```

`RuntimeChannel` is an extension to [`Runtime`]. Currently, it provides a
channel that is used by the [log] and [span] batch processors.

[log]: crate::logs::BatchLogProcessor
[span]: crate::trace::BatchSpanProcessor

---

## TrySend

`trait` · `opentelemetry_sdk::runtime::TrySend`

```rust
trait TrySend: Sync + Send
```

**Implementors** (1)

- `tokio::sync::mpsc::bounded::Sender`

**Methods** (1)

```rust
fn try_send(&self, item: Self::Message) -> Result<(), TrySendError>
```

TrySend is an abstraction of `Sender` that is capable of sending messages through a reference.

---
