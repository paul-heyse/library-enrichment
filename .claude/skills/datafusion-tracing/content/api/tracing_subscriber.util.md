# `tracing_subscriber::util`

Crate `tracing-subscriber` · 2 public items · structured records in [`model/tracing_subscriber.util.json`](../model/tracing_subscriber.util.json)

## TryInitError

`struct` · `tracing_subscriber::util::TryInitError`

```rust
struct TryInitError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Error returned by [`try_init`](SubscriberInitExt::try_init) if a global default subscriber could not be initialized.

---

## SubscriberInitExt

`trait` · `tracing_subscriber::util::SubscriberInitExt`

```rust
trait SubscriberInitExt where Self: Into<tracing_core::dispatcher::Dispatch>
```

**Methods** (3)

```rust
fn init(self)
fn set_default(self) -> dispatcher::DefaultGuard
fn try_init(self) -> Result<(), TryInitError>
```

Extension trait adding utility methods for subscriber initialization.

This trait provides extension methods to make configuring and setting a
[default subscriber] more ergonomic. It is automatically implemented for all
types that can be converted into a [trace dispatcher]. Since `Dispatch`
implements `From<T>` for all `T: Subscriber`, all `Subscriber`
implementations will implement this extension trait as well. Types which
can be converted into `Subscriber`s, such as builders that construct a
`Subscriber`, may implement `Into<Dispatch>`, and will also receive an
implementation of this trait.

[default subscriber]: https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber
[trace dispatcher]: https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html

---
