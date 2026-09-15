# `datafusion_physical_expr_common::metrics::value`

Crate `datafusion-physical-expr-common` · 9 public items · structured records in [`model/datafusion_physical_expr_common.metrics.value.json`](../model/datafusion_physical_expr_common.metrics.value.json)

## MetricValue

`enum` · `datafusion_physical_expr_common::metrics::value::MetricValue`

Also reachable as `datafusion_physical_expr_common::metrics::MetricValue`, `datafusion_physical_plan::metrics::MetricValue`

```rust
enum MetricValue
```

**Variants**: `OutputRows`, `ElapsedCompute`, `SpillCount`, `SpilledBytes`, `OutputBytes`, `OutputBatches`, `SpilledRows`, `CurrentMemoryUsage`, `Count`, `Gauge`, `PeakMemoryUsage`, `Time`, `StartTimestamp`, `EndTimestamp`, `PruningMetrics`, `Ratio`, `Custom`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq

**Methods** (6)

```rust
fn aggregate(&mut self, other: &Self)
fn as_usize(&self) -> usize
fn display_sort_key(&self) -> u8
fn is_timestamp(&self) -> bool
fn name(&self) -> &str
fn new_empty(&self) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Possible values for a [super::Metric].

Among other differences, the metric types have different ways to
logically interpret their underlying values and some metrics are
so common they are given special treatment.

---

## RatioMergeStrategy

`enum` · `datafusion_physical_expr_common::metrics::value::RatioMergeStrategy`

Also reachable as `datafusion_physical_expr_common::metrics::RatioMergeStrategy`, `datafusion_physical_plan::metrics::RatioMergeStrategy`

```rust
enum RatioMergeStrategy
```

**Variants**: `AddPartAddTotal`, `AddPartSetTotal`, `SetPartAddTotal`

**Derives**: Clone, Debug, Default

---

## Count

`struct` · `datafusion_physical_expr_common::metrics::value::Count`

Also reachable as `datafusion_physical_expr_common::metrics::Count`, `datafusion_physical_plan::metrics::Count`

```rust
struct Count
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (3)

```rust
fn add(&self, n: usize)
fn new() -> Self
fn value(&self) -> usize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A counter to record things such as number of input or output rows

Note `clone`ing counters update the same underlying metrics

---

## Gauge

`struct` · `datafusion_physical_expr_common::metrics::value::Gauge`

Also reachable as `datafusion_physical_expr_common::metrics::Gauge`, `datafusion_physical_plan::metrics::Gauge`

```rust
struct Gauge
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (6)

```rust
fn add(&self, n: usize)
fn new() -> Self
fn set(&self, n: usize) -> usize
fn set_max(&self, n: usize)
fn sub(&self, n: usize)
fn value(&self) -> usize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A gauge is the simplest metrics type. It just returns a value.
For example, you can easily expose current memory consumption with a gauge.

Note `clone`ing gauge update the same underlying metrics

---

## PruningMetrics

`struct` · `datafusion_physical_expr_common::metrics::value::PruningMetrics`

Also reachable as `datafusion_physical_expr_common::metrics::PruningMetrics`, `datafusion_physical_plan::metrics::PruningMetrics`

```rust
struct PruningMetrics
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn add_fully_matched(&self, n: usize)
fn add_matched(&self, n: usize)
fn add_pruned(&self, n: usize)
fn fully_matched(&self) -> usize
fn matched(&self) -> usize
fn new() -> Self
fn pruned(&self) -> usize
fn subtract_matched(&self, n: usize)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Counters tracking pruning metrics

For example, a file scanner initially is planned to scan 10 files, but skipped
8 of them using statistics, the pruning metrics would look like: 10 total -> 2 matched

Note `clone`ing update the same underlying metrics

---

## RatioMetrics

`struct` · `datafusion_physical_expr_common::metrics::value::RatioMetrics`

Also reachable as `datafusion_physical_expr_common::metrics::RatioMetrics`, `datafusion_physical_plan::metrics::RatioMetrics`

```rust
struct RatioMetrics
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (12)

```rust
fn add_part(&self, n: usize)
fn add_total(&self, n: usize)
fn display_raw_values(&self) -> bool
fn merge(&self, other: &Self)
fn merge_strategy(&self) -> &RatioMergeStrategy
fn new() -> Self
fn part(&self) -> usize
fn set_part(&self, n: usize)
fn set_total(&self, n: usize)
fn total(&self) -> usize
fn with_display_raw_values(self, display_raw_values: bool) -> Self
fn with_merge_strategy(self, merge_strategy: RatioMergeStrategy) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Counters tracking ratio metrics (e.g. matched vs total)

The counters are thread-safe and shared across clones.

---

## ScopedTimerGuard

`struct` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard`

Also reachable as `datafusion_physical_expr_common::metrics::ScopedTimerGuard`, `datafusion_physical_plan::metrics::ScopedTimerGuard`

```rust
struct ScopedTimerGuard<'a>
```

**Implements**: `core::ops::drop::Drop`

**Methods** (5)

```rust
fn done(self)
fn done_with(self, end_time: Instant)
fn restart(&mut self)
fn stop(&mut self)
fn stop_with(&mut self, end_time: Instant)
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

RAAI structure that adds all time between its construction and
destruction to the CPU time or the first call to `stop` whichever
comes first

---

## Time

`struct` · `datafusion_physical_expr_common::metrics::value::Time`

Also reachable as `datafusion_physical_expr_common::metrics::Time`, `datafusion_physical_plan::metrics::Time`

```rust
struct Time
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (7)

```rust
fn add(&self, other: &Time)
fn add_duration(&self, duration: Duration)
fn add_elapsed(&self, start: Instant)
fn new() -> Self
fn timer(&self) -> ScopedTimerGuard<'_>
fn timer_with(&self, now: Instant) -> ScopedTimerGuard<'_>
fn value(&self) -> usize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Measure a potentially non contiguous duration of time

---

## Timestamp

`struct` · `datafusion_physical_expr_common::metrics::value::Timestamp`

Also reachable as `datafusion_physical_expr_common::metrics::Timestamp`, `datafusion_physical_plan::metrics::Timestamp`

```rust
struct Timestamp
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (6)

```rust
fn new() -> Self
fn record(&self)
fn set(&self, now: DateTime<Utc>)
fn update_to_max(&self, other: &Timestamp)
fn update_to_min(&self, other: &Timestamp)
fn value(&self) -> Option<DateTime<Utc>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Stores a single timestamp, stored as the number of nanoseconds
elapsed from Jan 1, 1970 UTC

---
