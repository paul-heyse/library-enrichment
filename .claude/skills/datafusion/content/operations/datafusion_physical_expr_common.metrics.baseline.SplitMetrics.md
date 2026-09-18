# `datafusion_physical_expr_common::metrics::baseline::SplitMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.baseline.SplitMetrics.json).

<a id="op-a1b8a571c38085f5c08b69f5"></a>
## SplitMetrics

`struct` · `datafusion_physical_expr_common::metrics::baseline::SplitMetrics` · datafusion-physical-expr-common 55.1.0

```rust
struct SplitMetrics
```

Source: `src/metrics/baseline.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Metrics for tracking batch splitting activity

<a id="op-ebb06ce675fdfc6376208926"></a>
## batches_split

`struct_field` · `datafusion_physical_expr_common::metrics::baseline::SplitMetrics::batches_split` · datafusion-physical-expr-common 55.1.0

```rust
batches_split: super::Count
```

Source: `src/metrics/baseline.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Number of times an input [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) was split

<a id="op-b8228bbc3560e616213a8933"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::baseline::SplitMetrics::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> SplitMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::SplitMetrics", "path": "SplitMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 17], "end": [298, 22], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/baseline.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bc42e6a665c66f2bb7a3e1d"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::baseline::SplitMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::SplitMetrics", "path": "SplitMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 10], "end": [298, 15], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/baseline.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce79876b5d1e8f433c8bbac5"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::baseline::SplitMetrics::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::SplitMetrics", "path": "SplitMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [313, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new [`SplitMetrics`](../operations/datafusion_physical_expr_common.metrics.baseline.SplitMetrics.md#op-a1b8a571c38085f5c08b69f5)
