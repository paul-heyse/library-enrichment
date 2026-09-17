# `tracing_subscriber::filter::targets`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.filter.targets.json`](../model/tracing_subscriber.filter.targets.json)

## IntoIter

`struct` · `tracing_subscriber::filter::targets::IntoIter`

```rust
struct IntoIter
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An owning iterator over the [target]-[level] pairs of a `Targets` filter.

This struct is created by the `IntoIterator` trait implementation of [`Targets`].

# Examples

Merge the targets from one `Targets` with another:

```
use tracing_subscriber::filter::Targets;
use tracing_core::Level;

let mut filter = Targets::new().with_target("my_crate", Level::INFO);
let overrides = Targets::new().with_target("my_crate::interesting_module", Level::DEBUG);

filter.extend(overrides);
# drop(filter);
```

[target]: tracing_core::Metadata::target
[level]: tracing_core::Level

---

## Iter

`struct` · `tracing_subscriber::filter::targets::Iter`

```rust
struct Iter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

A borrowing iterator over the [target]-[level] pairs of a `Targets` filter.

This struct is created by [`iter`] method of [`Targets`], or from the `IntoIterator`
implementation for `&Targets`.

[target]: tracing_core::Metadata::target
[level]: tracing_core::Level
[`iter`]: Targets::iter

---

## Targets

`struct` · `tracing_subscriber::filter::targets::Targets`

Also reachable as `tracing_subscriber::filter::Targets`

```rust
struct Targets
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::str::traits::FromStr`, `tracing_subscriber::layer::Filter`, `tracing_subscriber::layer::Layer`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn default_level(&self) -> Option<LevelFilter>
fn iter(&self) -> Iter<'_>
fn new() -> Self
fn with_default(self, level: impl Into<LevelFilter>) -> Self
fn with_target(self, target: impl Into<String>, level: impl Into<LevelFilter>) -> Self
fn with_targets<T, L>(self, targets: impl IntoIterator<Item = (T, L)>) -> Self where String: From<T>, LevelFilter: From<L>
fn would_enable(&self, target: &str, level: &Level) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<I: IntoIterator<Item = (T, L)>>(&mut self, iter: I)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (T, L)>>(iter: I) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `tracing_subscriber::layer::Filter`**

