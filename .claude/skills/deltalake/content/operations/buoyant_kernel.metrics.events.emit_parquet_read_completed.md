# `buoyant_kernel::metrics::events::emit_parquet_read_completed`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.emit_parquet_read_completed.json).

<a id="op-5042c8650fad208d3cbe8c42"></a>
## emit_parquet_read_completed

`function` · `buoyant_kernel::metrics::events::emit_parquet_read_completed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn emit_parquet_read_completed(num_files: u64, bytes_read: u64)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1507).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1507`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emit a [`MetricEvent::ParquetReadCompleted`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-4d59be200d200d59297c6114) via a tracing span.

Call once per [`crate::ParquetHandler::read_parquet_files`](../operations/buoyant_kernel.ParquetHandler.md#op-9bb577f7cae9ed4c6da91a35) invocation, at iterator exhaustion
or drop.
