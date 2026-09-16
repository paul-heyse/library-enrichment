# `buoyant_kernel::metrics`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.metrics.json`](../model/buoyant_kernel.metrics.json)

## WithMetricsReporterLayer

`trait` · `buoyant_kernel::metrics::WithMetricsReporterLayer`

Also reachable as `delta_kernel::metrics::WithMetricsReporterLayer`

```rust
trait WithMetricsReporterLayer: Subscriber + for<'lookup> LookupSpan<'lookup>
```

**Methods** (1)

```rust
fn with_metrics_reporter_layer(self, reporter: Arc<dyn MetricsReporter>) -> Layered<ReportGeneratorLayer, Self> where Self: Sized
```

Extension trait that adds [`with_metrics_reporter_layer`] to any compatible tracing subscriber.

Only implemented for subscribers that also implement [`LookupSpan`], which is required by
[`ReportGeneratorLayer`] to store and retrieve per-span state.

[`with_metrics_reporter_layer`]: WithMetricsReporterLayer::with_metrics_reporter_layer

---
