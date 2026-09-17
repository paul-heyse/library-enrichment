# `tracing`

Crate `tracing` · 16 public items · structured records in [`model/tracing.json`](../model/tracing.json)

## debug

`macro` · `tracing::debug`

```rust
macro_rules! debug
```

Constructs an event at the debug level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::debug;
# fn main() {
# #[derive(Debug)] struct Position { x: f32, y: f32 }

let pos = Position { x: 3.234, y: -1.223 };

debug!(?pos.x, ?pos.y);
debug!(target: "app_events", position = ?pos, "New position");
debug!(name: "completed", position = ?pos);
# }
```

---

## debug_span

`macro` · `tracing::debug_span`

```rust
macro_rules! debug_span
```

Constructs a span at the debug level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{debug_span, span, Level};
# fn main() {
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
# }
```

```rust
# use tracing::debug_span;
# fn main() {
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

---

## enabled

`macro` · `tracing::enabled`

```rust
macro_rules! enabled
```

Checks whether a span or event is [enabled] based on the provided [metadata].

[enabled]: crate::Subscriber::enabled
[metadata]: crate::Metadata

This macro is a specialized tool: it is intended to be used prior
to an expensive computation required *just* for that event, but
*cannot* be done as part of an argument to that event, such as
when multiple events are emitted (e.g., iterating over a collection
and emitting an event for each item).

# Usage

[Subscribers] can make filtering decisions based all the data included in a
span or event's [`Metadata`]. This means that it is possible for `enabled!`
to return a _false positive_ (indicating that something would be enabled
when it actually would not be) or a _false negative_ (indicating that
something would be disabled when it would actually be enabled).

[Subscribers]: crate::subscriber::Subscriber
[`Metadata`]: crate::metadata::Metadata

This occurs when a subscriber is using a _more specific_ filter than the
metadata provided to the `enabled!` macro. Some situations that can result
in false positives or false negatives include:

- If a subscriber is using a filter which may enable a span or event based
  on field names, but `enabled!` is invoked without listing field names,
  `enabled!` may return a false negative if a specific field name would
  cause the subscriber to enable something that would otherwise be disabled.
- If a subscriber is using a filter which enables or disables specific events by
  file path and line number,  a particular event may be enabled/disabled
  even if an `enabled!` invocation with the same level, target, and fields
  indicated otherwise.
- The subscriber can choose to enable _only_ spans or _only_ events, which `enabled`
  will not reflect.

`enabled!()` requires a [level](crate::Level) argument, an optional `target:`
argument, and an optional set of field names. If the fields are not provided,
they are considered to be unknown. `enabled!` attempts to match the
syntax of `event!()` as closely as possible, which can be seen in the
examples below.

# Examples

If the current subscriber is interested in recording `DEBUG`-level spans and
events in the current file and module path, this will evaluate to true:
```rust
use tracing::{enabled, Level};

if enabled!(Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", and at the
level  `DEBUG`, this will evaluate to true:
```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", at
the level `DEBUG`, and with a field named "hello", this will evaluate
to true:

```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG, hello) {
    // some expensive work...
}
```

# Alternatives

`enabled!` queries subscribers with [`Metadata`] where
[`is_event`] and [`is_span`] both return `false`. Alternatively,
use [`event_enabled!`] or [`span_enabled!`] to ensure one of these
returns true.


[`Metadata`]: crate::Metadata
[`is_event`]: crate::Metadata::is_event
[`is_span`]: crate::Metadata::is_span
[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

---

## error

`macro` · `tracing::error`

```rust
macro_rules! error
```

Constructs an event at the error level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::error;
# fn main() {

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
# }
```

---

## error_span

`macro` · `tracing::error_span`

```rust
macro_rules! error_span
```

Constructs a span at the error level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{span, error_span, Level};
# fn main() {
error_span!("my_span");
// is equivalent to:
span!(Level::ERROR, "my_span");
# }
```

```rust
# use tracing::error_span;
# fn main() {
let span = error_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

---

## event

`macro` · `tracing::event`

```rust
macro_rules! event
```

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields.
Optionally, a format string and arguments may follow the fields; this will
be used to construct an implicit field named "message".

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros

# Examples

