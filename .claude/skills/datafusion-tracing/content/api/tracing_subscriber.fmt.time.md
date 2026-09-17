# `tracing_subscriber::fmt::time`

Crate `tracing-subscriber` · 5 public items · structured records in [`model/tracing_subscriber.fmt.time.json`](../model/tracing_subscriber.fmt.time.json)

## time

`function` · `tracing_subscriber::fmt::time::time`

Also reachable as `tracing_subscriber::fmt::time`

```rust
fn time() -> SystemTime
```

Returns a new `SystemTime` timestamp provider.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
# fn timer() -> tracing_subscriber::fmt::time::SystemTime {
tracing_subscriber::fmt::time::SystemTime::default()
# }
```

---

## uptime

`function` · `tracing_subscriber::fmt::time::uptime`

```rust
fn uptime() -> Uptime
```

Returns a new `Uptime` timestamp provider.

With this timer, timestamps will be formatted with the amount of time
elapsed since the timestamp provider was constructed.

This can then be configured further to determine how timestamps should be
configured.

This is equivalent to calling
```rust
# fn timer() -> tracing_subscriber::fmt::time::Uptime {
tracing_subscriber::fmt::time::Uptime::default()
# }
```

---

## SystemTime

`struct` · `tracing_subscriber::fmt::time::SystemTime`

```rust
struct SystemTime
```

**Implements**: `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result
```

Retrieve and print the current wall-clock time.

---

## Uptime

`struct` · `tracing_subscriber::fmt::time::Uptime`

```rust
struct Uptime
```

**Implements**: `core::convert::From`, `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(epoch: Instant) -> Self
```

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result
```

Retrieve and print the relative elapsed wall-clock time since an epoch.

The `Default` implementation for `Uptime` makes the epoch the current time.

---

## FormatTime

`trait` · `tracing_subscriber::fmt::time::FormatTime`

```rust
trait FormatTime
```

**Implementors** (7)

- `tracing_subscriber::fmt::time::SystemTime`
- `tracing_subscriber::fmt::time::Uptime`
- `tracing_subscriber::fmt::time::chrono_crate::ChronoLocal`
- `tracing_subscriber::fmt::time::chrono_crate::ChronoUtc`
- `tracing_subscriber::fmt::time::time_crate::LocalTime`
- `tracing_subscriber::fmt::time::time_crate::OffsetTime`
- `tracing_subscriber::fmt::time::time_crate::UtcTime`

**Methods** (1)

```rust
fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result
```

A type that can measure and format the current time.

This trait is used by `Format` to include a timestamp with each `Event` when it is logged.

Notable default implementations of this trait are `SystemTime` and `()`. The former prints the
current time as reported by `std::time::SystemTime`, and the latter does not print the current
time at all. `FormatTime` is also automatically implemented for any function pointer with the
appropriate signature.

The full list of provided implementations can be found in [`time`].

[`time`]: self

---
