# `opentelemetry_sdk::logs::record`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.logs.record.json`](../model/opentelemetry_sdk.logs.record.json)

## SdkLogRecord

`struct` · `opentelemetry_sdk::logs::record::SdkLogRecord`

Also reachable as `opentelemetry_sdk::logs::SdkLogRecord`

```rust
struct SdkLogRecord
```

**Implements**: `opentelemetry::logs::record::LogRecord`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn attributes_iter(&self) -> impl Iterator<Item = &(Key, AnyValue)>
fn body(&self) -> Option<&AnyValue>
fn event_name(&self) -> Option<&'static str>
fn observed_timestamp(&self) -> Option<SystemTime>
fn severity_number(&self) -> Option<Severity>
fn severity_text(&self) -> Option<&'static str>
fn target(&self) -> Option<&Cow<'static, str>>
fn timestamp(&self) -> Option<SystemTime>
fn trace_context(&self) -> Option<&TraceContext>
```

**via `opentelemetry::logs::record::LogRecord`**

```rust
fn add_attribute<K, V>(&mut self, key: K, value: V) where K: Into<Key>, V: Into<AnyValue>
fn add_attributes<I, K, V>(&mut self, attributes: I) where I: IntoIterator<Item = (K, V)>, K: Into<Key>, V: Into<AnyValue>
fn set_body(&mut self, body: AnyValue)
fn set_event_name(&mut self, name: &'static str)
fn set_observed_timestamp(&mut self, timestamp: SystemTime)
fn set_severity_number(&mut self, severity_number: Severity)
fn set_severity_text(&mut self, severity_text: &'static str)
fn set_target<T>(&mut self, _target: T) where T: Into<Cow<'static, str>>
fn set_timestamp(&mut self, timestamp: SystemTime)
fn set_trace_context(&mut self, trace_id: TraceId, span_id: SpanId, trace_flags: Option<TraceFlags>)
```

LogRecord represents all data carried by a log record, and
is provided to `LogExporter`s as input.

---

## TraceContext

`struct` · `opentelemetry_sdk::logs::record::TraceContext`

Also reachable as `opentelemetry_sdk::logs::TraceContext`

```rust
struct TraceContext
```

**Fields**: `trace_id`, `span_id`, `trace_flags`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(span_context: &SpanContext) -> Self
```

TraceContext stores the trace context for logs that have an associated
span.

---
