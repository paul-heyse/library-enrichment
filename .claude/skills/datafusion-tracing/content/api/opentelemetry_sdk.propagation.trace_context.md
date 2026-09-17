# `opentelemetry_sdk::propagation::trace_context`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.propagation.trace_context.json`](../model/opentelemetry_sdk.propagation.trace_context.json)

## TraceContextPropagator

`struct` · `opentelemetry_sdk::propagation::trace_context::TraceContextPropagator`

Also reachable as `opentelemetry_sdk::propagation::TraceContextPropagator`

```rust
struct TraceContextPropagator
```

**Implements**: `opentelemetry::propagation::text_map_propagator::TextMapPropagator`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `opentelemetry::propagation::text_map_propagator::TextMapPropagator`**

```rust
fn extract_with_context(&self, cx: &Context, extractor: &dyn Extractor) -> Context
fn fields(&self) -> FieldIter<'_>
fn inject_context(&self, cx: &Context, injector: &mut dyn Injector)
```

Propagates `SpanContext`s in [W3C TraceContext] format under `traceparent` and `tracestate` header.

The `traceparent` header represents the incoming request in a
tracing system in a common format, understood by all vendors.
Here’s an example of a `traceparent` header.

`traceparent: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01`

The `traceparent` HTTP header field identifies the incoming request in a
tracing system. It has four fields:

   - version
   - trace-id
   - parent-id
   - trace-flags

The `tracestate` header provides additional vendor-specific trace
identification information across different distributed tracing systems.
Here's an example of a `tracestate` header

`tracestate: vendorname1=opaqueValue1,vendorname2=opaqueValue2`

See the [w3c trace-context docs] for more details.

[w3c trace-context docs]: https://w3c.github.io/trace-context/
[W3C TraceContext]: https://www.w3.org/TR/trace-context/

---
