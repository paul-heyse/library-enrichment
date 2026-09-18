# `buoyant_kernel::metrics::reporter::MetricsReporter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.reporter.MetricsReporter.json).

<a id="op-f96fef64a12770f2efa7dcf9"></a>
## MetricsReporter

`trait` · `buoyant_kernel::metrics::reporter::MetricsReporter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait MetricsReporter: Send + Sync + std::fmt::Debug
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L25).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Receives [`MetricEvent`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-eeaec56c33e51aac3f98e89f)s as they occur during Delta operations and forwards them to a
monitoring system. Reporter implementations must be cheap to call on any thread.

<a id="op-a7aa83c0ddcf7ab5210306ad"></a>
## report

`function` · `buoyant_kernel::metrics::reporter::MetricsReporter::report` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn report(&self, event: MetricEvent)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L27).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Report a metric event.
