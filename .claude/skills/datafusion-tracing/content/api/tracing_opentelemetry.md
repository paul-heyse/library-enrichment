# `tracing_opentelemetry`

Crate `tracing-opentelemetry` · 1 public items · structured records in [`model/tracing_opentelemetry.json`](../model/tracing_opentelemetry.json)

## OtelData

`struct` · `tracing_opentelemetry::OtelData`

```rust
struct OtelData
```

**Derives**: Debug

**Methods** (2)

```rust
fn span_id(&self) -> Option<opentelemetry::SpanId>
fn trace_id(&self) -> Option<opentelemetry::TraceId>
```

Per-span OpenTelemetry data tracked by this crate.

---
