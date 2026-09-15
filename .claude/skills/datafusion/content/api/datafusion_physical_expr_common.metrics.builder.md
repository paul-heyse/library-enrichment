# `datafusion_physical_expr_common::metrics::builder`

Crate `datafusion-physical-expr-common` · 1 public items · structured records in [`model/datafusion_physical_expr_common.metrics.builder.json`](../model/datafusion_physical_expr_common.metrics.builder.json)

## MetricBuilder

`struct` · `datafusion_physical_expr_common::metrics::builder::MetricBuilder`

Also reachable as `datafusion_physical_expr_common::metrics::MetricBuilder`, `datafusion_physical_plan::metrics::MetricBuilder`

```rust
struct MetricBuilder<'a>
```

**Derives**: Clone

**Methods** (26)

```rust
fn build(self, value: MetricValue)
fn counter(self, counter_name: impl Into<Cow<'static, str>>, partition: usize) -> Count
fn elapsed_compute(self, partition: usize) -> Time
fn end_timestamp(self, partition: usize) -> Timestamp
fn gauge(self, gauge_name: impl Into<Cow<'static, str>>, partition: usize) -> Gauge
fn global_counter(self, counter_name: impl Into<Cow<'static, str>>) -> Count
fn global_gauge(self, gauge_name: impl Into<Cow<'static, str>>) -> Gauge
fn mem_used(self, partition: usize) -> Gauge
fn new(metrics: &'a ExecutionPlanMetricsSet) -> Self
fn output_batches(self, partition: usize) -> Count
fn output_bytes(self, partition: usize) -> Count
fn output_rows(self, partition: usize) -> Count
fn peak_memory_usage(self, gauge_name: impl Into<Cow<'static, str>>, partition: usize) -> Gauge
fn pruning_metrics(self, name: impl Into<Cow<'static, str>>, partition: usize) -> PruningMetrics
fn ratio_metrics(self, name: impl Into<Cow<'static, str>>, partition: usize) -> RatioMetrics
fn ratio_metrics_with_strategy(self, name: impl Into<Cow<'static, str>>, partition: usize, merge_strategy: RatioMergeStrategy) -> RatioMetrics
fn spill_count(self, partition: usize) -> Count
fn spilled_bytes(self, partition: usize) -> Count
fn spilled_rows(self, partition: usize) -> Count
fn start_timestamp(self, partition: usize) -> Timestamp
fn subset_time(self, subset_name: impl Into<Cow<'static, str>>, partition: usize) -> Time
fn with_category(self, category: MetricCategory) -> Self
fn with_label(self, label: Label) -> Self
fn with_new_label(self, name: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self
fn with_partition(self, partition: usize) -> Self
fn with_type(self, metric_type: MetricType) -> Self
```

Structure for constructing metrics, counters, timers, etc.

Note the use of `Cow<..>` is to avoid allocations in the common
case of constant strings. Dynamically created label strings are shared when
[`Label`] values are cloned.

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

---
