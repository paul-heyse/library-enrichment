# `datafusion_ffi::physical_expr::metrics`

Crate `datafusion-ffi` · 9 public items · structured records in [`model/datafusion_ffi.physical_expr.metrics.json`](../model/datafusion_ffi.physical_expr.metrics.json)

## FFI_MetricCategory

`enum` · `datafusion_ffi::physical_expr::metrics::FFI_MetricCategory`

```rust
enum FFI_MetricCategory
```

**Variants**: `Rows`, `Bytes`, `Timing`, `Uncategorized`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_MetricCategory.md).


FFI-stable mirror of [`MetricCategory`].

---

## FFI_MetricType

`enum` · `datafusion_ffi::physical_expr::metrics::FFI_MetricType`

```rust
enum FFI_MetricType
```

**Variants**: `Summary`, `Dev`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_MetricType.md).


FFI-stable mirror of [`MetricType`].

---

## FFI_MetricValue

`enum` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue`

```rust
enum FFI_MetricValue
```

**Variants**: `OutputRows`, `ElapsedComputeNs`, `SpillCount`, `SpilledBytes`, `OutputBytes`, `OutputBatches`, `SpilledRows`, `CurrentMemoryUsage`, `Count`, `Gauge`, `Time`, `StartTimestampNsUTC`, `EndTimestampNsUTC`, `PruningMetrics`, `Ratio`, `Custom`, `PeakMemoryUsage`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_MetricValue.md).


FFI-stable mirror of [`MetricValue`].

This is part of the stable ABI and must not be reordered. New variants must be
appended at the end.

---

## FFI_RatioMergeStrategy

`enum` · `datafusion_ffi::physical_expr::metrics::FFI_RatioMergeStrategy`

```rust
enum FFI_RatioMergeStrategy
```

**Variants**: `AddPartAddTotal`, `AddPartSetTotal`, `SetPartAddTotal`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_RatioMergeStrategy.md).


FFI-stable mirror of [`RatioMergeStrategy`].

---

## FFI_Label

`struct` · `datafusion_ffi::physical_expr::metrics::FFI_Label`

```rust
struct FFI_Label
```

**Fields**: `name`, `value`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_Label.md).


FFI-stable mirror of [`Label`].

---

## FFI_Metric

`struct` · `datafusion_ffi::physical_expr::metrics::FFI_Metric`

```rust
struct FFI_Metric
```

**Fields**: `value`, `labels`, `partition`, `metric_type`, `metric_category`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_Metric.md).


FFI-stable mirror of [`Metric`].

---

## FFI_MetricsSet

`struct` · `datafusion_ffi::physical_expr::metrics::FFI_MetricsSet`

```rust
struct FFI_MetricsSet
```

**Fields**: `metrics`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_MetricsSet.md).


FFI-stable mirror of [`MetricsSet`].

---

## FFI_PruningMetrics

`struct` · `datafusion_ffi::physical_expr::metrics::FFI_PruningMetrics`

```rust
struct FFI_PruningMetrics
```

**Fields**: `pruned`, `matched`, `fully_matched`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_PruningMetrics.md).


FFI-stable mirror of [`PruningMetrics`]. All counts are snapshotted at
conversion time.

---

## FFI_RatioMetrics

`struct` · `datafusion_ffi::physical_expr::metrics::FFI_RatioMetrics`

```rust
struct FFI_RatioMetrics
```

**Fields**: `part`, `total`, `merge_strategy`, `display_raw_values`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.metrics.FFI_RatioMetrics.md).


FFI-stable mirror of [`RatioMetrics`]. Numerator/denominator are
snapshotted at conversion time.

---
