# `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.baseline.BaselineMetrics.json).

<a id="op-8283b32c23bd1e20dcbbc8aa"></a>
## BaselineMetrics

`struct` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics` · datafusion-physical-expr-common 55.1.0

```rust
struct BaselineMetrics
```

Source: `src/metrics/baseline.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Helper for creating and tracking common "baseline" metrics for
each operator

Example:
```
use datafusion_physical_expr_common::metrics::{
    BaselineMetrics, ExecutionPlanMetricsSet,
};
let metrics = ExecutionPlanMetricsSet::new();

let partition = 2;
let baseline_metrics = BaselineMetrics::new(&metrics, partition);

// during execution, in CPU intensive operation:
let timer = baseline_metrics.elapsed_compute().timer();
// .. do CPU intensive work
timer.done();

// when operator is finished:
baseline_metrics.done();
```

<a id="op-6ac61b8f30927fcd849b305b"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> BaselineMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 17], "end": [52, 22], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/baseline.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b00bc7fb51c34e23921ff89"></a>
## done

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::done` · datafusion-physical-expr-common 55.1.0

```rust
fn done(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Records the fact that this operator's execution is complete
(recording the `end_time` metric).

Note care should be taken to call `done()` manually if
`BaselineMetrics` is not `drop`ped immediately upon operator
completion, as async streams may not be dropped immediately
depending on the consumer.

<a id="op-f4d40ce6b067498396dba26b"></a>
## drop

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::drop` · datafusion-physical-expr-common 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [238, 2], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/metrics/baseline.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acf181a234c4cfed8668bd0f"></a>
## elapsed_compute

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::elapsed_compute` · datafusion-physical-expr-common 55.1.0

```rust
fn elapsed_compute(&self) -> &Time
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

return the metric for cpu time spend in this operator

<a id="op-7a72b23c4ab930b26ec2664b"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/metrics/baseline.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/baseline.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d73cab56549c0c7b94ee4d0"></a>
## intermediate

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::intermediate` · datafusion-physical-expr-common 55.1.0

```rust
fn intermediate(&self) -> BaselineMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a [`BaselineMetrics`](../operations/datafusion_physical_expr_common.metrics.baseline.BaselineMetrics.md#op-8283b32c23bd1e20dcbbc8aa) that updates the same `elapsed_compute` ignoring
all other metrics

This is useful when an operator offloads some of its intermediate work to separate tasks
that as a result won't be recorded by [`Self::record_poll`](../operations/datafusion_physical_expr_common.metrics.baseline.BaselineMetrics.md#op-26e3977564f8bce55aadac0d)

<a id="op-9a327980c6ace4bca3d66e5c"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new BaselineMetric structure, and set `start_time` to now

<a id="op-2726682da2ddeddea25f2dc2"></a>
## output_batches

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::output_batches` · datafusion-physical-expr-common 55.1.0

```rust
fn output_batches(&self) -> &Count
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

return the metric for the total number of output batches produced

<a id="op-01834f3b032fb95e5cc61599"></a>
## output_rows

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::output_rows` · datafusion-physical-expr-common 55.1.0

```rust
fn output_rows(&self) -> &Count
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

return the metric for the total number of output rows produced

<a id="op-16d94d1a35648bbaf8e5a5e5"></a>
## output_rows_skew_metric

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::output_rows_skew_metric` · datafusion-physical-expr-common 55.1.0

```rust
fn output_rows_skew_metric(metrics: &MetricsSet) -> Option<Arc<Metric>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a derived metric that summarizes how unevenly `output_rows`
are distributed across partitions.

The score is normalized to the range `[0%, 100%]`, where `0%`
indicates a perfectly balanced distribution and `100%` indicates the
most skewed distribution.

The calculation is:
`effective_parallelism = square(sum(r_i)) / sum(square(r_i))`
`output_rows_skew = (1 - ((effective_parallelism - 1) / (partition_count - 1))) * 100%`

Example: for 4 partitions with output rows `[10, 10, 10, 10]`,
`effective_parallelism = 40^2 / (10^2 + 10^2 + 10^2 + 10^2) = 4`,
so `output_rows_skew = 0%`. For `[40, 0, 0, 0]`, the score is `100%`.

<a id="op-e9eb875095a0838180bedafe"></a>
## record_output

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::record_output` · datafusion-physical-expr-common 55.1.0

```rust
fn record_output(&self, num_rows: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Record that some number of rows have been produced as output

See the [`RecordOutput`](../operations/datafusion_physical_expr_common.metrics.baseline.RecordOutput.md#op-3d47c46d82c1365d5c0c4dae) for conveniently recording record
batch output for other thing

<a id="op-26e3977564f8bce55aadac0d"></a>
## record_poll

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::record_poll` · datafusion-physical-expr-common 55.1.0

```rust
fn record_poll(&self, poll: Poll<Option<Result<RecordBatch>>>) -> Poll<Option<Result<RecordBatch>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Process a poll result of a stream producing output for an operator.

Note: this method only updates `output_rows` and `end_time` metrics.
Remember to update `elapsed_compute` and other metrics manually.

<a id="op-fcf423a785f1fb2e66f59beb"></a>
## try_done

`function` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics::try_done` · datafusion-physical-expr-common 55.1.0

```rust
fn try_done(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::baseline::BaselineMetrics", "path": "BaselineMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [232, 2], "filename": "src/metrics/baseline.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/baseline.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

If not previously recorded `done()`, record
