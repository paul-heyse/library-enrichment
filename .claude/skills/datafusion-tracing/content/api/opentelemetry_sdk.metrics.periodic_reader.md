# `opentelemetry_sdk::metrics::periodic_reader`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.metrics.periodic_reader.json`](../model/opentelemetry_sdk.metrics.periodic_reader.json)

## PeriodicReader

`struct` · `opentelemetry_sdk::metrics::periodic_reader::PeriodicReader`

Also reachable as `opentelemetry_sdk::metrics::PeriodicReader`

```rust
struct PeriodicReader<E: PushMetricExporter>
```

**Implements**: `opentelemetry_sdk::metrics::reader::MetricReader`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn builder(exporter: E) -> PeriodicReaderBuilder<E>
```

**via `opentelemetry_sdk::metrics::reader::MetricReader`**

```rust
fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult
fn force_flush(&self) -> OTelSdkResult
fn register_pipeline(&self, pipeline: Weak<Pipeline>)
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
fn temporality(&self, kind: InstrumentKind) -> Temporality
```

A `MetricReader` that periodically collects and exports metrics at a configurable interval.

By default, [`PeriodicReader`] collects and exports metrics every **60 seconds**.
The time taken for export is **not** included in the interval. Use [`PeriodicReaderBuilder`]
to customize the interval.

[`PeriodicReader`] spawns a background thread to handle metric collection and export.
This thread remains active until [`shutdown()`] is called.

## Collection Process
"Collection" refers to gathering aggregated metrics from the SDK's internal storage.
During this phase, callbacks from observable instruments are also triggered.

[`PeriodicReader`] does **not** enforce a timeout for collection. If an
observable callback takes too long, it may delay the next collection cycle.
If a callback never returns, it **will stall** all metric collection (and exports)
indefinitely.

## Exporter Compatibility
When used with the [`OTLP Exporter`](https://docs.rs/opentelemetry-otlp), the following
transport options are supported:

- **`grpc-tonic`**: Requires [`MeterProvider`] to be initialized within a `tokio` runtime.
- **`reqwest-blocking-client`**: Works with both a standard (`main`) function and `tokio::main`.

[`PeriodicReader`] does **not** enforce a timeout for exports either. Instead,
the configured exporter is responsible for enforcing timeouts. If an export operation
never returns, [`PeriodicReader`] will **stop exporting new metrics**, stalling
metric collection.

## Manual Export & Shutdown
Users can manually trigger an export via [`force_flush()`]. Calling [`shutdown()`]
exports any remaining metrics and should be done before application exit to ensure
all data is sent.

**Warning**: If using **tokio’s current-thread runtime**, calling [`shutdown()`]
from the main thread may cause a deadlock. To prevent this, call [`shutdown()`]
from a separate thread or use tokio's `spawn_blocking`.

[`PeriodicReader`]: crate::metrics::PeriodicReader
[`PeriodicReaderBuilder`]: crate::metrics::PeriodicReaderBuilder
[`MeterProvider`]: crate::metrics::SdkMeterProvider
[`shutdown()`]: crate::metrics::SdkMeterProvider::shutdown
[`force_flush()`]: crate::metrics::SdkMeterProvider::force_flush

# Example

```no_run
use opentelemetry_sdk::metrics::PeriodicReader;
# fn example<E>(get_exporter: impl Fn() -> E)
# where
#     E: opentelemetry_sdk::metrics::exporter::PushMetricExporter,
# {

let exporter = get_exporter(); // set up a push exporter

let reader = PeriodicReader::builder(exporter).build();
# drop(reader);
# }
```

---

## PeriodicReaderBuilder

`struct` · `opentelemetry_sdk::metrics::periodic_reader::PeriodicReaderBuilder`

Also reachable as `opentelemetry_sdk::metrics::PeriodicReaderBuilder`

```rust
struct PeriodicReaderBuilder<E>
```

**Derives**: Debug

**Methods** (2)

```rust
fn build(self) -> PeriodicReader<E>
fn with_interval(self, interval: Duration) -> Self
```

Configuration options for [PeriodicReader].

---
