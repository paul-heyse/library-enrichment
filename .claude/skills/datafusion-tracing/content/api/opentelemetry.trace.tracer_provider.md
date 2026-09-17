# `opentelemetry::trace::tracer_provider`

Crate `opentelemetry` · 1 public items · structured records in [`model/opentelemetry.trace.tracer_provider.json`](../model/opentelemetry.trace.tracer_provider.json)

## TracerProvider

`trait` · `opentelemetry::trace::tracer_provider::TracerProvider`

Also reachable as `opentelemetry::trace::TracerProvider`

```rust
trait TracerProvider
```

**Implementors** (3)

- `opentelemetry::global::trace::GlobalTracerProvider`
- `opentelemetry::trace::noop::NoopTracerProvider`
- `opentelemetry_sdk::trace::provider::SdkTracerProvider`

**Methods** (2)

```rust
fn tracer(&self, name: impl Into<Cow<'static, str>>) -> Self::Tracer
fn tracer_with_scope(&self, scope: InstrumentationScope) -> Self::Tracer
```

Types that can create instances of [`Tracer`].

See the [`global`] module for examples of storing and retrieving tracer
provider instances.

[`global`]: crate::global

---
