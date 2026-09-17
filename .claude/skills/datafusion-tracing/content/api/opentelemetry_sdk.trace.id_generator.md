# `opentelemetry_sdk::trace::id_generator`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.id_generator.json`](../model/opentelemetry_sdk.trace.id_generator.json)

## RandomIdGenerator

`struct` · `opentelemetry_sdk::trace::id_generator::RandomIdGenerator`

Also reachable as `opentelemetry_sdk::trace::RandomIdGenerator`

```rust
struct RandomIdGenerator
```

**Implements**: `opentelemetry_sdk::trace::id_generator::IdGenerator`

**Derives**: Clone, Debug, Default

**via `opentelemetry_sdk::trace::id_generator::IdGenerator`**

```rust
fn new_span_id(&self) -> SpanId
fn new_trace_id(&self) -> TraceId
```

Default [`IdGenerator`] implementation.

Generates Trace and Span ids using a random number generator.

---

## IdGenerator

`trait` · `opentelemetry_sdk::trace::id_generator::IdGenerator`

Also reachable as `opentelemetry_sdk::trace::IdGenerator`

```rust
trait IdGenerator: Send + Sync + fmt::Debug
```

**Implementors** (1)

- `opentelemetry_sdk::trace::id_generator::RandomIdGenerator`

**Methods** (2)

```rust
fn new_span_id(&self) -> SpanId
fn new_trace_id(&self) -> TraceId
```

Interface for generating IDs

---
