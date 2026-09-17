# `tracing_core::metadata`

Crate `tracing-core` · 6 public items · structured records in [`model/tracing_core.metadata.json`](../model/tracing_core.metadata.json)

## Kind

`struct` · `tracing_core::metadata::Kind`

Also reachable as `tracing_core::Kind`

```rust
struct Kind
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
const fn hint(self) -> Self
fn is_event(&self) -> bool
fn is_hint(&self) -> bool
fn is_span(&self) -> bool
```

Indicates whether the callsite is a span or event.

---

## Level

`struct` · `tracing_core::metadata::Level`

Also reachable as `tracing::Level`, `tracing_core::Level`

```rust
struct Level
```

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, ParseLevelError>
```

Describes the level of verbosity of a span or event.

# Comparing Levels

`Level` implements the [`PartialOrd`] and [`Ord`] traits, allowing two
`Level`s to be compared to determine which is considered more or less
verbose. Levels which are more verbose are considered "greater than" levels
which are less verbose, with [`Level::ERROR`] considered the lowest, and
[`Level::TRACE`] considered the highest.

For example:
```
use tracing_core::Level;

assert!(Level::TRACE > Level::DEBUG);
assert!(Level::ERROR < Level::WARN);
assert!(Level::INFO <= Level::DEBUG);
assert_eq!(Level::TRACE, Level::TRACE);
```

# Filtering

`Level`s are typically used to implement filtering that determines which
spans and events are enabled. Depending on the use case, more or less
verbose diagnostics may be desired. For example, when running in
development, [`DEBUG`]-level traces may be enabled by default. When running in
production, only [`INFO`]-level and lower traces might be enabled. Libraries
may include very verbose diagnostics at the [`DEBUG`] and/or [`TRACE`] levels.
Applications using those libraries typically chose to ignore those traces. However, when
debugging an issue involving said libraries, it may be useful to temporarily
enable the more verbose traces.

The [`LevelFilter`] type is provided to enable filtering traces by
verbosity. `Level`s can be compared against [`LevelFilter`]s, and
[`LevelFilter`] has a variant for each `Level`, which compares analogously
to that level. In addition, [`LevelFilter`] adds a [`LevelFilter::OFF`]
variant, which is considered "less verbose" than every other `Level`. This is
intended to allow filters to completely disable tracing in a particular context.

For example:
```
use tracing_core::{Level, LevelFilter};

assert!(LevelFilter::OFF < Level::TRACE);
assert!(LevelFilter::TRACE > Level::DEBUG);
assert!(LevelFilter::ERROR < Level::WARN);
assert!(LevelFilter::INFO <= Level::DEBUG);
assert!(LevelFilter::INFO >= Level::INFO);
```

## Examples

Below is a simple example of how a [`Subscriber`] could implement filtering through
a [`LevelFilter`]. When a span or event is recorded, the [`Subscriber::enabled`] method
compares the span or event's `Level` against the configured [`LevelFilter`].
The optional [`Subscriber::max_level_hint`] method can also be implemented to allow spans
and events above a maximum verbosity level to be skipped more efficiently,
often improving performance in short-lived programs.

```
use tracing_core::{span, Event, Level, LevelFilter, Subscriber, Metadata};
# use tracing_core::span::{Id, Record, Current};

#[derive(Debug)]
pub struct MySubscriber {
    /// The most verbose level that this subscriber will enable.
    max_level: LevelFilter,

    // ...
}

