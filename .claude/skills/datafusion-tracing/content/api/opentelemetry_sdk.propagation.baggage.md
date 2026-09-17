# `opentelemetry_sdk::propagation::baggage`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.propagation.baggage.json`](../model/opentelemetry_sdk.propagation.baggage.json)

## BaggagePropagator

`struct` · `opentelemetry_sdk::propagation::baggage::BaggagePropagator`

Also reachable as `opentelemetry_sdk::propagation::BaggagePropagator`

```rust
struct BaggagePropagator
```

**Implements**: `opentelemetry::propagation::text_map_propagator::TextMapPropagator`

**Derives**: Debug, Default

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

Propagates name-value pairs in [W3C Baggage] format.

Baggage is used to annotate telemetry, adding context and
information to metrics, traces, and logs. It is an abstract data type
represented by a set of name-value pairs describing user-defined properties.
Each name in a [`Baggage`] is associated with exactly one value.
`Baggage`s are serialized according to the editor's draft of
the [W3C Baggage] specification.

# Examples

```
use opentelemetry::{baggage::{Baggage, BaggageExt}, propagation::TextMapPropagator};
use opentelemetry_sdk::propagation::BaggagePropagator;
use std::collections::HashMap;

// Example baggage value passed in externally via http headers
let mut headers = HashMap::new();
headers.insert("baggage".to_string(), "user_id=1".to_string());

let propagator = BaggagePropagator::new();
// can extract from any type that impls `Extractor`, usually an HTTP header map
let cx = propagator.extract(&headers);

// Iterate over extracted name-value pairs
for (name, value) in cx.baggage() {
    // ...
}

// Add new baggage
let mut baggage = Baggage::new();
let _ = baggage.insert("server_id", "42");

let cx_with_additions = cx.with_baggage(baggage);

// Inject baggage into http request
propagator.inject_context(&cx_with_additions, &mut headers);

let header_value = headers.get("baggage").expect("header is injected");
assert!(!header_value.contains("user_id=1"), "still contains previous name-value");
assert!(header_value.contains("server_id=42"), "does not contain new name-value pair");
```

[W3C Baggage]: https://w3c.github.io/baggage
[`Baggage`]: opentelemetry::baggage::Baggage

---
