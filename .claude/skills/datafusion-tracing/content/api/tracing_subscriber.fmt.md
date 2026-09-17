# `tracing_subscriber::fmt`

Crate `tracing-subscriber` · 7 public items · structured records in [`model/tracing_subscriber.fmt.json`](../model/tracing_subscriber.fmt.json)

## fmt

`function` · `tracing_subscriber::fmt::fmt`

Also reachable as `tracing_subscriber::fmt`

```rust
fn fmt() -> SubscriberBuilder
```

Returns a new [`SubscriberBuilder`] for configuring a [formatting subscriber].

This is essentially shorthand for [`SubscriberBuilder::default()]`.

# Examples

Using [`init`] to set the default subscriber:

```rust
tracing_subscriber::fmt().init();
```

Configuring the output format:

```rust

tracing_subscriber::fmt()
    // Configure formatting settings.
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    // Set the subscriber as the default.
    .init();
```

[`try_init`] returns an error if the default subscriber could not be set:

```rust
use std::error::Error;

fn init_subscriber() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        // Configure the subscriber to emit logs in JSON format.
        .json()
        // Configure the subscriber to flatten event fields in the output JSON objects.
        .flatten_event(true)
        // Set the subscriber as the default, returning an error if this fails.
        .try_init()?;

    Ok(())
}
```

Rather than setting the subscriber as the default, [`finish`] _returns_ the
constructed subscriber, which may then be passed to other functions:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .compact()
    .finish();

tracing::subscriber::with_default(subscriber, || {
    // the subscriber will only be set as the default
    // inside this closure...
})
```

[formatting subscriber]: Subscriber
[`SubscriberBuilder::default()`]: SubscriberBuilder::default
[`init`]: SubscriberBuilder::init()
[`try_init`]: SubscriberBuilder::try_init()
[`finish`]: SubscriberBuilder::finish()

---

## init

`function` · `tracing_subscriber::fmt::init`

```rust
fn init()
```

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable].

The configuration of the subscriber initialized by this function
depends on what [feature flags](crate#feature-flags) are enabled.

If the `tracing-log` feature is enabled, this will also install
the LogTracer to convert `Log` records into `tracing` `Event`s.

If the `env-filter` feature is enabled, this is shorthand for

```rust
# use tracing_subscriber::EnvFilter;
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
```

# Panics
Panics if the initialization was unsuccessful, likely because a
global subscriber was already installed by another call to `try_init`.

[`RUST_LOG` environment variable]: crate::filter::EnvFilter::DEFAULT_ENV

---

## layer

`function` · `tracing_subscriber::fmt::layer`

```rust
fn layer<S>() -> Layer<S>
```

Returns a new [formatting layer] that can be [composed] with other layers to
construct a [`Subscriber`].

This is a shorthand for the equivalent [`Layer::default()`] function.

[formatting layer]: Layer
[composed]: crate::layer
[`Layer::default()`]: Layer::default

---

## try_init

`function` · `tracing_subscriber::fmt::try_init`

```rust
fn try_init() -> Result<(), alloc::boxed::Box<dyn Error + Send + Sync + 'static>>
```

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable],
if one is not already set.

If the `tracing-log` feature is enabled, this will also install
the [`LogTracer`] to convert `log` records into `tracing` `Event`s.

This is shorthand for

```rust
# fn doc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
tracing_subscriber::fmt().try_init()
# }
```


# Errors

Returns an Error if the initialization was unsuccessful,
likely because a global subscriber was already installed by another
call to `try_init`.

[`LogTracer`]:
    https://docs.rs/tracing-log/0.1.0/tracing_log/struct.LogTracer.html
[`RUST_LOG` environment variable]: crate::filter::EnvFilter::DEFAULT_ENV

---

## Subscriber

`struct` · `tracing_subscriber::fmt::Subscriber`

Also reachable as `tracing_subscriber::FmtSubscriber`

```rust
struct Subscriber<N = format::DefaultFields, E = format::Format<format::Full>, F = filter::LevelFilter, W = fn() -> io::Stdout>
```

**Implements**: `tracing_core::subscriber::Subscriber`, `tracing_subscriber::registry::LookupSpan`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn builder() -> SubscriberBuilder
fn new() -> Self
```

**via `tracing_core::subscriber::Subscriber`**

```rust
fn clone_span(&self, id: &span::Id) -> span::Id
fn current_span(&self) -> span::Current
unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()>
fn enabled(&self, meta: &Metadata<'_>) -> bool
fn enter(&self, id: &span::Id)
fn event(&self, event: &Event<'_>)
fn event_enabled(&self, event: &Event<'_>) -> bool
fn exit(&self, id: &span::Id)
fn max_level_hint(&self) -> Option<tracing_core::LevelFilter>
fn new_span(&self, attrs: &span::Attributes<'_>) -> span::Id
fn record(&self, span: &span::Id, values: &span::Record<'_>)
fn record_follows_from(&self, span: &span::Id, follows: &span::Id)
fn register_callsite(&self, meta: &'static Metadata<'static>) -> Interest
fn try_close(&self, id: span::Id) -> bool
```

**via `tracing_subscriber::registry::LookupSpan`**

```rust
fn span_data(&'a self, id: &span::Id) -> Option<Self::Data>
```

A `Subscriber` that logs formatted representations of `tracing` events.

This consists of an inner `Formatter` wrapped in a layer that performs filtering.

---

## SubscriberBuilder

`struct` · `tracing_subscriber::fmt::SubscriberBuilder`

```rust
struct SubscriberBuilder<N = format::DefaultFields, E = format::Format<format::Full>, F = filter::LevelFilter, W = fn() -> io::Stdout>
```

**Derives**: Debug, Default

**Methods** (32)

```rust
fn compact(self) -> SubscriberBuilder<N, format::Format<format::Compact, T>, F, W> where N: for<'writer> FormatFields<'writer> + 'static
fn event_format<E2>(self, fmt_event: E2) -> SubscriberBuilder<N, E2, F, W> where E2: FormatEvent<Registry, N> + 'static, N: for<'writer> FormatFields<'writer> + 'static, W: for<'writer> MakeWriter<'writer> + 'static
fn finish(self) -> Subscriber<N, E, F, W>
fn flatten_event(self, flatten_event: bool) -> SubscriberBuilder<format::JsonFields, format::Format<format::Json, T>, F, W>
fn fmt_fields<N2>(self, fmt_fields: N2) -> SubscriberBuilder<N2, E, F, W> where N2: for<'writer> FormatFields<'writer> + 'static
fn init(self)
fn json(self) -> SubscriberBuilder<format::JsonFields, format::Format<format::Json, T>, F, W> where N: for<'writer> FormatFields<'writer> + 'static
fn log_internal_errors(self, log_internal_errors: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn map_event_format<E2>(self, f: impl FnOnce(E) -> E2) -> SubscriberBuilder<N, E2, F, W> where E2: FormatEvent<Registry, N> + 'static, N: for<'writer> FormatFields<'writer> + 'static, W: for<'writer> MakeWriter<'writer> + 'static
fn map_fmt_fields<N2>(self, f: impl FnOnce(N) -> N2) -> SubscriberBuilder<N2, E, F, W> where N2: for<'writer> FormatFields<'writer> + 'static
fn map_writer<W2>(self, f: impl FnOnce(W) -> W2) -> SubscriberBuilder<N, E, F, W2> where W2: for<'writer> MakeWriter<'writer> + 'static
fn pretty(self) -> SubscriberBuilder<format::Pretty, format::Format<format::Pretty, T>, F, W>
fn reload_handle(&self) -> reload::Handle<EnvFilter, Formatter<N, E, W>>
fn try_init(self) -> Result<(), Box<dyn Error + Send + Sync + 'static>>
fn with_ansi(self, ansi: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_ansi_sanitization(self, ansi_sanitization: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_current_span(self, display_current_span: bool) -> SubscriberBuilder<format::JsonFields, format::Format<format::Json, T>, F, W>
fn with_env_filter(self, filter: impl Into<EnvFilter>) -> SubscriberBuilder<N, E, EnvFilter, W> where Formatter<N, E, W>: tracing_core::Subscriber + 'static
fn with_file(self, display_filename: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_filter_reloading(self) -> SubscriberBuilder<N, E, reload::Layer<EnvFilter, Formatter<N, E, W>>, W>
fn with_level(self, display_level: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_line_number(self, display_line_number: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_max_level(self, filter: impl Into<LevelFilter>) -> SubscriberBuilder<N, E, LevelFilter, W>
fn with_span_events(self, kind: format::FmtSpan) -> Self
fn with_span_list(self, display_span_list: bool) -> SubscriberBuilder<format::JsonFields, format::Format<format::Json, T>, F, W>
fn with_target(self, display_target: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_test_writer(self) -> SubscriberBuilder<N, E, F, TestWriter>
fn with_thread_ids(self, display_thread_ids: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_thread_names(self, display_thread_names: bool) -> SubscriberBuilder<N, format::Format<L, T>, F, W>
fn with_timer<T2>(self, timer: T2) -> SubscriberBuilder<N, format::Format<L, T2>, F, W>
fn with_writer<W2>(self, make_writer: W2) -> SubscriberBuilder<N, E, F, W2> where W2: for<'writer> MakeWriter<'writer> + 'static
fn without_time(self) -> SubscriberBuilder<N, format::Format<L, ()>, F, W>
```

Configures and constructs `Subscriber`s.

---

## Formatter

`type_alias` · `tracing_subscriber::fmt::Formatter`

```rust
type Formatter<N = format::DefaultFields, E = format::Format<format::Full>, W = fn() -> io::Stdout> = layer::Layered<fmt_layer::Layer<registry::Registry, N, E, W>, registry::Registry>
```

A `Subscriber` that logs formatted representations of `tracing` events.
This type only logs formatted events; it does not perform any filtering.

---
