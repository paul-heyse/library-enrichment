# `opentelemetry_sdk::trace::tracer`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.trace.tracer.json`](../model/opentelemetry_sdk.trace.tracer.json)

## SdkTracer

`struct` · `opentelemetry_sdk::trace::tracer::SdkTracer`

Also reachable as `opentelemetry_sdk::trace::SdkTracer`, `opentelemetry_sdk::trace::Tracer`

```rust
struct SdkTracer
```

**Implements**: `opentelemetry::trace::tracer::Tracer`

**Derives**: Clone, Debug

**via `opentelemetry::trace::tracer::Tracer`**

```rust
fn build_with_context(&self, builder: SpanBuilder, parent_cx: &Context) -> Self::Span
```

`Tracer` implementation to create and manage spans

---
