# `datafusion_physical_expr_common::metrics::Metric`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.Metric.json).

<a id="op-f3d3e8659b0526690e35cebe"></a>
## Metric

`struct` · `datafusion_physical_expr_common::metrics::Metric` · datafusion-physical-expr-common 55.1.0

```rust
struct Metric
```

Source: `src/metrics/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Something that tracks a value of interest (metric) during execution.

Typically [`Metric`](../operations/datafusion_physical_expr_common.metrics.Metric.md#op-f3d3e8659b0526690e35cebe)s are not created directly, but instead
are created using [`MetricBuilder`](../operations/datafusion_physical_expr_common.metrics.builder.MetricBuilder.md#op-69e5b13fd1a8e33cbdc0d1a8) or methods on
[`ExecutionPlanMetricsSet`](../operations/datafusion_physical_expr_common.metrics.ExecutionPlanMetricsSet.md#op-ad96f6574f61ed588f114741).

```
use datafusion_physical_expr_common::metrics::*;

let metrics = ExecutionPlanMetricsSet::new();
assert!(metrics.clone_inner().output_rows().is_none());

// Create a counter to increment using the MetricBuilder
let partition = 1;
let output_rows = MetricBuilder::new(&metrics).output_rows(partition);

// Counter can be incremented
output_rows.add(13);

// The value can be retrieved directly:
assert_eq!(output_rows.value(), 13);

// As well as from the metrics set
assert_eq!(metrics.clone_inner().output_rows(), Some(13));
```

<a id="op-333d3beb48c62b00084962ef"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::Metric::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6d42986ad35ca41bc213c6a"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::Metric::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [130, 2], "filename": "src/metrics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a13b367456e96bd5368857d0"></a>
## labels

`function` · `datafusion_physical_expr_common::metrics::Metric::labels` · datafusion-physical-expr-common 55.1.0

```rust
fn labels(&self) -> &[Label]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

What labels are present for this metric?

<a id="op-a4daf0c6d57b435002f6487f"></a>
## metric_category

`function` · `datafusion_physical_expr_common::metrics::Metric::metric_category` · datafusion-physical-expr-common 55.1.0

```rust
fn metric_category(&self) -> Option<MetricCategory>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the metric category, if one was declared.

`None` means the metric is always included (except in `none` mode).

<a id="op-26c7811b8d374e2522a31a5b"></a>
## metric_type

`function` · `datafusion_physical_expr_common::metrics::Metric::metric_type` · datafusion-physical-expr-common 55.1.0

```rust
fn metric_type(&self) -> MetricType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return the metric type (verbosity level) associated with this metric

<a id="op-b68894fa5d88b6943df8288b"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::Metric::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(value: MetricValue, partition: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new [`Metric`](../operations/datafusion_physical_expr_common.metrics.Metric.md#op-f3d3e8659b0526690e35cebe). Consider using [`MetricBuilder`](../operations/datafusion_physical_expr_common.metrics.builder.MetricBuilder.md#op-69e5b13fd1a8e33cbdc0d1a8)
rather than this function directly.

<a id="op-51aaede843993dcd95f2ec12"></a>
## new_with_labels

`function` · `datafusion_physical_expr_common::metrics::Metric::new_with_labels` · datafusion-physical-expr-common 55.1.0

```rust
fn new_with_labels(value: MetricValue, partition: Option<usize>, labels: Vec<Label>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new [`Metric`](../operations/datafusion_physical_expr_common.metrics.Metric.md#op-f3d3e8659b0526690e35cebe). Consider using [`MetricBuilder`](../operations/datafusion_physical_expr_common.metrics.builder.MetricBuilder.md#op-69e5b13fd1a8e33cbdc0d1a8)
rather than this function directly.

<a id="op-235edf50302ebdb6e6419e70"></a>
## partition

`function` · `datafusion_physical_expr_common::metrics::Metric::partition` · datafusion-physical-expr-common 55.1.0

```rust
fn partition(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return a reference to the partition

<a id="op-a3555ef8373eb95b17081d94"></a>
## value

`function` · `datafusion_physical_expr_common::metrics::Metric::value` · datafusion-physical-expr-common 55.1.0

```rust
fn value(&self) -> &MetricValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return a reference to the value of this metric

<a id="op-dd8a31d04590d04f78a38a2d"></a>
## value_mut

`function` · `datafusion_physical_expr_common::metrics::Metric::value_mut` · datafusion-physical-expr-common 55.1.0

```rust
fn value_mut(&mut self) -> &mut MetricValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return a mutable reference to the value of this metric

<a id="op-c92a35a662c744d4c66ef2d9"></a>
## with_category

`function` · `datafusion_physical_expr_common::metrics::Metric::with_category` · datafusion-physical-expr-common 55.1.0

```rust
fn with_category(self, category: MetricCategory) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the semantic category for this metric.

See [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4) for details on the determinism properties
of each category.

<a id="op-66003abf0ae88a7f2fdbd0b5"></a>
## with_label

`function` · `datafusion_physical_expr_common::metrics::Metric::with_label` · datafusion-physical-expr-common 55.1.0

```rust
fn with_label(self, label: Label) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add a new label to this metric

<a id="op-4fb1a6652e3cbcb967fd0aa5"></a>
## with_type

`function` · `datafusion_physical_expr_common::metrics::Metric::with_type` · datafusion-physical-expr-common 55.1.0

```rust
fn with_type(self, metric_type: MetricType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::Metric", "path": "Metric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [213, 2], "filename": "src/metrics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the type for this metric. Defaults to [`MetricType::Dev`](../operations/datafusion_common.format.MetricType.md#op-3ccd1cdb2d4e12f173262bb0)
