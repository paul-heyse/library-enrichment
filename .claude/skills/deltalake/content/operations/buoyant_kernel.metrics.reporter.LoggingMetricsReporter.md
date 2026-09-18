# `buoyant_kernel::metrics::reporter::LoggingMetricsReporter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.reporter.LoggingMetricsReporter.json).

<a id="op-93f287fb59465d891c24a891"></a>
## LoggingMetricsReporter

`struct` · `buoyant_kernel::metrics::reporter::LoggingMetricsReporter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LoggingMetricsReporter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L32).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`MetricsReporter`](../operations/buoyant_kernel.metrics.reporter.MetricsReporter.md#op-f96fef64a12770f2efa7dcf9) that logs each event as a tracing event at the configured level.

<a id="op-0a3f092f49bf8c50c4ca1f3b"></a>
## fmt

`function` · `buoyant_kernel::metrics::reporter::LoggingMetricsReporter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::LoggingMetricsReporter", "path": "LoggingMetricsReporter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a459195d02477f022e8926c5"></a>
## new

`function` · `buoyant_kernel::metrics::reporter::LoggingMetricsReporter::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(level: Level) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::LoggingMetricsReporter", "path": "LoggingMetricsReporter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [41, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new reporter that logs each [`MetricEvent`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-eeaec56c33e51aac3f98e89f) at the given tracing level.

<a id="op-16413a6795151978c5e18fac"></a>
## report

`function` · `buoyant_kernel::metrics::reporter::LoggingMetricsReporter::report` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn report(&self, event: MetricEvent)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L44).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::LoggingMetricsReporter", "path": "LoggingMetricsReporter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [55, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": null, "id": "buoyant_kernel::metrics::reporter::MetricsReporter", "path": "MetricsReporter"}, "trait_path": "buoyant_kernel::metrics::reporter::MetricsReporter"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:44`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f3e66438c9c9c5a8fc9480d"></a>
## level

`struct_field` · `buoyant_kernel::metrics::reporter::LoggingMetricsReporter::level` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
level: tracing::Level
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L33).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
