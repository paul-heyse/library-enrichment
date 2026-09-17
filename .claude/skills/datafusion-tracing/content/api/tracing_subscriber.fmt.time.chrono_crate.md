# `tracing_subscriber::fmt::time::chrono_crate`

Crate `tracing-subscriber` · 2 public items · structured records in [`model/tracing_subscriber.fmt.time.chrono_crate.json`](../model/tracing_subscriber.fmt.time.chrono_crate.json)

## ChronoLocal

`struct` · `tracing_subscriber::fmt::time::chrono_crate::ChronoLocal`

Also reachable as `tracing_subscriber::fmt::time::ChronoLocal`

```rust
struct ChronoLocal
```

**Implements**: `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(format_string: String) -> Self
fn rfc_3339() -> Self
```

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> alloc::fmt::Result
```

Formats [local time]s and [UTC time]s with `FormatTime` implementations
that use the [`chrono` crate].

[local time]: [`chrono::offset::Local`]
[UTC time]: [`chrono::offset::Utc`]
[`chrono` crate]: [`chrono`]
Formats the current [local time] using a [formatter] from the [`chrono`] crate.

[local time]: chrono::Local::now()
[formatter]: chrono::format

---

## ChronoUtc

`struct` · `tracing_subscriber::fmt::time::chrono_crate::ChronoUtc`

Also reachable as `tracing_subscriber::fmt::time::ChronoUtc`

```rust
struct ChronoUtc
```

**Implements**: `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(format_string: String) -> Self
fn rfc_3339() -> Self
```

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> alloc::fmt::Result
```

Formats the current [UTC time] using a [formatter] from the [`chrono`] crate.

[UTC time]: chrono::Utc::now()
[formatter]: chrono::format

---
