# `opentelemetry::trace::span_context`

Crate `opentelemetry` · 3 public items · structured records in [`model/opentelemetry.trace.span_context.json`](../model/opentelemetry.trace.span_context.json)

## TraceStateError

`enum` · `opentelemetry::trace::span_context::TraceStateError`

```rust
enum TraceStateError
```

**Variants**: `Key`, `Value`, `List`

Error returned by `TraceState` operations.

---

## SpanContext

`struct` · `opentelemetry::trace::span_context::SpanContext`

Also reachable as `opentelemetry::trace::SpanContext`

```rust
struct SpanContext
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn empty_context() -> Self
fn is_remote(&self) -> bool
fn is_sampled(&self) -> bool
fn is_valid(&self) -> bool
fn new(trace_id: TraceId, span_id: SpanId, trace_flags: TraceFlags, is_remote: bool, trace_state: TraceState) -> Self
fn span_id(&self) -> SpanId
fn trace_flags(&self) -> TraceFlags
fn trace_id(&self) -> TraceId
fn trace_state(&self) -> &TraceState
```

Immutable portion of a [`Span`] which can be serialized and propagated.

This representation conforms to the [W3C TraceContext specification].

Spans that do not have the `sampled` flag set in their [`TraceFlags`] will
be ignored by most tracing tools.

[`Span`]: crate::trace::Span
[W3C TraceContext specification]: https://www.w3.org/TR/trace-context

---

## TraceState

`struct` · `opentelemetry::trace::span_context::TraceState`

Also reachable as `opentelemetry::trace::TraceState`

```rust
struct TraceState
```

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn delete<K: Into<String>>(&self, key: K) -> Result<TraceState, TraceStateError>
fn from_key_value<T, K, V>(trace_state: T) -> Result<Self, TraceStateError> where T: IntoIterator<Item = (K, V)>, K: ToString, V: ToString
fn get(&self, key: &str) -> Option<&str>
fn header(&self) -> String
fn header_delimited(&self, entry_delimiter: &str, list_delimiter: &str) -> String
fn insert<K, V>(&self, key: K, value: V) -> Result<TraceState, TraceStateError> where K: Into<String>, V: Into<String>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

TraceState carries system-specific configuration data, represented as a list
of key-value pairs. TraceState allows multiple tracing systems to
participate in the same trace.

Please review the [W3C specification] for details on this field.

[W3C specification]: https://www.w3.org/TR/trace-context/#tracestate-header

---
