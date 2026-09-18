# `buoyant_kernel::metrics::WithMetricsReporterLayer`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.WithMetricsReporterLayer.json).

<a id="op-d20c4dd9ad0134577fa7d22c"></a>
## WithMetricsReporterLayer

`trait` · `buoyant_kernel::metrics::WithMetricsReporterLayer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait WithMetricsReporterLayer: Subscriber + for<'lookup> LookupSpan<'lookup>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/mod.rs#L114).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/mod.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extension trait that adds [`with_metrics_reporter_layer`] to any compatible tracing subscriber.

Only implemented for subscribers that also implement [`LookupSpan`], which is required by
[`ReportGeneratorLayer`](../operations/buoyant_kernel.metrics.reporter.ReportGeneratorLayer.md#op-737450e9f241f4861a6e187f) to store and retrieve per-span state.

[`with_metrics_reporter_layer`]: WithMetricsReporterLayer::with_metrics_reporter_layer

Unresolved upstream links (retained, not inferred): ``LookupSpan``.

<a id="op-b3498de8ec1b330986469c1f"></a>
## with_metrics_reporter_layer

`function` · `buoyant_kernel::metrics::WithMetricsReporterLayer::with_metrics_reporter_layer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_metrics_reporter_layer(self, reporter: Arc<dyn MetricsReporter>) -> Layered<ReportGeneratorLayer, Self> where Self: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/mod.rs#L163).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/mod.rs:163`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Wrap this subscriber with a [`ReportGeneratorLayer`](../operations/buoyant_kernel.metrics.reporter.ReportGeneratorLayer.md#op-737450e9f241f4861a6e187f) that converts tracing spans into
[`MetricEvent`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-eeaec56c33e51aac3f98e89f)s and forwards them to `reporter`.

# Example

```
use std::sync::Arc;
use buoyant_kernel as delta_kernel;
use delta_kernel::metrics::{WithMetricsReporterLayer, LoggingMetricsReporter};
use tracing_subscriber::prelude::*;

tracing_subscriber::registry()
    .with_metrics_reporter_layer(
        Arc::new(LoggingMetricsReporter::new(tracing::Level::INFO))
    );
```

# Important: keep a real global default subscriber installed

Prefer installing this subscriber as the *global* default via
[`tracing::dispatcher::set_global_default`] (or
[`tracing_subscriber::util::SubscriberInitExt::init`]). The global path rebuilds
the callsite interest cache internally and guarantees that every thread has a
real subscriber to consult the first time it hits a callsite.

If you need thread-local isolation -- e.g. per-test
[`tracing_subscriber::util::SubscriberInitExt::set_default`] guards in a
multi-threaded test binary -- also install a bare global default subscriber
(e.g. `tracing_subscriber::registry().init()`) up front. Test code should
prefer `test_utils::install_thread_local_metrics_reporter`, which bundles
the global-subscriber install with the thread-local `set_default` so callers
cannot accidentally skip the first step. Several kernel metrics are emitted
from `Drop` impls (notably storage list/read completion in the default
engine), and those `Drop` sites can run on threads that have no subscriber
installed -- for example tokio worker threads owned by the `DefaultEngine`'s
background runtime. If a no-subscriber thread is the first to hit such a
callsite, tracing caches its `Interest` as `never` process-globally,
silently disabling that metric for the rest of the process. Keeping a real
global default active means every thread's "current dispatcher" is a real
subscriber, so the cached interest stays in the `always`/`sometimes` regime.

[`tracing::callsite::rebuild_interest_cache`] is *not* a reliable substitute on
its own: it only re-evaluates callsites that are already registered, but
callsites can be registered for the first time at any point on no-subscriber
threads, re-poisoning the cache after the rebuild.

[`tracing_subscriber::util::SubscriberInitExt::init`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/util/trait.SubscriberInitExt.html#method.init
[`tracing_subscriber::util::SubscriberInitExt::set_default`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/util/trait.SubscriberInitExt.html#method.set_default

Unresolved upstream links (retained, not inferred): ``tracing::callsite::rebuild_interest_cache``, ``tracing::dispatcher::set_global_default``.