impl MySubscriber {
    /// Returns a new `MySubscriber` which will record spans and events up to
    /// `max_level`.
    pub fn with_max_level(max_level: LevelFilter) -> Self {
        Self {
            max_level,
            // ...
        }
    }
}
impl Subscriber for MySubscriber {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        // A span or event is enabled if it is at or below the configured
        // maximum level.
        meta.level() <= &self.max_level
    }

    // This optional method returns the most verbose level that this
    // subscriber will enable. Although implementing this method is not
    // *required*, it permits additional optimizations when it is provided,
    // allowing spans and events above the max level to be skipped
    // more efficiently.
    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(self.max_level)
    }

    // Implement the rest of the subscriber...
    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        // ...
        # drop(span); Id::from_u64(1)
    }

    fn event(&self, event: &Event<'_>) {
        // ...
        # drop(event);
    }

    // ...
    # fn enter(&self, _: &Id) {}
    # fn exit(&self, _: &Id) {}
    # fn record(&self, _: &Id, _: &Record<'_>) {}
    # fn record_follows_from(&self, _: &Id, _: &Id) {}
}
```

It is worth noting that the `tracing-subscriber` crate provides [additional
APIs][envfilter] for performing more sophisticated filtering, such as
enabling different levels based on which module or crate a span or event is
recorded in.

[`DEBUG`]: Level::DEBUG
[`INFO`]: Level::INFO
[`TRACE`]: Level::TRACE
[`Subscriber::enabled`]: crate::subscriber::Subscriber::enabled
[`Subscriber::max_level_hint`]: crate::subscriber::Subscriber::max_level_hint
[`Subscriber`]: crate::subscriber::Subscriber
[envfilter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html

---

## LevelFilter

`struct` · `tracing_core::metadata::LevelFilter`

Also reachable as `tracing::level_filters::LevelFilter`, `tracing_core::LevelFilter`, `tracing_subscriber::filter::LevelFilter`

```rust
struct LevelFilter
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `tracing_subscriber::layer::Filter`, `tracing_subscriber::layer::Layer`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn current() -> Self
const fn from_level(level: Level) -> Self
const fn into_level(self) -> Option<Level>
```

**via `core::convert::From`**

```rust
fn from(level: Level) -> Self
fn from(level: Option<Level>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(from: &str) -> Result<Self, Self::Err>
```

A filter comparable to a verbosity [`Level`].

If a [`Level`] is considered less than or equal to a `LevelFilter`, it
should be considered enabled; if greater than the `LevelFilter`, that level
is disabled. See [`LevelFilter::current`] for more details.

Note that this is essentially identical to the `Level` type, but with the
addition of an [`OFF`] level that completely disables all trace
instrumentation.

See the documentation for the [`Level`] type to see how `Level`s
and `LevelFilter`s interact.

[`OFF`]: LevelFilter::OFF

---

## Metadata

`struct` · `tracing_core::metadata::Metadata`

Also reachable as `tracing::Metadata`, `tracing_core::Metadata`

```rust
struct Metadata<'a>
```

**Derives**: Debug, Eq, PartialEq

**Methods** (11)

```rust
fn callsite(&self) -> callsite::Identifier
fn fields(&self) -> &field::FieldSet
fn file(&self) -> Option<&'a str>
fn is_event(&self) -> bool
fn is_span(&self) -> bool
fn level(&self) -> &Level
fn line(&self) -> Option<u32>
fn module_path(&self) -> Option<&'a str>
fn name(&self) -> &'static str
const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self
fn target(&self) -> &'a str
```

Metadata describing a [span] or [event].

All spans and events have the following metadata:
- A [name], represented as a static string.
- A [target], a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`] type for details.
- The names of the [fields] defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`]s when filtering spans and events, and it
may also be used as part of their data payload.

When created by the `event!` or `span!` macro, the metadata describing a
particular event or span is constructed statically and exists as a single
static instance. Thus, the overhead of creating the metadata is
_significantly_ lower than that of creating the actual span. Therefore,
filtering is based on metadata, rather than on the constructed span.

## Equality

In well-behaved applications, two `Metadata` with equal
[callsite identifiers] will be equal in all other ways (i.e., have the same
`name`, `target`, etc.). Consequently, in release builds, [`Metadata::eq`]
*only* checks that its arguments have equal callsites. However, the equality
of `Metadata`'s other fields is checked in debug builds.

[span]: super::span
[event]: super::event
[name]: Self::name
[target]: Self::target
[fields]: Self::fields
[verbosity level]: Self::level
[file name]: Self::file
[line number]: Self::line
[module path]: Self::module_path
[`Subscriber`]: super::subscriber::Subscriber
[callsite identifiers]: Self::callsite

---

## ParseLevelError

`struct` · `tracing_core::metadata::ParseLevelError`

```rust
struct ParseLevelError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Returned if parsing a `Level` fails.

---

## ParseLevelFilterError

`struct` · `tracing_core::metadata::ParseLevelFilterError`

Also reachable as `tracing::level_filters::ParseLevelFilterError`, `tracing_subscriber::filter::LevelParseError`

```rust
struct ParseLevelFilterError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Indicates that a string could not be parsed to a valid level.

---
