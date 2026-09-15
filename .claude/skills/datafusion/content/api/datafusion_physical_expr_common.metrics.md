# `datafusion_physical_expr_common::metrics`

Crate `datafusion-physical-expr-common` · 5 public items · structured records in [`model/datafusion_physical_expr_common.metrics.json`](../model/datafusion_physical_expr_common.metrics.json)

## ExecutionPlanMetricsSet

`struct` · `datafusion_physical_expr_common::metrics::ExecutionPlanMetricsSet`

Also reachable as `datafusion_physical_plan::metrics::ExecutionPlanMetricsSet`

```rust
struct ExecutionPlanMetricsSet
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn clone_inner(&self) -> MetricsSet
fn new() -> Self
fn register(&self, metric: Arc<Metric>)
```

**via `core::convert::From`**

```rust
fn from(metrics: MetricsSet) -> Self
```

A set of [`Metric`]s for an individual operator.

This structure is intended as a convenience for execution plan
implementations so they can generate different streams for multiple
partitions but easily report them together.

Each `clone()` of this structure will add metrics to the same
underlying metrics set

---

## Label

`struct` · `datafusion_physical_expr_common::metrics::Label`

Also reachable as `datafusion_physical_plan::metrics::Label`

```rust
struct Label
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn name(&self) -> &str
fn new(name: impl Into<LabelValue>, value: impl Into<LabelValue>) -> Self
fn value(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

`name=value` pairs identifying a metric. This concept is called various things
in various different systems:

"labels" in
[prometheus](https://prometheus.io/docs/concepts/data_model/) and
"tags" in
[InfluxDB](https://docs.influxdata.com/influxdb/v1.8/write_protocols/line_protocol_tutorial/)
, "attributes" in [open
telemetry]<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/data-model.md>,
etc.

As the name and value are expected to often be constant strings, borrowed
static strings avoid allocations in that common case. Dynamic strings are
stored behind [`Arc<str>`] so cloning labels does not copy the underlying
string data.

---

## LabelValue

`struct` · `datafusion_physical_expr_common::metrics::LabelValue`

Also reachable as `datafusion_physical_plan::metrics::LabelValue`

```rust
struct LabelValue
```

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(value: Arc<str>) -> Self
fn from(value: &'static str) -> Self
fn from(value: Cow<'static, str>) -> Self
fn from(value: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A label name or value.

String literals preserve the existing allocation-free path. Dynamic strings
can be stored behind [`Arc<str>`], so cloning a [`Label`] only increments an
atomic reference count and does not allocate or copy the underlying string
data.

---

## Metric

`struct` · `datafusion_physical_expr_common::metrics::Metric`

Also reachable as `datafusion::physical_plan::Metric`, `datafusion_physical_plan::Metric`, `datafusion_physical_plan::execution_plan::Metric`, `datafusion_physical_plan::metrics::Metric`

```rust
struct Metric
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**Methods** (11)

```rust
fn labels(&self) -> &[Label]
fn metric_category(&self) -> Option<MetricCategory>
fn metric_type(&self) -> MetricType
fn new(value: MetricValue, partition: Option<usize>) -> Self
fn new_with_labels(value: MetricValue, partition: Option<usize>, labels: Vec<Label>) -> Self
fn partition(&self) -> Option<usize>
fn value(&self) -> &MetricValue
fn value_mut(&mut self) -> &mut MetricValue
fn with_category(self, category: MetricCategory) -> Self
fn with_label(self, label: Label) -> Self
fn with_type(self, metric_type: MetricType) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Something that tracks a value of interest (metric) during execution.

Typically [`Metric`]s are not created directly, but instead
are created using [`MetricBuilder`] or methods on
[`ExecutionPlanMetricsSet`].

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

---

## MetricsSet

`struct` · `datafusion_physical_expr_common::metrics::MetricsSet`

Also reachable as `datafusion_physical_plan::metrics::MetricsSet`

```rust
struct MetricsSet
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`

**Derives**: Clone, Debug, Default

**Methods** (16)

```rust
fn aggregate_by_name(&self) -> Self
fn elapsed_compute(&self) -> Option<usize>
fn filter_by_categories(self, allowed: &[MetricCategory]) -> Self
fn filter_by_metric_types(self, allowed: &[MetricType]) -> Self
fn filter_by_names(self, names: &[String]) -> Self
fn iter(&self) -> impl Iterator<Item = &Arc<Metric>>
fn new() -> Self
fn output_rows(&self) -> Option<usize>
fn push(&mut self, metric: Arc<Metric>)
fn sorted_for_display(self) -> Self
fn spill_count(&self) -> Option<usize>
fn spilled_bytes(&self) -> Option<usize>
fn spilled_rows(&self) -> Option<usize>
fn sum<F>(&self, f: F) -> Option<MetricValue> where F: FnMut(&Metric) -> bool
fn sum_by_name(&self, metric_name: &str) -> Option<MetricValue>
fn timestamps_removed(self) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<I: IntoIterator<Item = Arc<Metric>>>(&mut self, iter: I)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Arc<Metric>>>(iter: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

A snapshot of the metrics for a particular execution plan.

---
