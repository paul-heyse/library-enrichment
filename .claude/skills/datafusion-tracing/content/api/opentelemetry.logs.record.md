# `opentelemetry::logs::record`

Crate `opentelemetry` · 3 public items · structured records in [`model/opentelemetry.logs.record.json`](../model/opentelemetry.logs.record.json)

## AnyValue

`enum` · `opentelemetry::logs::record::AnyValue`

Also reachable as `opentelemetry::logs::AnyValue`

```rust
enum AnyValue
```

**Variants**: `Int`, `Double`, `String`, `Boolean`, `Bytes`, `ListAny`, `Map`

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(val: i32) -> AnyValue
fn from(val: StringValue) -> AnyValue
fn from(val: &[u8]) -> AnyValue
fn from(val: String) -> AnyValue
fn from(val: u16) -> AnyValue
fn from(val: i16) -> AnyValue
fn from(val: bool) -> AnyValue
fn from(val: f32) -> AnyValue
fn from(val: u8) -> AnyValue
fn from(val: i8) -> AnyValue
fn from(val: &'static str) -> AnyValue
fn from(val: f64) -> AnyValue
fn from(val: i64) -> AnyValue
fn from(val: Cow<'static, str>) -> AnyValue
fn from(val: u32) -> AnyValue
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

Value types for representing arbitrary values in a log record.
Note: The `tracing` and `log` crates only support basic types that can be
converted to these core variants: `i64`, `f64`, `StringValue`, and `bool`.
Any complex and custom types are supported through their Debug implementation,
and converted to String. More complex types (`Bytes`, `ListAny`, and `Map`) are
included here to meet specification requirements and are available to support
custom appenders that may be implemented for other logging crates.
These types allow for handling dynamic data structures, so keep in mind the
potential performance overhead of using boxed vectors and maps in appenders.

---

## Severity

`enum` · `opentelemetry::logs::record::Severity`

Also reachable as `opentelemetry::logs::Severity`

```rust
enum Severity
```

**Variants**: `Trace`, `Trace2`, `Trace3`, `Trace4`, `Debug`, `Debug2`, `Debug3`, `Debug4`, `Info`, `Info2`, `Info3`, `Info4`, `Warn`, `Warn2`, `Warn3`, `Warn4`, `Error`, `Error2`, `Error3`, `Error4`, `Fatal`, `Fatal2`, `Fatal3`, `Fatal4`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
const fn name(&self) -> &'static str
```

A normalized severity value.

---

## LogRecord

`trait` · `opentelemetry::logs::record::LogRecord`

Also reachable as `opentelemetry::logs::LogRecord`

```rust
trait LogRecord
```

**Implementors** (1)

- `opentelemetry_sdk::logs::record::SdkLogRecord`

**Methods** (10)

```rust
fn add_attribute<K, V>(&mut self, key: K, value: V) where K: Into<Key>, V: Into<AnyValue>
fn add_attributes<I, K, V>(&mut self, attributes: I) where I: IntoIterator<Item = (K, V)>, K: Into<Key>, V: Into<AnyValue>
fn set_body(&mut self, body: AnyValue)
fn set_event_name(&mut self, name: &'static str)
fn set_observed_timestamp(&mut self, timestamp: SystemTime)
fn set_severity_number(&mut self, number: Severity)
fn set_severity_text(&mut self, text: &'static str)
fn set_target<T>(&mut self, _target: T) where T: Into<Cow<'static, str>>
fn set_timestamp(&mut self, timestamp: SystemTime)
fn set_trace_context(&mut self, trace_id: TraceId, span_id: SpanId, trace_flags: Option<TraceFlags>)
```

SDK implemented trait for managing log records

---
