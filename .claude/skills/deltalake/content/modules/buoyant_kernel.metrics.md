# `buoyant_kernel::metrics`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.json).

<a id="op-0582065e7ca5d5c675116aae"></a>
## metrics

`module` · `buoyant_kernel::metrics` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod metrics
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metrics collection for Delta Kernel operations.

This module provides metrics tracking for various Delta operations including
snapshot creation, scans, and transactions. Metrics are collected during operations
and reported as events via the `MetricsReporter` trait.

Each operation (Snapshot, Transaction, Scan) is assigned a unique operation ID ([`MetricId`](../operations/buoyant_kernel.metrics.events.MetricId.md#op-7be8d0b429e95b1f453ce324))
when it starts, and all subsequent events for that operation reference this ID.
This allows reporters to correlate events and track operation lifecycles.

# Example: Implementing a Custom MetricsReporter

```
use buoyant_kernel as delta_kernel;
use delta_kernel::metrics::{MetricsReporter, MetricEvent};

#[derive(Debug)]
struct LoggingReporter;

impl MetricsReporter for LoggingReporter {
    fn report(&self, event: MetricEvent) {
        match event {
            MetricEvent::LogSegmentLoadSuccess(e) => {
                println!("Log segment loaded in {:?}: {} commits", e.duration, e.num_commit_files);
            }
            MetricEvent::SnapshotBuildSuccess(e) => {
                println!("Snapshot completed: v{} in {:?}", e.version, e.duration);
            }
            MetricEvent::SnapshotBuildFailure(e) => {
                println!("Snapshot failed: {}", e.operation_id);
            }
            _ => {}
        }
    }
}
```

# Example: Implementing a Composite Reporter

If you need to send metrics to multiple destinations, you can create a composite reporter:

```
use std::sync::Arc;
use buoyant_kernel as delta_kernel;
use delta_kernel::metrics::{MetricsReporter, MetricEvent};

#[derive(Debug)]
struct CompositeReporter {
    reporters: Vec<Arc<dyn MetricsReporter>>,
}

impl MetricsReporter for CompositeReporter {
    fn report(&self, event: MetricEvent) {
        for reporter in &self.reporters {
            reporter.report(event.clone());
        }
    }
}
```

# Handler Metrics

Storage, JSON, and Parquet handler operations emit one event per read or copy
operation, fired when the returned iterator is exhausted or dropped:
`StorageListCompleted` / `StorageReadCompleted` / `StorageCopyCompleted` for storage,
`JsonReadCompleted` for `JsonHandler::read_json_files`, and `ParquetReadCompleted`
for `ParquetHandler::read_parquet_files`. `DefaultEngine` wraps its three handlers
in [`MeteredStorageHandler`](../operations/buoyant_kernel.metrics.metered_storage.MeteredStorageHandler.md#op-649a125cd3d23861605596b7), [`MeteredJsonHandler`](../operations/buoyant_kernel.metrics.metered_json.MeteredJsonHandler.md#op-4917f5cfca39dfff79dc6c93), and [`MeteredParquetHandler`](../operations/buoyant_kernel.metrics.metered_parquet.MeteredParquetHandler.md#op-0ff1413b5221a5a8440c7dda)
at construction; any other engine gets the same coverage by wrapping its [`Engine`]
in [`MeteredDeltaEngine`](../operations/buoyant_kernel.metrics.metered_engine.MeteredDeltaEngine.md#op-6a6a81013b8e6f35873abf9f).

These metrics are standalone and track aggregate handler performance without
correlating to specific Snapshot/Transaction operations.

[`Engine`]: crate::Engine
