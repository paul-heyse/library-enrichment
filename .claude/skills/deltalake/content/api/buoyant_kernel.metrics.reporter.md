# `buoyant_kernel::metrics::reporter`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.metrics.reporter.json`](../model/buoyant_kernel.metrics.reporter.json)

## LoggingMetricsReporter

`struct` · `buoyant_kernel::metrics::reporter::LoggingMetricsReporter`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.reporter.LoggingMetricsReporter.md)

Also reachable as `buoyant_kernel::metrics::LoggingMetricsReporter`, `delta_kernel::metrics::reporter::LoggingMetricsReporter`

```rust
struct LoggingMetricsReporter
```

**Implements**: `buoyant_kernel::metrics::reporter::MetricsReporter`

**Derives**: Debug

**Methods** (1)

```rust
fn new(level: Level) -> Self
```

**via `buoyant_kernel::metrics::reporter::MetricsReporter`**

```rust
fn report(&self, event: MetricEvent)
```

A [`MetricsReporter`] that logs each event as a tracing event at the configured level.

---

## ReportGeneratorLayer

`struct` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.reporter.ReportGeneratorLayer.md)

Also reachable as `buoyant_kernel::metrics::ReportGeneratorLayer`, `delta_kernel::metrics::reporter::ReportGeneratorLayer`

```rust
struct ReportGeneratorLayer
```

**Implements**: `tracing_subscriber::layer::Layer`

**Derives**: Debug

**Methods** (1)

```rust
fn new(reporter: Arc<dyn MetricsReporter>) -> Self
```

**via `tracing_subscriber::layer::Layer`**

```rust
fn on_close(&self, id: Id, ctx: Context<'_, S>)
fn on_enter(&self, id: &Id, ctx: Context<'_, S>)
fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>)
fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

A [`tracing_subscriber::Layer`] that converts kernel tracing spans into [`MetricEvent`]s and
forwards them to a registered [`MetricsReporter`].

Typically added to a subscriber via
[`super::WithMetricsReporterLayer::with_metrics_reporter_layer`].

---

## MetricsReporter

`trait` · `buoyant_kernel::metrics::reporter::MetricsReporter`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.metrics.reporter.MetricsReporter.md)

Also reachable as `buoyant_kernel::metrics::MetricsReporter`, `delta_kernel::metrics::reporter::MetricsReporter`

```rust
trait MetricsReporter: Send + Sync + std::fmt::Debug
```

**Implementors** (1)

- `buoyant_kernel::metrics::reporter::LoggingMetricsReporter`

**Methods** (1)

```rust
fn report(&self, event: MetricEvent)
```

Receives [`MetricEvent`]s as they occur during Delta operations and forwards them to a
monitoring system. Reporter implementations must be cheap to call on any thread.

---
