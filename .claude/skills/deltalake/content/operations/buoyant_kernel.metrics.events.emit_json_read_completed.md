# `buoyant_kernel::metrics::events::emit_json_read_completed`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.emit_json_read_completed.json).

<a id="op-5be5663fcc630dd90325d760"></a>
## emit_json_read_completed

`function` · `buoyant_kernel::metrics::events::emit_json_read_completed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn emit_json_read_completed(num_files: u64, bytes_read: u64)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1493).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1493`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emit a [`MetricEvent::JsonReadCompleted`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-913b38a3d1dfbf50a7239843) via a tracing span.

Call once per [`crate::JsonHandler::read_json_files`](../operations/buoyant_kernel.JsonHandler.md#op-fad9d2a21cfba1f126da6c44) invocation, at iterator exhaustion or
drop.
