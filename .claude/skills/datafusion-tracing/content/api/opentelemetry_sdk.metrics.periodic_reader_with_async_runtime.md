# `opentelemetry_sdk::metrics::periodic_reader_with_async_runtime`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.metrics.periodic_reader_with_async_runtime.json`](../model/opentelemetry_sdk.metrics.periodic_reader_with_async_runtime.json)

## PeriodicReader

`struct` · `opentelemetry_sdk::metrics::periodic_reader_with_async_runtime::PeriodicReader`

```rust
struct PeriodicReader<E: PushMetricExporter>
```

**Implements**: `opentelemetry_sdk::metrics::reader::MetricReader`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn builder<RT>(exporter: E, runtime: RT) -> PeriodicReaderBuilder<E, RT> where RT: Runtime
```

**via `opentelemetry_sdk::metrics::reader::MetricReader`**

```rust
fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult
fn force_flush(&self) -> OTelSdkResult
fn register_pipeline(&self, pipeline: Weak<Pipeline>)
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
fn temporality(&self, kind: InstrumentKind) -> super::Temporality
```

A [MetricReader] that continuously collects and exports metric data at a set
interval.

By default it will collect and export data every 60 seconds, and will cancel
export attempts that exceed 30 seconds. The export time is not counted
towards the interval between attempts.

The [collect] method of the returned continues to gather and
return metric data to the user. It will not automatically send that data to
the exporter outside of the predefined interval.

The [runtime] can be selected based on feature flags set for this crate.

The exporter can be any exporter that implements [PushMetricExporter] such
as [opentelemetry-otlp].

[collect]: MetricReader::collect
[runtime]: crate::runtime
[opentelemetry-otlp]: https://docs.rs/opentelemetry-otlp/latest/opentelemetry_otlp/

# Example

```no_run
use opentelemetry_sdk::metrics::periodic_reader_with_async_runtime::PeriodicReader;
# fn example<E, R>(get_exporter: impl Fn() -> E, get_runtime: impl Fn() -> R)
# where
#     E: opentelemetry_sdk::metrics::exporter::PushMetricExporter,
#     R: opentelemetry_sdk::runtime::Runtime,
# {

let exporter = get_exporter(); // set up a push exporter like OTLP
let runtime = get_runtime(); // select runtime: e.g. opentelemetry_sdk:runtime::Tokio

let reader = PeriodicReader::builder(exporter, runtime).build();
# drop(reader);
# }
```

---

## PeriodicReaderBuilder

`struct` · `opentelemetry_sdk::metrics::periodic_reader_with_async_runtime::PeriodicReaderBuilder`

```rust
struct PeriodicReaderBuilder<E, RT>
```

**Derives**: Debug

**Methods** (3)

```rust
fn build(self) -> PeriodicReader<E>
fn with_interval(self, interval: Duration) -> Self
fn with_timeout(self, timeout: Duration) -> Self
```

Configuration options for [PeriodicReader].

A periodic reader is a [MetricReader] that collects and exports metric data
to the exporter at a defined interval.

By default, the returned [MetricReader] will collect and export data every
60 seconds, and will cancel export attempts that exceed 30 seconds. The
export time is not counted towards the interval between attempts.

The [collect] method of the returned [MetricReader] continues to gather and
return metric data to the user. It will not automatically send that data to
the exporter outside of the predefined interval.

[collect]: MetricReader::collect

---
