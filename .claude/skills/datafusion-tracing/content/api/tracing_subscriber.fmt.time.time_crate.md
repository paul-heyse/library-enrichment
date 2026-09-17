# `tracing_subscriber::fmt::time::time_crate`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.fmt.time.time_crate.json`](../model/tracing_subscriber.fmt.time.time_crate.json)

## LocalTime

`struct` · `tracing_subscriber::fmt::time::time_crate::LocalTime`

Also reachable as `tracing_subscriber::fmt::time::LocalTime`

```rust
struct LocalTime<F>
```

**Implements**: `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn new(format: F) -> Self
fn rfc_3339() -> Self
```

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result
```

Formats the current [local time] using a [formatter] from the [`time` crate].

To format the current [UTC time] instead, use the [`UtcTime`] type.

<div class="example-wrap" style="display:inline-block">
<pre class="compile_fail" style="white-space:normal;font:inherit;">
    <strong>Warning</strong>: The <a href = "https://docs.rs/time/0.3/time/"><code>time</code>
    crate</a> must be compiled with <code>--cfg unsound_local_offset</code> in order to use
    local timestamps. When this cfg is not enabled, local timestamps cannot be recorded, and
    events will be logged without timestamps.

   Alternatively, [`OffsetTime`] can log with a local offset if it is initialized early.

   See the <a href="https://docs.rs/time/0.3.4/time/#feature-flags"><code>time</code>
   documentation</a> for more details.
</pre></div>

[local time]: time::OffsetDateTime::now_local
[UTC time]:     time::OffsetDateTime::now_utc
[formatter]:    time::formatting::Formattable
[`time` crate]: time

---

## OffsetTime

`struct` · `tracing_subscriber::fmt::time::time_crate::OffsetTime`

Also reachable as `tracing_subscriber::fmt::time::OffsetTime`

```rust
struct OffsetTime<F>
```

**Implements**: `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn local_rfc_3339() -> Result<Self, time::error::IndeterminateOffset>
fn new(offset: time::UtcOffset, format: F) -> Self
```

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result
```

Formats the current time using a fixed offset and a [formatter] from the [`time` crate].

This is typically used as an alternative to [`LocalTime`]. `LocalTime` determines the offset
every time it formats a message, which may be unsound or fail. With `OffsetTime`, the offset is
determined once. This makes it possible to do so while the program is still single-threaded and
handle any errors. However, this also means the offset cannot change while the program is
running (the offset will not change across DST changes).

[formatter]: time::formatting::Formattable
[`time` crate]: time

---

## UtcTime

`struct` · `tracing_subscriber::fmt::time::time_crate::UtcTime`

Also reachable as `tracing_subscriber::fmt::time::UtcTime`

```rust
struct UtcTime<F>
```

**Implements**: `tracing_subscriber::fmt::time::FormatTime`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn new(format: F) -> Self
fn rfc_3339() -> Self
```

**via `tracing_subscriber::fmt::time::FormatTime`**

```rust
fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result
```

Formats the current [UTC time] using a [formatter] from the [`time` crate].

To format the current [local time] instead, use the [`LocalTime`] type.

[local time]: time::OffsetDateTime::now_local
[UTC time]:     time::OffsetDateTime::now_utc
[formatter]:    time::formatting::Formattable
[`time` crate]: time

---
