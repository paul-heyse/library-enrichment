# `datafusion_physical_expr_common::metrics::baseline`

Crate `datafusion-physical-expr-common` · 4 public items · structured records in [`model/datafusion_physical_expr_common.metrics.baseline.json`](../model/datafusion_physical_expr_common.metrics.baseline.json)

## BaselineMetrics

`struct` · `datafusion_physical_expr_common::metrics::baseline::BaselineMetrics`

Also reachable as `datafusion_physical_expr_common::metrics::BaselineMetrics`, `datafusion_physical_plan::metrics::BaselineMetrics`

```rust
struct BaselineMetrics
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn done(&self)
fn elapsed_compute(&self) -> &Time
fn intermediate(&self) -> BaselineMetrics
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
fn output_batches(&self) -> &Count
fn output_rows(&self) -> &Count
fn output_rows_skew_metric(metrics: &MetricsSet) -> Option<Arc<Metric>>
fn record_output(&self, num_rows: usize)
fn record_poll(&self, poll: Poll<Option<Result<RecordBatch>>>) -> Poll<Option<Result<RecordBatch>>>
fn try_done(&self)
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.metrics.baseline.BaselineMetrics.md).


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

---

## SpillMetrics

`struct` · `datafusion_physical_expr_common::metrics::baseline::SpillMetrics`

Also reachable as `datafusion_physical_expr_common::metrics::SpillMetrics`, `datafusion_physical_plan::metrics::SpillMetrics`

```rust
struct SpillMetrics
```

**Fields**: `spill_file_count`, `spilled_bytes`, `spilled_rows`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.metrics.baseline.SpillMetrics.md).


Helper for creating and tracking spill-related metrics for
each operator

---

## SplitMetrics

`struct` · `datafusion_physical_expr_common::metrics::baseline::SplitMetrics`

Also reachable as `datafusion_physical_expr_common::metrics::SplitMetrics`, `datafusion_physical_plan::metrics::SplitMetrics`

```rust
struct SplitMetrics
```

**Fields**: `batches_split`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.metrics.baseline.SplitMetrics.md).


Metrics for tracking batch splitting activity

---

## RecordOutput

`trait` · `datafusion_physical_expr_common::metrics::baseline::RecordOutput`

Also reachable as `datafusion_physical_expr_common::metrics::RecordOutput`, `datafusion_physical_plan::metrics::RecordOutput`

```rust
trait RecordOutput
```

**Implementors** (3)

- `arrow_array::record_batch::RecordBatch`
- `core::option::Option`
- `datafusion_common::error::Result`

**Methods** (1)

```rust
fn record_output(self, bm: &BaselineMetrics) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.metrics.baseline.RecordOutput.md).


Trait for things that produce output rows as a result of execution.

---
