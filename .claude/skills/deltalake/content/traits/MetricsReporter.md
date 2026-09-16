# MetricsReporter

`buoyant_kernel::metrics::reporter::MetricsReporter`

```rust
trait MetricsReporter: Send + Sync + std::fmt::Debug
```

Also reachable as `buoyant_kernel::metrics::MetricsReporter`, `delta_kernel::metrics::reporter::MetricsReporter`

Prose: [`api/buoyant_kernel.metrics.reporter.md`](../api/buoyant_kernel.metrics.reporter.md#metricsreporter) · records: [`model/buoyant_kernel.metrics.reporter.json`](../model/buoyant_kernel.metrics.reporter.json)

## Required

Every implementation must supply these.

```rust
fn report(&self, event: MetricEvent)
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::metrics::reporter::LoggingMetricsReporter`

## Documentation

Receives [`MetricEvent`]s as they occur during Delta operations and forwards them to a
monitoring system. Reporter implementations must be cheap to call on any thread.
