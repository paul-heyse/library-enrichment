# `opentelemetry_sdk::metrics::instrument`

Crate `opentelemetry_sdk` · 4 public items · structured records in [`model/opentelemetry_sdk.metrics.instrument.json`](../model/opentelemetry_sdk.metrics.instrument.json)

## InstrumentKind

`enum` · `opentelemetry_sdk::metrics::instrument::InstrumentKind`

Also reachable as `opentelemetry_sdk::metrics::InstrumentKind`

```rust
enum InstrumentKind
```

**Variants**: `Counter`, `UpDownCounter`, `Histogram`, `ObservableCounter`, `ObservableUpDownCounter`, `Gauge`, `ObservableGauge`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

The identifier of a group of instruments that all perform the same function.

---

## Instrument

`struct` · `opentelemetry_sdk::metrics::instrument::Instrument`

Also reachable as `opentelemetry_sdk::metrics::Instrument`

```rust
struct Instrument
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn kind(&self) -> InstrumentKind
fn name(&self) -> &str
fn scope(&self) -> &InstrumentationScope
fn unit(&self) -> &str
```

Describes the properties of an instrument at creation, used for filtering in
views. This is utilized in the `with_view` methods on `MeterProviderBuilder`
to customize metric output.

Users can use a reference to `Instrument` to select which instrument(s) a
[Stream] should be applied to.

# Example

```rust
use opentelemetry_sdk::metrics::{Instrument, Stream};

let my_view_change_cardinality = |i: &Instrument| {
    if i.name() == "my_second_histogram" {
        // Note: If Stream is invalid, `build()` will return an error. By
        // calling `.ok()`, any such error is ignored and treated as if the
        // view does not match the instrument. If this is not the desired
        // behavior, consider handling the error explicitly.
        Stream::builder().with_cardinality_limit(2).build().ok()
    } else {
        None
    }
};
```

---

## Stream

`struct` · `opentelemetry_sdk::metrics::instrument::Stream`

Also reachable as `opentelemetry_sdk::metrics::Stream`

```rust
struct Stream
```

**Derives**: Debug, Default

**Methods** (1)

```rust
fn builder() -> StreamBuilder
```

Describes the stream of data an instrument produces. Used in `with_view`
methods on `MeterProviderBuilder` to customize the metric output.

---

## StreamBuilder

`struct` · `opentelemetry_sdk::metrics::instrument::StreamBuilder`

Also reachable as `opentelemetry_sdk::metrics::StreamBuilder`

```rust
struct StreamBuilder
```

**Derives**: Debug, Default

**Methods** (7)

```rust
fn build(self) -> Result<Stream, Box<dyn Error>>
fn with_aggregation(self, aggregation: Aggregation) -> Self
fn with_allowed_attribute_keys(self, attribute_keys: impl IntoIterator<Item = Key>) -> Self
fn with_cardinality_limit(self, limit: usize) -> Self
fn with_description(self, description: impl Into<Cow<'static, str>>) -> Self
fn with_name(self, name: impl Into<Cow<'static, str>>) -> Self
fn with_unit(self, unit: impl Into<Cow<'static, str>>) -> Self
```

A builder for creating Stream objects.

# Example

```
use opentelemetry_sdk::metrics::{Aggregation, Stream};
use opentelemetry::Key;

let stream = Stream::builder()
    .with_name("my_stream")
    .with_aggregation(Aggregation::Sum)
    .with_cardinality_limit(100)
    .build()
    .unwrap();
```

---
