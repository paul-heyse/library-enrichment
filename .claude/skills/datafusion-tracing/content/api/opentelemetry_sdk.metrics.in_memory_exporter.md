# `opentelemetry_sdk::metrics::in_memory_exporter`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.metrics.in_memory_exporter.json`](../model/opentelemetry_sdk.metrics.in_memory_exporter.json)

## InMemoryMetricExporter

`struct` · `opentelemetry_sdk::metrics::in_memory_exporter::InMemoryMetricExporter`

Also reachable as `opentelemetry_sdk::metrics::InMemoryMetricExporter`

```rust
struct InMemoryMetricExporter
```

**Implements**: `opentelemetry_sdk::metrics::exporter::PushMetricExporter`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn get_finished_metrics(&self) -> Result<Vec<ResourceMetrics>, InMemoryExporterError>
fn reset(&self)
```

**via `opentelemetry_sdk::metrics::exporter::PushMetricExporter`**

```rust
async fn export(&self, metrics: &ResourceMetrics) -> OTelSdkResult
fn force_flush(&self) -> OTelSdkResult
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
fn temporality(&self) -> Temporality
```

 An in-memory metrics exporter that stores metrics data in memory.

 This exporter is useful for testing and debugging purposes. It stores
 metric data in a `VecDeque<ResourceMetrics>`. Metrics can be retrieved
 using the `get_finished_metrics` method.

 # Panics

 This exporter may panic
 - if there's an issue with locking the `metrics` Mutex, such as if the Mutex is poisoned.
 - the data point recorded is not one of [i64, u64, f64]. This shouldn't happen if used with OpenTelemetry API.

 # Example

 ```
# use opentelemetry_sdk::metrics;
# use opentelemetry::{KeyValue};
# use opentelemetry::metrics::MeterProvider;
# use opentelemetry_sdk::metrics::InMemoryMetricExporter;
# use opentelemetry_sdk::metrics::PeriodicReader;

# #[tokio::main]
# async fn main() {
 // Create an InMemoryMetricExporter
  let exporter = InMemoryMetricExporter::default();

  // Create a MeterProvider and register the exporter
  let meter_provider = metrics::SdkMeterProvider::builder()
      .with_reader(PeriodicReader::builder(exporter.clone()).build())
      .build();

  // Create and record metrics using the MeterProvider
  let meter = meter_provider.meter("example");
  let counter = meter.u64_counter("my_counter").build();
  counter.add(1, &[KeyValue::new("key", "value")]);

  meter_provider.force_flush().unwrap();

  // Retrieve the finished metrics from the exporter
  let finished_metrics = exporter.get_finished_metrics().unwrap();

  // Print the finished metrics
 for resource_metrics in finished_metrics {
      println!("{:?}", resource_metrics);
  }
# }
 ```

---

## InMemoryMetricExporterBuilder

`struct` · `opentelemetry_sdk::metrics::in_memory_exporter::InMemoryMetricExporterBuilder`

Also reachable as `opentelemetry_sdk::metrics::InMemoryMetricExporterBuilder`

```rust
struct InMemoryMetricExporterBuilder
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn build(self) -> InMemoryMetricExporter
fn new() -> Self
fn with_temporality(self, temporality: Temporality) -> Self
```

Builder for [`InMemoryMetricExporter`].
# Example

```
# use opentelemetry_sdk::metrics::{InMemoryMetricExporter, InMemoryMetricExporterBuilder};

let exporter = InMemoryMetricExporterBuilder::new().build();
```

---