```rust
fn callsite_enabled(&self, metadata: &'static Metadata<'static>) -> Interest
fn enabled(&self, metadata: &Metadata<'_>, _: &layer::Context<'_, S>) -> bool
fn max_level_hint(&self) -> Option<LevelFilter>
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn enabled(&self, metadata: &Metadata<'_>, _: layer::Context<'_, S>) -> bool
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
```

 A filter that enables or disables spans and events based on their [target]
 and [level].

 Targets are typically equal to the Rust module path of the code where the
 span or event was recorded, although they may be overridden.

 This type can be used for both [per-layer filtering][plf] (using its
 [`Filter`] implementation) and [global filtering][global] (using its
 [`Layer`] implementation).

 See the [documentation on filtering with layers][filtering] for details.

 # Filtering With `Targets`

 A `Targets` filter consists of one or more [target] prefixes, paired with
 [`LevelFilter`]s. If a span or event's [target] begins with one of those
 prefixes, and its [level] is at or below the [`LevelFilter`] enabled for
 that prefix, then the span or event will be enabled.

 This is similar to the behavior implemented by the [`env_logger` crate] in
 the `log` ecosystem.

 The [`EnvFilter`] type also provided by this crate is very similar to `Targets`,
 but is capable of a more sophisticated form of filtering where events may
 also be enabled or disabled based on the span they are recorded in.
 `Targets` can be thought of as a lighter-weight form of [`EnvFilter`] that
 can be used instead when this dynamic filtering is not required.

 # Examples

 A `Targets` filter can be constructed by programmatically adding targets and
 levels to enable:

 ```
 use tracing_subscriber::{filter, prelude::*};
 use tracing_core::Level;

 let filter = filter::Targets::new()
     // Enable the `INFO` level for anything in `my_crate`
     .with_target("my_crate", Level::INFO)
     // Enable the `DEBUG` level for a specific module.
     .with_target("my_crate::interesting_module", Level::DEBUG);

 // Build a new subscriber with the `fmt` layer using the `Targets`
 // filter we constructed above.
 tracing_subscriber::registry()
     .with(tracing_subscriber::fmt::layer())
     .with(filter)
     .init();
 ```

 [`LevelFilter::OFF`] can be used to disable a particular target:
 ```
 use tracing_subscriber::filter::{Targets, LevelFilter};
 use tracing_core::Level;

 let filter = Targets::new()
     .with_target("my_crate", Level::INFO)
     // Disable all traces from `annoying_module`.
     .with_target("my_crate::annoying_module", LevelFilter::OFF);
 # drop(filter);
 ```

 Alternatively, `Targets` implements [`std::str::FromStr`], allowing it to be
 parsed from a comma-delimited list of `target=level` pairs. For example:

 ```rust
 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 use tracing_subscriber::filter;
 use tracing_core::Level;

 let filter = "my_crate=info,my_crate::interesting_module=trace,other_crate=debug"
     .parse::<filter::Targets>()?;

 // The parsed filter is identical to a filter constructed using `with_target`:
 assert_eq!(
     filter,
     filter::Targets::new()
         .with_target("my_crate", Level::INFO)
         .with_target("my_crate::interesting_module", Level::TRACE)
         .with_target("other_crate", Level::DEBUG)
 );
 # Ok(()) }
 ```

 This is particularly useful when the list of enabled targets is configurable
 by the user at runtime.

 The `Targets` filter can be used as a [per-layer filter][plf] *and* as a
 [global filter][global]:

 ```rust
 use tracing_subscriber::{
     fmt,
     filter::{Targets, LevelFilter},
     prelude::*,
 };
 use tracing_core::Level;
 use std::{sync::Arc, fs::File};
 # fn docs() -> Result<(), Box<dyn std::error::Error>> {

 // A layer that logs events to stdout using the human-readable "pretty"
 // format.
 let stdout_log = fmt::layer().pretty();

 // A layer that logs events to a file, using the JSON format.
 let file = File::create("debug_log.json")?;
 let debug_log = fmt::layer()
     .with_writer(Arc::new(file))
     .json();

 tracing_subscriber::registry()
     // Only log INFO and above to stdout, unless the span or event
     // has the `my_crate::cool_module` target prefix.
     .with(stdout_log
         .with_filter(
             Targets::default()
                 .with_target("my_crate::cool_module", Level::DEBUG)
                 .with_default(Level::INFO)
        )
     )
     // Log everything enabled by the global filter to `debug_log.json`.
     .with(debug_log)
     // Configure a global filter for the whole subscriber stack. This will
     // control what spans and events are recorded by both the `debug_log`
     // and the `stdout_log` layers, and `stdout_log` will *additionally* be
     // filtered by its per-layer filter.
     .with(
         Targets::default()
             .with_target("my_crate", Level::TRACE)
             .with_target("other_crate", Level::INFO)
             .with_target("other_crate::annoying_module", LevelFilter::OFF)
             .with_target("third_crate", Level::DEBUG)
     ).init();
 # Ok(()) }
```

 [target]: tracing_core::Metadata::target
 [level]: tracing_core::Level
 [`Filter`]: crate::layer::Filter
 [`Layer`]: crate::layer::Layer
 [plf]: crate::layer#per-layer-filtering
 [global]: crate::layer#global-filtering
 [filtering]: crate::layer#filtering-with-layers
 [`env_logger` crate]: https://docs.rs/env_logger/0.9.0/env_logger/index.html#enabling-logging
 [`EnvFilter`]: crate::filter::EnvFilter

---
