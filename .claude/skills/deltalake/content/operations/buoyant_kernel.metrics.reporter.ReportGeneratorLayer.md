# `buoyant_kernel::metrics::reporter::ReportGeneratorLayer`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.reporter.ReportGeneratorLayer.json).

<a id="op-737450e9f241f4861a6e187f"></a>
## ReportGeneratorLayer

`struct` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ReportGeneratorLayer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L67).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:67`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`tracing_subscriber::Layer`] that converts kernel tracing spans into [`MetricEvent`](../operations/buoyant_kernel.metrics.events.MetricEvent.md#op-eeaec56c33e51aac3f98e89f)s and
forwards them to a registered [`MetricsReporter`](../operations/buoyant_kernel.metrics.reporter.MetricsReporter.md#op-f96fef64a12770f2efa7dcf9).

Typically added to a subscriber via
[`super::WithMetricsReporterLayer::with_metrics_reporter_layer`](../operations/buoyant_kernel.metrics.WithMetricsReporterLayer.md#op-b3498de8ec1b330986469c1f).

Unresolved upstream links (retained, not inferred): ``tracing_subscriber::Layer``.

<a id="op-23572b272f7e39ca2ff8a4b0"></a>
## fmt

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba45a003f7b6bdaaba988807"></a>
## new

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(reporter: Arc<dyn MetricsReporter>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [96, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:73`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new layer that forwards metric events to the given reporter.

<a id="op-752dbe786a9739ee9223e6f5"></a>
## on_close

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::on_close` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn on_close(&self, id: Id, ctx: Context<'_, S>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L175).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "tracing_core::subscriber::Subscriber", "path": "Subscriber"}}}, {"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'lookup"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'lookup"}], "constraints": []}}, "id": "tracing_subscriber::registry::LookupSpan", "path": "tracing_subscriber::registry::LookupSpan"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [98, 1], "end": [198, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "tracing_subscriber::layer::Layer", "path": "Layer"}, "trait_path": "tracing_subscriber::layer::Layer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:175`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-366b0979060631dc5938e10d"></a>
## on_enter

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::on_enter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn on_enter(&self, id: &Id, ctx: Context<'_, S>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "tracing_core::subscriber::Subscriber", "path": "Subscriber"}}}, {"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'lookup"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'lookup"}], "constraints": []}}, "id": "tracing_subscriber::registry::LookupSpan", "path": "tracing_subscriber::registry::LookupSpan"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [98, 1], "end": [198, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "tracing_subscriber::layer::Layer", "path": "Layer"}, "trait_path": "tracing_subscriber::layer::Layer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:163`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7019ca2f7e31e39290a0297e"></a>
## on_event

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::on_event` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L155).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "tracing_core::subscriber::Subscriber", "path": "Subscriber"}}}, {"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'lookup"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'lookup"}], "constraints": []}}, "id": "tracing_subscriber::registry::LookupSpan", "path": "tracing_subscriber::registry::LookupSpan"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [98, 1], "end": [198, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "tracing_subscriber::layer::Layer", "path": "Layer"}, "trait_path": "tracing_subscriber::layer::Layer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:155`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-658082087034487470d8b06b"></a>
## on_new_span

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::on_new_span` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "tracing_core::subscriber::Subscriber", "path": "Subscriber"}}}, {"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'lookup"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'lookup"}], "constraints": []}}, "id": "tracing_subscriber::registry::LookupSpan", "path": "tracing_subscriber::registry::LookupSpan"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [98, 1], "end": [198, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "tracing_subscriber::layer::Layer", "path": "Layer"}, "trait_path": "tracing_subscriber::layer::Layer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0f45a13eaf85c595b0c114c"></a>
## on_record

`function` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::on_record` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L159).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::reporter::ReportGeneratorLayer", "path": "ReportGeneratorLayer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "tracing_core::subscriber::Subscriber", "path": "Subscriber"}}}, {"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'lookup"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'lookup"}], "constraints": []}}, "id": "tracing_subscriber::registry::LookupSpan", "path": "tracing_subscriber::registry::LookupSpan"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [98, 1], "end": [198, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "tracing_subscriber::layer::Layer", "path": "Layer"}, "trait_path": "tracing_subscriber::layer::Layer"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:159`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8d85f94282d84ada8bb3c5b"></a>
## reporter

`struct_field` · `buoyant_kernel::metrics::reporter::ReportGeneratorLayer::reporter` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
reporter: std::sync::Arc<dyn MetricsReporter>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/reporter.rs#L68).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/reporter.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
