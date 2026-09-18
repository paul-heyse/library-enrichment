# `datafusion_physical_expr_common::metrics::baseline::SpillMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.baseline.SpillMetrics.json).

<a id="op-c00b69e761302957fb74e40d"></a>
## SpillMetrics

`struct` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics` · datafusion-physical-expr-common 55.1.0

```rust
struct SpillMetrics
```

Source: `src/metrics/baseline.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Helper for creating and tracking spill-related metrics for
each operator

<a id="op-954564a9dca845d24269663e"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> SpillMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::SpillMetrics", "path": "SpillMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 17], "end": [274, 22], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/baseline.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-265c9d87a6ef651f369d05d9"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::SpillMetrics", "path": "SpillMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 10], "end": [274, 15], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/baseline.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad87f6c4e586e53f1cd7d1f8"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::SpillMetrics", "path": "SpillMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [295, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new SpillMetrics structure

<a id="op-8073c95b59b6698ac88d8fbf"></a>
## spill_file_count

`struct_field` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics::spill_file_count` · datafusion-physical-expr-common 55.1.0

```rust
spill_file_count: super::Count
```

Source: `src/metrics/baseline.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

count of spills during the execution of the operator

<a id="op-a18e8fd2c41d9b5ace2ce465"></a>
## spilled_bytes

`struct_field` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics::spilled_bytes` · datafusion-physical-expr-common 55.1.0

```rust
spilled_bytes: super::Count
```

Source: `src/metrics/baseline.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

total bytes actually written to disk during the execution of the operator

<a id="op-350cd264485fb5a48f706102"></a>
## spilled_rows

`struct_field` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics::spilled_rows` · datafusion-physical-expr-common 55.1.0

```rust
spilled_rows: super::Count
```

Source: `src/metrics/baseline.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

total spilled rows during the execution of the operator