```rust
use tracing::{event, Level};

# fn main() {
let data = (42, "forty-two");
let private_data = "private";
let error = "a bad error";

event!(Level::ERROR, %error, "Received error");
event!(
    target: "app_events",
    Level::WARN,
    private_data,
    ?data,
    "App warning: {}",
    error
);
event!(name: "answer", Level::INFO, the_answer = data.0);
event!(Level::INFO, the_answer = data.0);
# }
```

---

## event_enabled

`macro` · `tracing::event_enabled`

```rust
macro_rules! event_enabled
```

Tests whether an event with the specified level and target would be enabled.

This is similar to [`enabled!`], but queries the current subscriber specifically for
an event, whereas [`enabled!`] queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`].

# Examples

```rust
# use tracing::{event_enabled, Level};
if event_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if event_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if event_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```

[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

---

## info

`macro` · `tracing::info`

```rust
macro_rules! info
```

Constructs an event at the info level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::info;
# // this is so the test will still work in no-std mode
# #[derive(Debug)]
# pub struct Ipv4Addr;
# impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
# fn main() {
# struct Connection { port: u32, speed: f32 }
use tracing::field;

let addr = Ipv4Addr::new(127, 0, 0, 1);
let conn = Connection { port: 40, speed: 3.20 };

info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
info!(name: "completed", "completed connection to {:?}", addr);
# }
```

---

## info_span

`macro` · `tracing::info_span`

```rust
macro_rules! info_span
```

Constructs a span at the info level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{span, info_span, Level};
# fn main() {
info_span!("my_span");
// is equivalent to:
span!(Level::INFO, "my_span");
# }
```

```rust
# use tracing::info_span;
# fn main() {
let span = info_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

---

## record_all

`macro` · `tracing::record_all`

```rust
macro_rules! record_all
```

Records multiple values on a span in a single call. As with recording
individual values, all fields must be declared when the span is created.

This macro supports two optional sigils:
- `%` uses the Display implementation.
- `?` uses the Debug implementation.

For more details, see the [top-level documentation][lib].

[lib]: tracing/#recording-fields

# Examples

```
# use tracing::{field, info_span, record_all};
let span = info_span!("my span", field1 = field::Empty, field2 = field::Empty, field3 = field::Empty).entered();
record_all!(span, field1 = ?"1", field2 = %"2", field3 = 3);
```

---

## span

`macro` · `tracing::span`

```rust
macro_rules! span
```

Constructs a new span.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros

# Examples

Creating a new span:
```
# use tracing::{span, Level};
# fn main() {
let span = span!(Level::TRACE, "my span");
let _enter = span.enter();
// do work inside the span...
# }
```

---

## span_enabled

`macro` · `tracing::span_enabled`

```rust
macro_rules! span_enabled
```

Tests whether a span with the specified level and target would be enabled.

This is similar to [`enabled!`], but queries the current subscriber specifically for
an event, whereas [`enabled!`] queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`].

# Examples

```rust
# use tracing::{span_enabled, Level};
if span_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if span_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if span_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```

[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

---

## trace

`macro` · `tracing::trace`

```rust
macro_rules! trace
```

Constructs an event at the trace level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::trace;
# #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
# impl Position {
# const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
# fn dist(&self, other: Position) -> f32 {
#    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
#    (x + y).sqrt()
# }
# }
# fn main() {
let pos = Position { x: 3.234, y: -1.223 };
let origin_dist = pos.dist(Position::ORIGIN);

trace!(position = ?pos, ?origin_dist);
trace!(
    target: "app_events",
    position = ?pos,
    "x is {} and y is {}",
    if pos.x >= 0.0 { "positive" } else { "negative" },
    if pos.y >= 0.0 { "positive" } else { "negative" }
);
trace!(name: "completed", position = ?pos);
# }
```

---

## trace_span

`macro` · `tracing::trace_span`

```rust
macro_rules! trace_span
```

Constructs a span at the trace level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
# }
```

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

---

## warn

`macro` · `tracing::warn`

```rust
macro_rules! warn
```

Constructs an event at the warn level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::warn;
# fn main() {

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
# }
```

---

## warn_span

`macro` · `tracing::warn_span`

```rust
macro_rules! warn_span
```

Constructs a span at the warn level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{warn_span, span, Level};
# fn main() {
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
# }
```

```rust
use tracing::warn_span;
# fn main() {
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

---
