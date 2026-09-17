# `opentelemetry_sdk::trace::span_limit`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.trace.span_limit.json`](../model/opentelemetry_sdk.trace.span_limit.json)

## SpanLimits

`struct` · `opentelemetry_sdk::trace::span_limit::SpanLimits`

Also reachable as `opentelemetry_sdk::trace::SpanLimits`

```rust
struct SpanLimits
```

**Fields**: `max_events_per_span`, `max_attributes_per_span`, `max_links_per_span`, `max_attributes_per_event`, `max_attributes_per_link`

**Derives**: Clone, Copy, Debug, Default

Span limit configuration to keep attributes, events and links to a span in a reasonable number.

---
