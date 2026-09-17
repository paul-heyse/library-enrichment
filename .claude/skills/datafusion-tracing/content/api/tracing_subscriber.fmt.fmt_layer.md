# `tracing_subscriber::fmt::fmt_layer`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.fmt.fmt_layer.json`](../model/tracing_subscriber.fmt.fmt_layer.json)

## FmtContext

`struct` · `tracing_subscriber::fmt::fmt_layer::FmtContext`

Also reachable as `tracing_subscriber::fmt::FmtContext`

```rust
struct FmtContext<'a, S, N>
```

**Implements**: `tracing_subscriber::fmt::format::FormatFields`

**Derives**: Debug

**Methods** (10)

```rust
fn current_span(&self) -> Current
fn event_scope(&self) -> Option<registry::Scope<'_, S>> where S: for<'lookup> registry::LookupSpan<'lookup>
fn exists(&self, id: &Id) -> bool where S: for<'lookup> LookupSpan<'lookup>
fn field_format(&self) -> &N
fn lookup_current(&self) -> Option<SpanRef<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn metadata(&self, id: &Id) -> Option<&'static Metadata<'static>> where S: for<'lookup> LookupSpan<'lookup>
fn parent_span(&self) -> Option<SpanRef<'_, S>>
fn span(&self, id: &Id) -> Option<SpanRef<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn span_scope(&self, id: &Id) -> Option<registry::Scope<'_, S>> where S: for<'lookup> LookupSpan<'lookup>
fn visit_spans<E, F>(&self, f: F) -> Result<(), E> where F: FnMut(&SpanRef<'_, S>) -> Result<(), E>
```

**via `tracing_subscriber::fmt::format::FormatFields`**

```rust
fn format_fields<R: RecordFields>(&self, writer: format::Writer<'writer>, fields: R) -> fmt::Result
```

Provides the current span context to a formatter.

---

## FormattedFields

`struct` · `tracing_subscriber::fmt::fmt_layer::FormattedFields`

Also reachable as `tracing_subscriber::fmt::FormattedFields`

```rust
struct FormattedFields<E: ?Sized>
```

**Fields**: `fields`

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn as_writer(&mut self) -> format::Writer<'_>
fn new(fields: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A formatted representation of a span's fields stored in its [extensions].

Because `FormattedFields` is generic over the type of the formatter that
produced it, multiple versions of a span's formatted fields can be stored in
the [`Extensions`][extensions] type-map. This means that when multiple
formatters are in use, each can store its own formatted representation
without conflicting.

[extensions]: crate::registry::Extensions

---

## Layer

`struct` · `tracing_subscriber::fmt::fmt_layer::Layer`

Also reachable as `tracing_subscriber::fmt::Layer`

```rust
struct Layer<S, N = format::DefaultFields, E = format::Format<format::Full>, W = fn() -> io::Stdout>
```

**Implements**: `tracing_subscriber::layer::Layer`

**Derives**: Debug, Default

**Methods** (30)

```rust
fn compact(self) -> Layer<S, N, format::Format<format::Compact, T>, W> where N: for<'writer> FormatFields<'writer> + 'static
fn event_format<E2>(self, e: E2) -> Layer<S, N, E2, W> where E2: FormatEvent<S, N> + 'static
fn flatten_event(self, flatten_event: bool) -> Layer<S, format::JsonFields, format::Format<format::Json, T>, W>
fn fmt_fields<N2>(self, fmt_fields: N2) -> Layer<S, N2, E, W> where N2: for<'writer> FormatFields<'writer> + 'static
fn json(self) -> Layer<S, format::JsonFields, format::Format<format::Json, T>, W>
fn log_internal_errors(self, log_internal_errors: bool) -> Self
fn map_event_format<E2>(self, f: impl FnOnce(E) -> E2) -> Layer<S, N, E2, W> where E2: FormatEvent<S, N> + 'static
fn map_fmt_fields<N2>(self, f: impl FnOnce(N) -> N2) -> Layer<S, N2, E, W> where N2: for<'writer> FormatFields<'writer> + 'static
fn map_writer<W2>(self, f: impl FnOnce(W) -> W2) -> Layer<S, N, E, W2> where W2: for<'writer> MakeWriter<'writer> + 'static
fn new() -> Self
fn pretty(self) -> Layer<S, format::Pretty, format::Format<format::Pretty, T>, W>
fn set_ansi(&mut self, ansi: bool)
fn set_span_events(&mut self, kind: FmtSpan)
fn with_ansi(self, ansi: bool) -> Self
fn with_ansi_sanitization(self, ansi_sanitization: bool) -> Self
fn with_current_span(self, display_current_span: bool) -> Layer<S, format::JsonFields, format::Format<format::Json, T>, W>
fn with_file(self, display_filename: bool) -> Layer<S, N, format::Format<L, T>, W>
fn with_level(self, display_level: bool) -> Layer<S, N, format::Format<L, T>, W>
fn with_line_number(self, display_line_number: bool) -> Layer<S, N, format::Format<L, T>, W>
fn with_span_events(self, kind: FmtSpan) -> Self
fn with_span_list(self, display_span_list: bool) -> Layer<S, format::JsonFields, format::Format<format::Json, T>, W>
fn with_target(self, display_target: bool) -> Layer<S, N, format::Format<L, T>, W>
fn with_test_writer(self) -> Layer<S, N, E, TestWriter>
fn with_thread_ids(self, display_thread_ids: bool) -> Layer<S, N, format::Format<L, T>, W>
fn with_thread_names(self, display_thread_names: bool) -> Layer<S, N, format::Format<L, T>, W>
fn with_timer<T2>(self, timer: T2) -> Layer<S, N, format::Format<L, T2>, W>
fn with_writer<W2>(self, make_writer: W2) -> Layer<S, N, E, W2> where W2: for<'writer> MakeWriter<'writer> + 'static
fn without_time(self) -> Layer<S, N, format::Format<L, ()>, W>
fn writer(&self) -> &W
fn writer_mut(&mut self) -> &mut W
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn on_close(&self, id: Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &Id, ctx: Context<'_, S>)
fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>)
fn on_exit(&self, id: &Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

A [`Layer`] that logs formatted representations of `tracing` events.

## Examples

Constructing a layer with the default configuration:

```rust
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::prelude::*;

let subscriber = Registry::default()
    .with(fmt::Layer::default());

tracing::subscriber::set_global_default(subscriber).unwrap();
```

Overriding the layer's behavior:

```rust
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::prelude::*;

let fmt_layer = fmt::layer()
   .with_target(false) // don't include event targets when logging
   .with_level(false); // don't include event levels when logging

let subscriber = Registry::default().with(fmt_layer);
# tracing::subscriber::set_global_default(subscriber).unwrap();
```

Setting a custom event formatter:

```rust
use tracing_subscriber::fmt::{self, format, time};
use tracing_subscriber::prelude::*;

let fmt = format().with_timer(time::Uptime::default());
let fmt_layer = fmt::layer()
    .event_format(fmt)
    .with_target(false);
# let subscriber = fmt_layer.with_subscriber(tracing_subscriber::registry::Registry::default());
# tracing::subscriber::set_global_default(subscriber).unwrap();
```

[`Layer`]: super::layer::Layer

---
