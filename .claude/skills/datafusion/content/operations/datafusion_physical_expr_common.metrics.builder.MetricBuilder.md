# `datafusion_physical_expr_common::metrics::builder::MetricBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.builder.MetricBuilder.json).

<a id="op-69e5b13fd1a8e33cbdc0d1a8"></a>
## MetricBuilder

`struct` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder` · datafusion-physical-expr-common 55.1.0

```rust
struct MetricBuilder<'a>
```

Source: `src/metrics/builder.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Structure for constructing metrics, counters, timers, etc.

Note the use of `Cow<..>` is to avoid allocations in the common
case of constant strings. Dynamically created label strings are shared when
[`Label`](../operations/datafusion_physical_expr_common.metrics.Label.md#op-32cbece590fe957187c93920) values are cloned.

```rust
use datafusion_physical_expr_common::metrics::*;

let metrics = ExecutionPlanMetricsSet::new();
let partition = 1;

// Create the standard output_rows metric
let output_rows = MetricBuilder::new(&metrics).output_rows(partition);

// Create a operator specific counter with some labels
let num_bytes = MetricBuilder::new(&metrics)
    .with_new_label("filename", "my_awesome_file.parquet")
    .counter("num_bytes", partition);
```

<a id="op-183bf0a9dfa488f999afadbb"></a>
## build

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::build` · datafusion-physical-expr-common 55.1.0

```rust
fn build(self, value: MetricValue)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a metric of the specified value
registered with the MetricsSet

<a id="op-512aa90f895a2542e18fdd1f"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> MetricBuilder<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/metrics/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/builder.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80c007f4bb0f6784276af488"></a>
## counter

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::counter` · datafusion-physical-expr-common 55.1.0

```rust
fn counter(self, counter_name: impl Into<Cow<'static, str>>, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`Count`](../operations/datafusion_physical_expr_common.metrics.value.Count.md#op-2921678004049ad87901f32b) for recording some
arbitrary metric of an operator.

<a id="op-6a4a5648b7824e64d9d7eb29"></a>
## elapsed_compute

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::elapsed_compute` · datafusion-physical-expr-common 55.1.0

```rust
fn elapsed_compute(self, partition: usize) -> Time
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new Timer for recording the elapsed
CPU time spent by an operator

<a id="op-5dc08269089dad535e849043"></a>
## end_timestamp

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::end_timestamp` · datafusion-physical-expr-common 55.1.0

```rust
fn end_timestamp(self, partition: usize) -> Timestamp
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new Timestamp for recording the
ending time of execution for a partition

<a id="op-2771c621700c4c1a7144f40f"></a>
## gauge

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::gauge` · datafusion-physical-expr-common 55.1.0

```rust
fn gauge(self, gauge_name: impl Into<Cow<'static, str>>, partition: usize) -> Gauge
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`Gauge`](../operations/datafusion_physical_expr_common.metrics.value.Gauge.md#op-114f4af4648b8e9e64d55ecd) for reporting some
arbitrary metric of an operator.

<a id="op-c9509a49e19994fb7d3ec279"></a>
## global_counter

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::global_counter` · datafusion-physical-expr-common 55.1.0

```rust
fn global_counter(self, counter_name: impl Into<Cow<'static, str>>) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`Count`](../operations/datafusion_physical_expr_common.metrics.value.Count.md#op-2921678004049ad87901f32b) for recording a
metric of an overall operator (not per partition)

<a id="op-531c82a524b8c708f52e9bbb"></a>
## global_gauge

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::global_gauge` · datafusion-physical-expr-common 55.1.0

```rust
fn global_gauge(self, gauge_name: impl Into<Cow<'static, str>>) -> Gauge
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`Gauge`](../operations/datafusion_physical_expr_common.metrics.value.Gauge.md#op-114f4af4648b8e9e64d55ecd) for reporting a
metric of an overall operator (not per partition)

<a id="op-914c56dc186780470147ab8d"></a>
## mem_used

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::mem_used` · datafusion-physical-expr-common 55.1.0

```rust
fn mem_used(self, partition: usize) -> Gauge
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new gauge for reporting current memory usage

<a id="op-f8a203812c409e0555dc9ddc"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(metrics: &'a ExecutionPlanMetricsSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new `MetricBuilder` that will register the result of `build()` with the `metrics`

`self.metric_type` controls when such metric is displayed. See comments in
[`MetricType`](../operations/datafusion_common.format.MetricType.md#op-fb6314931b98a471bb076826) for details.

<a id="op-27c269d391cd7c3754f307ad"></a>
## output_batches

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::output_batches` · datafusion-physical-expr-common 55.1.0

```rust
fn output_batches(self, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new counter for recording total output batches

<a id="op-661e8b8efcf1777d71c12b75"></a>
## output_bytes

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::output_bytes` · datafusion-physical-expr-common 55.1.0

```rust
fn output_bytes(self, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new counter for recording total output bytes

<a id="op-23d682010f023cb04db9b595"></a>
## output_rows

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::output_rows` · datafusion-physical-expr-common 55.1.0

```rust
fn output_rows(self, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new counter for recording output rows

<a id="op-834ec423af3b4511c4f98327"></a>
## peak_memory_usage

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::peak_memory_usage` · datafusion-physical-expr-common 55.1.0

```rust
fn peak_memory_usage(self, gauge_name: impl Into<Cow<'static, str>>, partition: usize) -> Gauge
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`Gauge`](../operations/datafusion_physical_expr_common.metrics.value.Gauge.md#op-114f4af4648b8e9e64d55ecd) for recording peak memory
usage in bytes.

<a id="op-37968cc9f6f47e4ad235b7a5"></a>
## pruning_metrics

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::pruning_metrics` · datafusion-physical-expr-common 55.1.0

```rust
fn pruning_metrics(self, name: impl Into<Cow<'static, str>>, partition: usize) -> PruningMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new `PruningMetrics`

<a id="op-fc9d83e548fe03ce8ed0e41f"></a>
## ratio_metrics

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::ratio_metrics` · datafusion-physical-expr-common 55.1.0

```rust
fn ratio_metrics(self, name: impl Into<Cow<'static, str>>, partition: usize) -> RatioMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`RatioMetrics`](../operations/datafusion_physical_expr_common.metrics.value.RatioMetrics.md#op-a47dbfa1bdc8f5bf2bf4eb22)

<a id="op-589640d90e2ff94bdf044d2d"></a>
## ratio_metrics_with_strategy

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::ratio_metrics_with_strategy` · datafusion-physical-expr-common 55.1.0

```rust
fn ratio_metrics_with_strategy(self, name: impl Into<Cow<'static, str>>, partition: usize, merge_strategy: RatioMergeStrategy) -> RatioMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new [`RatioMetrics`](../operations/datafusion_physical_expr_common.metrics.value.RatioMetrics.md#op-a47dbfa1bdc8f5bf2bf4eb22) with a specific merge strategy

<a id="op-cbbb4bc26e59a631c723dfa7"></a>
## spill_count

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::spill_count` · datafusion-physical-expr-common 55.1.0

```rust
fn spill_count(self, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new counter for recording the number of spills
triggered by an operator

<a id="op-73ac98dd83e31e4f244519fe"></a>
## spilled_bytes

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::spilled_bytes` · datafusion-physical-expr-common 55.1.0

```rust
fn spilled_bytes(self, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new counter for recording the total spilled bytes
triggered by an operator

<a id="op-022b9d2c9dc2f72c922ceb7d"></a>
## spilled_rows

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::spilled_rows` · datafusion-physical-expr-common 55.1.0

```rust
fn spilled_rows(self, partition: usize) -> Count
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consume self and create a new counter for recording the total spilled rows
triggered by an operator

<a id="op-b16de623e7c6f771d6cd7938"></a>
## start_timestamp

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::start_timestamp` · datafusion-physical-expr-common 55.1.0

```rust
fn start_timestamp(self, partition: usize) -> Timestamp
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new Timestamp for recording the
starting time of execution for a partition

<a id="op-9e47ad0d362e433b6042ed99"></a>
## subset_time

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::subset_time` · datafusion-physical-expr-common 55.1.0

```rust
fn subset_time(self, subset_name: impl Into<Cow<'static, str>>, partition: usize) -> Time
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Consumes self and creates a new Timer for recording some
subset of an operators execution time.

<a id="op-83e0cdaa1724f94ef5568d55"></a>
## with_category

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::with_category` · datafusion-physical-expr-common 55.1.0

```rust
fn with_category(self, category: MetricCategory) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the semantic category for the metric being constructed.

See [`MetricCategory`](../operations/datafusion_common.format.MetricCategory.md#op-d05732d49bd39fe3321965e4) for details on the determinism properties
of each category.

<a id="op-32b5556a7fb7d8a515bc700d"></a>
## with_label

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::with_label` · datafusion-physical-expr-common 55.1.0

```rust
fn with_label(self, label: Label) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add a label to the metric being constructed

<a id="op-ece19cee18841328c2065ab0"></a>
## with_new_label

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::with_new_label` · datafusion-physical-expr-common 55.1.0

```rust
fn with_new_label(self, name: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add a label to the metric being constructed

<a id="op-196996a00964a0fa52d80ad7"></a>
## with_partition

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::with_partition` · datafusion-physical-expr-common 55.1.0

```rust
fn with_partition(self, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the partition of the metric being constructed

<a id="op-dd9b3e8dd881f354a62ff1a1"></a>
## with_type

`function` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder::with_type` · datafusion-physical-expr-common 55.1.0

```rust
fn with_type(self, metric_type: MetricType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::builder::MetricBuilder", "path": "MetricBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [358, 2], "filename": "src/metrics/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/builder.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Set the metric type to the metric being constructed
