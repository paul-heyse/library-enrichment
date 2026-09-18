# `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.ExecutionPlanMetricsSet.json).

<a id="op-ad96f6574f61ed588f114741"></a>
## ExecutionPlanMetricsSet

`struct` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet` · datafusion-physical-expr-common 55.1.0

```rust
struct ExecutionPlanMetricsSet
```

Source: `src/metrics/mod.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A set of [`Metric`](../operations/datafusion_physical_expr_common.metrics.Metric.md#op-f3d3e8659b0526690e35cebe)s for an individual operator.

This structure is intended as a convenience for execution plan
implementations so they can generate different streams for multiple
partitions but easily report them together.

Each `clone()` of this structure will add metrics to the same
underlying metrics set

<a id="op-448ec6eda757cf1fdaf529d5"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 26], "end": [495, 31], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/mod.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-909108cc491c8ec5704024bb"></a>
## clone_inner

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::clone_inner` · datafusion-physical-expr-common 55.1.0

```rust
fn clone_inner(&self) -> MetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [518, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return a clone of the inner [`MetricsSet`](../operations/datafusion_physical_expr_common.metrics.MetricsSet.md#op-c077c70588e76ca4a87a0b1b)

<a id="op-cb5c2964f27da4c29f5cbe4f"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> ExecutionPlanMetricsSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 10], "end": [495, 17], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/mod.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a293255cb75d085abf6d4496"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 19], "end": [495, 24], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/mod.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1ac377bd5d436a1f0bfacd3"></a>
## from

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::from` · datafusion-physical-expr-common 55.1.0

```rust
fn from(metrics: MetricsSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [526, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::MetricsSet", "path": "MetricsSet"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metrics/mod.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1a7226f1f730d5091483bf5"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [518, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new empty shared metrics set

<a id="op-5cc0d1d0b4cea8a35c5d5ba8"></a>
## register

`function` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet::register` · datafusion-physical-expr-common 55.1.0

```rust
fn register(&self, metric: Arc<Metric>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet", "path": "ExecutionPlanMetricsSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [518, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add the specified metric to the underlying metric set
