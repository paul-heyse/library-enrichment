# `tracing_opentelemetry::metrics`

Crate `tracing-opentelemetry` · 1 public items · structured records in [`model/tracing_opentelemetry.metrics.json`](../model/tracing_opentelemetry.metrics.json)

## MetricsLayer

`struct` · `tracing_opentelemetry::metrics::MetricsLayer`

Also reachable as `tracing_opentelemetry::MetricsLayer`

```rust
struct MetricsLayer<S, M>
```

**Implements**: `tracing_subscriber::layer::Layer`

**Methods** (1)

```rust
fn new(meter_provider: M) -> MetricsLayer<S, M> where M: MeterProvider
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool
fn on_close(&self, id: tracing_core::span::Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &tracing_core::span::Id, ctx: Context<'_, S>)
fn on_event(&self, event: &tracing_core::Event<'_>, ctx: Context<'_, S>)
fn on_exit(&self, id: &tracing_core::span::Id, ctx: Context<'_, S>)
fn on_follows_from(&self, span: &tracing_core::span::Id, follows: &tracing_core::span::Id, ctx: Context<'_, S>)
fn on_id_change(&self, old: &tracing_core::span::Id, new: &tracing_core::span::Id, ctx: Context<'_, S>)
fn on_layer(&mut self, subscriber: &mut S)
fn on_new_span(&self, attrs: &tracing_core::span::Attributes<'_>, id: &tracing_core::span::Id, ctx: Context<'_, S>)
fn on_record(&self, span: &tracing_core::span::Id, values: &tracing_core::span::Record<'_>, ctx: Context<'_, S>)
fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest
```

A layer that publishes metrics via the OpenTelemetry SDK.

# Usage

No configuration is needed for this Layer, as it's only responsible for
pushing data out to the `opentelemetry` family of crates. For example, when
using `opentelemetry-otlp`, that crate will provide its own set of
configuration options for setting up the duration metrics will be collected
before exporting to the OpenTelemetry Collector, aggregation of data points,
etc.

```no_run
use tracing_opentelemetry::MetricsLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
# use opentelemetry_sdk::metrics::SdkMeterProvider;

// Constructing a MeterProvider is out-of-scope for the docs here, but there
// are examples in the opentelemetry repository. See:
// https://github.com/open-telemetry/opentelemetry-rust/blob/dfeac078ff7853e7dc814778524b93470dfa5c9c/examples/metrics-basic/src/main.rs#L7
# let meter_provider: SdkMeterProvider = unimplemented!();

let opentelemetry_metrics =  MetricsLayer::new(meter_provider);
let subscriber = Registry::default().with(opentelemetry_metrics);
tracing::subscriber::set_global_default(subscriber).unwrap();
```

To publish a new metric, add a key-value pair to your `tracing::Event` that
contains following prefixes:
- `monotonic_counter.` (non-negative numbers): Used when the counter should
  only ever increase
- `counter.`: Used when the counter can go up or down
- `histogram.`: Used to report arbitrary values that are likely to be statistically meaningful
- `gauge.`: Used to report instantaneous values that can go up or down

Examples:
```
# use tracing::info;
info!(monotonic_counter.foo = 1);
info!(monotonic_counter.bar = 1.1);

info!(counter.baz = 1);
info!(counter.baz = -1);
info!(counter.xyz = 1.1);

info!(histogram.qux = 1);
info!(histogram.abc = -1);
info!(histogram.def = 1.1);

info!(gauge.foo = 1);
info!(gauge.bar = 1.1);
```

# Mixing data types

## Floating-point numbers

Do not mix floating point and non-floating point numbers for the same
metric. If a floating point number will be used for a given metric, be sure
to cast any other usages of that metric to a floating point number.

Do this:
```
# use tracing::info;
info!(monotonic_counter.foo = 1_f64);
info!(monotonic_counter.foo = 1.1);
```

This is because all data published for a given metric name must be the same
numeric type.

## Integers

Positive and negative integers can be mixed freely. The instrumentation
provided by `tracing` assumes that all integers are `i64` unless explicitly
cast to something else. In the case that an integer *is* cast to `u64`, this
subscriber will handle the conversion internally.

For example:
```
# use tracing::info;
// The subscriber receives an i64
info!(counter.baz = 1);

// The subscriber receives an i64
info!(counter.baz = -1);

// The subscriber receives a u64, but casts it to i64 internally
info!(counter.baz = 1_u64);

// The subscriber receives a u64, but cannot cast it to i64 because of
// overflow. An error is printed to stderr, and the metric is dropped.
info!(counter.baz = (i64::MAX as u64) + 1)
```

# Attributes

When `MetricsLayer` outputs metrics, it converts key-value pairs into [Attributes] and associates them with metrics.

[Attributes]: https://opentelemetry.io/docs/specs/otel/common/#attribute

For example:
```
# use tracing::info;
// adds attributes bar="baz" and qux=2 to the `foo` counter.
info!(monotonic_counter.foo = 1, bar = "baz", qux = 2);
```

# Implementation Details

`MetricsLayer` holds a set of maps, with each map corresponding to a
type of metric supported by OpenTelemetry. These maps are populated lazily.
The first time that a metric is emitted by the instrumentation, a `Metric`
instance will be created and added to the corresponding map. This means that
any time a metric is emitted by the instrumentation, one map lookup has to
be performed.

In the future, this can be improved by associating each `Metric` instance to
its callsite, eliminating the need for any maps.

---
