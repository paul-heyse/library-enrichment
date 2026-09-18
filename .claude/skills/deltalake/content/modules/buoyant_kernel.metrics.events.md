# `buoyant_kernel::metrics::events`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.json).

<a id="op-8dc676cea6545843146d5660"></a>
## events

`module` · `buoyant_kernel::metrics::events` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod events
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metric event types emitted during Delta Kernel operations.

Each [`MetricEvent`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-eeaec56c33e51aac3f98e89f) variant wraps a per-event struct that owns its fields, `Display` impl,
span name, and the small set of methods the `tracing` layer in [`crate::metrics::reporter`](../modules/buoyant_kernel.metrics.reporter.md#op-006b7e8f236f8106f1d57b4c)
calls to construct and finalize the event. Per-event code is colocated in a single block
per type below.

Enums carried in event payloads derive the full strum set (`EnumString`, `Display`,
`AsRefStr`, `IntoStaticStr`) with stable serialized names. `IntoStaticStr` in particular
lets connectors convert a value to its `&'static str` metric-label string via `.into()`
instead of maintaining their own variant-to-string `match`.

Event construction is infallible: a malformed span field warns and falls back to a
default rather than failing the operation being observed.
