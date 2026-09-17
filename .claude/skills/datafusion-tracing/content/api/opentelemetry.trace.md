# `opentelemetry::trace`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.trace.json`](../model/opentelemetry.trace.json)

## Event

`struct` · `opentelemetry::trace::Event`

```rust
struct Event
```

**Fields**: `name`, `timestamp`, `attributes`, `dropped_attributes_count`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new<T: Into<Cow<'static, str>>>(name: T, timestamp: time::SystemTime, attributes: Vec<KeyValue>, dropped_attributes_count: u32) -> Self
fn with_name<T: Into<Cow<'static, str>>>(name: T) -> Self
```

Events record things that happened during a [`Span`]'s lifetime.

---

## Link

`struct` · `opentelemetry::trace::Link`

```rust
struct Link
```

**Fields**: `span_context`, `attributes`, `dropped_attributes_count`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(span_context: SpanContext, attributes: Vec<KeyValue>, dropped_attributes_count: u32) -> Self
fn with_context(span_context: SpanContext) -> Self
```

Link is the relationship between two Spans.

The relationship can be within the same trace or across different traces.

---
