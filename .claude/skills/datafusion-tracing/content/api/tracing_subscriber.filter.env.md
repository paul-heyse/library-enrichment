# `tracing_subscriber::filter::env`

Crate `tracing-subscriber` · 2 public items · structured records in [`model/tracing_subscriber.filter.env.json`](../model/tracing_subscriber.filter.env.json)

## EnvFilter

`struct` · `tracing_subscriber::filter::env::EnvFilter`

Also reachable as `tracing_subscriber::EnvFilter`, `tracing_subscriber::filter::EnvFilter`

```rust
struct EnvFilter
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `tracing_subscriber::layer::Filter`, `tracing_subscriber::layer::Layer`

**Derives**: Clone, Debug, Default

**Methods** (15)

```rust
fn add_directive(self, directive: Directive) -> Self
fn builder() -> Builder
fn enabled<S>(&self, metadata: &Metadata<'_>, _: Context<'_, S>) -> bool
fn from_default_env() -> Self
fn from_env<A: AsRef<str>>(env: A) -> Self
fn max_level_hint(&self) -> Option<LevelFilter>
fn new<S: AsRef<str>>(directives: S) -> Self
fn on_close<S>(&self, id: span::Id, _: Context<'_, S>)
fn on_enter<S>(&self, id: &span::Id, _: Context<'_, S>)
fn on_exit<S>(&self, id: &span::Id, _: Context<'_, S>)
fn on_new_span<S>(&self, attrs: &span::Attributes<'_>, id: &span::Id, _: Context<'_, S>)
fn on_record<S>(&self, id: &span::Id, values: &span::Record<'_>, _: Context<'_, S>)
fn try_from_default_env() -> Result<Self, FromEnvError>
fn try_from_env<A: AsRef<str>>(env: A) -> Result<Self, FromEnvError>
fn try_new<S: AsRef<str>>(dirs: S) -> Result<Self, filter::directive::ParseError>
```

**via `core::convert::From`**

```rust
fn from(s: S) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(spec: &str) -> Result<Self, Self::Err>
```

**via `tracing_subscriber::layer::Filter`**

```rust
fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest
fn enabled(&self, meta: &Metadata<'_>, ctx: &Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
fn on_close(&self, id: span::Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)
fn on_record(&self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool
fn on_close(&self, id: span::Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)
fn on_record(&self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
```

A [`Layer`] which filters spans and events based on a set of filter
directives.

`EnvFilter` implements both the [`Layer`](#impl-Layer<S>) and [`Filter`] traits, so it may
be used for both [global filtering][global] and [per-layer filtering][plf],
respectively. See [the documentation on filtering with `Layer`s][filtering]
for details.

The [`Targets`] type implements a similar form of filtering, but without the
ability to dynamically enable events based on the current span context, and
without filtering on field values. When these features are not required,
[`Targets`] provides a lighter-weight alternative to [`EnvFilter`].

# Directives

A filter consists of one or more comma-separated directives which match on [`Span`]s and [`Event`]s.
Each directive may have a corresponding maximum verbosity [`level`] which
enables (e.g., _selects for_) spans and events that match. Like `log`,
`tracing` considers less exclusive levels (like `trace` or `info`) to be more
verbose than more exclusive levels (like `error` or `warn`).

The directive syntax is similar to that of [`env_logger`]'s. At a high level, the syntax for directives
consists of several parts:

```text
target[span{field=value}]=level
```

Each component (`target`, `span`, `field`, `value`, and `level`) will be covered in turn.

- `target` matches the event or span's target. In general, this is the module path and/or crate name.
  Examples of targets `h2`, `tokio::net`, or `tide::server`. For more information on targets,
  please refer to [`Metadata`]'s documentation.
- `span` matches on the span's name. If a `span` directive is provided alongside a `target`,
  the `span` directive will match on spans _within_ the `target`.
- `field` matches on [fields] within spans. Field names can also be supplied without a `value`
  and will match on any [`Span`] or [`Event`] that has a field with that name.
  For example: `[span{field=\"value\"}]=debug`, `[{field}]=trace`.
- `value` matches on the value of a span's field. If a value is a numeric literal or a bool,
  it will match _only_ on that value. Otherwise, this filter matches the
  [`std::fmt::Debug`] output from the value.
- `level` sets a maximum verbosity level accepted by this directive.

When a field value directive (`[{<FIELD NAME>=<FIELD_VALUE>}]=...`) matches a
value's [`std::fmt::Debug`] output (i.e., the field value in the directive
is not a `bool`, `i64`, `u64`, or `f64` literal), the matched pattern may be
interpreted as either a regular expression or as the precise expected
output of the field's [`std::fmt::Debug`] implementation. By default, these
filters are interpreted as regular expressions, but this can be disabled
using the [`Builder::with_regex`] builder method to use precise matching
instead.

When field value filters are interpreted as regular expressions, the
[`regex` crate's regular expression syntax][re-syntax] is supported.

**Note**: When filters are constructed from potentially untrusted inputs,
[disabling regular expression matching](Builder::with_regex) is strongly
recommended.

## Usage Notes

- The portion of the directive which is included within the square brackets is `tracing`-specific.
- Any portion of the directive can be omitted.
    - The sole exception are the `field` and `value` directives. If a `value` is provided,
      a `field` must _also_ be provided. However, the converse does not hold, as fields can
      be matched without a value.
- If only a level is provided, it will set the maximum level for all `Span`s and `Event`s
  that are not enabled by other filters.
- A directive without a level will enable anything that it matches. This is equivalent to `=trace`.
- When a crate has a dash in its name, the default target for events will be the
  crate's module path as it appears in Rust. This means every dash will be replaced
  with an underscore.
- A dash in a target will only appear when being specified explicitly:
  `tracing::info!(target: "target-name", ...);`

## Example Syntax

- `tokio::net=info` will enable all spans or events that:
   - have the `tokio::net` target,
   - at the level `info` or above.
- `warn,tokio::net=info` will enable all spans and events that:
   - are at the level `warn` or above, *or*
   - have the `tokio::net` target at the level `info` or above.
- `my_crate[span_a]=trace` will enable all spans and events that:
   - are within the `span_a` span or named `span_a` _if_ `span_a` has the target `my_crate`,
   - at the level `trace` or above.
- `[span_b{name=\"bob\"}]` will enable all spans or event that:
   - have _any_ target,
   - are inside a span named `span_b`,
   - which has a field named `name` with value `bob`,
   - at _any_ level.

# Examples

Parsing an `EnvFilter` from the [default environment
variable](EnvFilter::from_default_env) (`RUST_LOG`):

```
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_default_env())
    .init();
```

Parsing an `EnvFilter` [from a user-provided environment
variable](EnvFilter::from_env):

```
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_env("MYAPP_LOG"))
    .init();
```

Using `EnvFilter` as a [per-layer filter][plf] to filter only a single
[`Layer`]:

```
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// Parse an `EnvFilter` configuration from the `RUST_LOG`
// environment variable.
let filter = EnvFilter::from_default_env();

// Apply the filter to this layer *only*.
let filtered_layer = fmt::layer().with_filter(filter);

// Some other layer, whose output we don't want to filter.
let unfiltered_layer = // ...
    # fmt::layer();

tracing_subscriber::registry()
    .with(filtered_layer)
    .with(unfiltered_layer)
    .init();
```
# Constructing `EnvFilter`s

An `EnvFilter` is be constructed by parsing a string containing one or more
directives. The [`EnvFilter::new`] constructor parses an `EnvFilter` from a
string, ignoring any invalid directives, while [`EnvFilter::try_new`]
returns an error if invalid directives are encountered. Similarly, the
[`EnvFilter::from_env`] and [`EnvFilter::try_from_env`] constructors parse
an `EnvFilter` from the value of the provided environment variable, with
lossy and strict validation, respectively.

A [builder](EnvFilter::builder) interface is available to set additional
configuration options prior to parsing an `EnvFilter`. See the [`Builder`
type's documentation](Builder) for details on the options that can be
configured using the builder.

[`Span`]: tracing_core::span
[fields]: tracing_core::Field
[`Event`]: tracing_core::Event
[`level`]: tracing_core::Level
[`Metadata`]: tracing_core::Metadata
[`Targets`]: crate::filter::Targets
[`env_logger`]: https://crates.io/crates/env_logger
[`Filter`]: #impl-Filter<S>
[global]: crate::layer#global-filtering
[plf]: crate::layer#per-layer-filtering
[filtering]: crate::layer#filtering-with-layers
[re-syntax]: https://docs.rs/regex/1.11.1/regex/#syntax

---

## FromEnvError

`struct` · `tracing_subscriber::filter::env::FromEnvError`

Also reachable as `tracing_subscriber::filter::FromEnvError`

```rust
struct FromEnvError
```

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(p: filter::directive::ParseError) -> Self
fn from(v: env::VarError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Indicates that an error occurred while parsing a `EnvFilter` from an
environment variable.

---
