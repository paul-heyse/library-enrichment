# `opentelemetry::propagation::composite`

Crate `opentelemetry` · 1 public items · structured records in [`model/opentelemetry.propagation.composite.json`](../model/opentelemetry.propagation.composite.json)

## TextMapCompositePropagator

`struct` · `opentelemetry::propagation::composite::TextMapCompositePropagator`

Also reachable as `opentelemetry::propagation::TextMapCompositePropagator`

```rust
struct TextMapCompositePropagator
```

**Implements**: `opentelemetry::propagation::text_map_propagator::TextMapPropagator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(propagators: Vec<Box<dyn TextMapPropagator + Send + Sync>>) -> Self
```

**via `opentelemetry::propagation::text_map_propagator::TextMapPropagator`**

```rust
fn extract_with_context(&self, cx: &Context, extractor: &dyn Extractor) -> Context
fn fields(&self) -> FieldIter<'_>
fn inject_context(&self, context: &Context, injector: &mut dyn Injector)
```

Composite propagator for [`TextMapPropagator`]s.

A propagator that chains multiple [`TextMapPropagator`] propagators together,
injecting or extracting by their respective HTTP header names.

Injection and extraction from this propagator will preserve the order of the
injectors and extractors passed in during initialization.

# Examples

```
use opentelemetry::{
    baggage::BaggageExt,
    propagation::{TextMapPropagator, TextMapCompositePropagator},

    trace::{TraceContextExt, Tracer, TracerProvider},
    Context, KeyValue,
};
use opentelemetry_sdk::propagation::{
    BaggagePropagator, TraceContextPropagator,
};
use opentelemetry_sdk::trace as sdktrace;
use std::collections::HashMap;

// First create 1 or more propagators
let baggage_propagator = BaggagePropagator::new();
let trace_context_propagator = TraceContextPropagator::new();

// Then create a composite propagator
let composite_propagator = TextMapCompositePropagator::new(vec![
    Box::new(baggage_propagator),
    Box::new(trace_context_propagator),
]);

// Then for a given implementation of `Injector`
let mut injector = HashMap::new();

// And a given span
let example_span = sdktrace::SdkTracerProvider::default()
    .tracer("example-component")
    .start("span-name");

// with the current context, call inject to add the headers
composite_propagator.inject_context(
    &Context::current_with_span(example_span)
        .with_baggage(vec![KeyValue::new("test", "example")]),
    &mut injector,
);

// The injector now has both `baggage` and `traceparent` headers
assert!(injector.get("baggage").is_some());
assert!(injector.get("traceparent").is_some());
```

---
