# `opentelemetry::propagation::text_map_propagator`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.propagation.text_map_propagator.json`](../model/opentelemetry.propagation.text_map_propagator.json)

## FieldIter

`struct` · `opentelemetry::propagation::text_map_propagator::FieldIter`

```rust
struct FieldIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(fields: &'a [String]) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over fields of a [`TextMapPropagator`]

---

## TextMapPropagator

`trait` · `opentelemetry::propagation::text_map_propagator::TextMapPropagator`

Also reachable as `opentelemetry::propagation::TextMapPropagator`

```rust
trait TextMapPropagator: Debug
```

**Implementors** (4)

- `opentelemetry::propagation::composite::TextMapCompositePropagator`
- `opentelemetry::trace::noop::NoopTextMapPropagator`
- `opentelemetry_sdk::propagation::baggage::BaggagePropagator`
- `opentelemetry_sdk::propagation::trace_context::TraceContextPropagator`

**Methods** (5)

```rust
fn extract(&self, extractor: &dyn Extractor) -> Context
fn extract_with_context(&self, cx: &Context, extractor: &dyn Extractor) -> Context
fn fields(&self) -> FieldIter<'_>
fn inject(&self, injector: &mut dyn Injector)
fn inject_context(&self, cx: &Context, injector: &mut dyn Injector)
```

Methods to inject and extract a value as text into injectors and extractors that travel
in-band across process boundaries.

---
