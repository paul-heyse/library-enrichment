# `datafusion_physical_plan::operator_statistics`

Crate `datafusion-physical-plan` · 15 public items · structured records in [`model/datafusion_physical_plan.operator_statistics.json`](../model/datafusion_physical_plan.operator_statistics.json)

## StatisticsResult

`enum` · `datafusion_physical_plan::operator_statistics::StatisticsResult`

```rust
enum StatisticsResult
```

**Variants**: `Computed`, `Delegate`

**Derives**: Debug

Result of attempting to compute statistics with a [`StatisticsProvider`].

---

## ndv_after_selectivity

`function` · `datafusion_physical_plan::operator_statistics::ndv_after_selectivity`

```rust
fn ndv_after_selectivity(original_ndv: usize, original_rows: usize, selectivity: f64) -> usize
```

Estimate NDV after applying a selectivity factor (filtering).

When filtering rows, each distinct value has multiple rows. If a value
appears `k` times, the probability it survives the filter is `1 - (1-s)^k`
where `s` is the selectivity.

Assuming uniform distribution (each value appears `rows/ndv` times):
```text
NDV_after ~ NDV_before * [1 - (1 - selectivity)^(rows/NDV)]
```

---

## num_distinct_vals

`function` · `datafusion_physical_plan::operator_statistics::num_distinct_vals`

```rust
fn num_distinct_vals(domain_size: usize, num_selected: usize) -> usize
```

Estimate the number of distinct values when sampling from a population.

Given a domain with `domain_size` distinct values and `num_selected` rows
sampled/filtered from it, estimates how many distinct values will appear
in the sample.

Uses the formula: `Expected distinct = N * [1 - (1 - 1/N)^n]`

# References

Based on Calcite's `RelMdUtil.numDistinctVals()`:
<https://github.com/apache/calcite/blob/main/core/src/main/java/org/apache/calcite/rel/metadata/RelMdUtil.java>

---

## AggregateStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider`

```rust
struct AggregateStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for [`AggregateExec`](crate::aggregates::AggregateExec)
that estimates output cardinality from the NDV of GROUP BY columns.

For each GROUP BY column, looks up `distinct_count` from the enhanced
child statistics. The estimated output rows is the product of all
column NDVs, capped at the input row count. This assumes independence
between columns, so correlated columns (e.g., `city` and `state`) will
produce overestimates.

For GROUPING SETS / CUBE / ROLLUP, delegates to the built-in
`partition_statistics`, which handles per-set NDV estimation correctly.

Delegates when:
- The plan is not an `AggregateExec`
- The aggregate is `Partial` (per-partition, not bounded by global NDV)
- GROUP BY is empty (scalar aggregate)
- Any GROUP BY expression is not a simple column reference
- Any GROUP BY column lacks NDV information

---

## ClosureStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider`

```rust
struct ClosureStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug

**Methods** (1)

```rust
fn new(f: impl Fn(&dyn ExecutionPlan, &[ExtendedStatistics]) -> Result<StatisticsResult> + Send + Sync + 'static) -> Self
```

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

A [`StatisticsProvider`] backed by a user-supplied closure.

Useful for injecting custom statistics in tests or for cardinality feedback
pipelines where real runtime statistics need to override plan estimates.
The closure receives the current plan node and its children's enhanced
statistics, returning a [`StatisticsResult`].

To distinguish between multiple nodes of the same type (e.g., two
`FilterExec` nodes), match on structural properties like the input schema's
column names, number of columns, or child row counts.

# Example

```rust,ignore (requires crate-internal imports)
let provider = ClosureStatisticsProvider::new(|plan, child_stats| {
    if plan.downcast_ref::<FilterExec>().is_some() {
        Ok(StatisticsResult::Computed(ExtendedStatistics::from(Statistics {
            num_rows: Precision::Inexact(42),
            ..Statistics::new_unknown(plan.schema().as_ref())
        })))
    } else {
        Ok(StatisticsResult::Delegate)
    }
});
```

---

## DefaultStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider`

```rust
struct DefaultStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, _child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Default statistics provider that delegates to each operator's built-in
`partition_statistics` implementation.

---

## ExtendedStatistics

`struct` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics`

```rust
struct ExtendedStatistics
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn base(&self) -> &Statistics
fn base_arc(&self) -> &Arc<Statistics>
fn get_extension<T: 'static + Send + Sync>(&self) -> Option<&T>
fn has_extension<T: 'static + Send + Sync>(&self) -> bool
fn merge_extensions(&mut self, other: &ExtendedStatistics)
fn new(base: Statistics) -> Self
fn new_arc(base: Arc<Statistics>) -> Self
fn set_extension<T: 'static + Send + Sync>(&mut self, value: T)
```

**via `core::convert::From`**

```rust
fn from(base: Statistics) -> Self
fn from(base: Arc<Statistics>) -> Self
```

Statistics with support for custom extensions.

Wraps the standard [`Statistics`] and adds a type-erased extension map
for custom statistics like histograms, sketches, or domain-specific metadata.

# Example

```ignore
// Define a custom statistics extension
#[derive(Debug, Clone)]
struct HistogramStats {
    buckets: Vec<(i64, i64, usize)>, // (min, max, count)
}

// Set extension in a planner
let mut stats = ExtendedStatistics::from(base_stats);
stats.set_extension(HistogramStats { buckets: vec![] });

// Retrieve in a consumer
if let Some(hist) = stats.get_extension::<HistogramStats>() {
    // Use histogram for better estimation
}
```

---

## FilterStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider`

```rust
struct FilterStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for [`FilterExec`](crate::filter::FilterExec) that uses
pre-computed enhanced child statistics from the registry walk.

Unlike the default provider (which calls `partition_statistics` and gets raw
child stats), this provider receives enhanced child stats that may include
NDV overrides injected at the scan level. It applies the same selectivity
estimation logic as `FilterExec::statistics_helper`, then additionally
adjusts each column's `distinct_count` using [`ndv_after_selectivity`] based
on the computed selectivity ratio.

---

## JoinStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider`

```rust
struct JoinStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for equi-joins (hash join, sort-merge join) and cross joins.

For equi-joins, estimates output cardinality as
`left_rows * right_rows / product(max(left_ndv_i, right_ndv_i))`
across all join key columns (assuming independence between keys),
falling back to the Cartesian product when any key lacks NDV on both sides.
For cross joins, uses the exact Cartesian product.

The base inner-join estimate is then adjusted for the join type:
- Semi joins: capped at the preserved-side row count
- Anti joins: preserved-side minus matched rows (clamped to 0)
- Left/Right outer: at least as many rows as the preserved side
- Full outer: at least `left + right - inner_estimate`
- Left mark: exactly `left_rows` (one output row per left row)

Delegates when:
- The plan is not a supported join type
- Either input lacks row count information

---

## LimitStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider`

```rust
struct LimitStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for [`LocalLimitExec`](crate::limit::LocalLimitExec) and
[`GlobalLimitExec`](crate::limit::GlobalLimitExec).

Caps output row count at the limit value, accounting for any leading skip offset
in `GlobalLimitExec`.

---

## PassthroughStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider`

```rust
struct PassthroughStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for single-input operators with
[`CardinalityEffect::Equal`](crate::execution_plan::CardinalityEffect::Equal).

These operators (Sort, Repartition, CoalescePartitions, etc.) don't
transform statistics, so we pass through the enhanced child stats directly.
This avoids the fallback calling `partition_statistics(None)` which would
trigger a redundant internal recursion with raw (non-enhanced) stats.

---

## ProjectionStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider`

```rust
struct ProjectionStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for [`ProjectionExec`](crate::projection::ProjectionExec)
that uses pre-computed enhanced child statistics from the registry walk.

Maps enhanced child column statistics to output columns based on the
projection expressions, preserving NDV and other statistics through
column references.

---

## StatisticsRegistry

`struct` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry`

```rust
struct StatisticsRegistry
```

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn compute(&self, plan: &dyn ExecutionPlan) -> Result<ExtendedStatistics>
fn compute_base(&self, plan: &dyn ExecutionPlan) -> Result<Statistics>
fn default_with_builtin_providers() -> Self
fn new() -> Self
fn providers(&self) -> &[Arc<dyn StatisticsProvider>]
fn register(&mut self, provider: Arc<dyn StatisticsProvider>)
fn with_providers(providers: Vec<Arc<dyn StatisticsProvider>>) -> Self
```

Registry that chains [`StatisticsProvider`] implementations.

The registry is a stateless provider chain: it holds no mutable state
and is cheaply `Clone`able / `Send` / `Sync`.

---

## UnionStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider`

```rust
struct UnionStatisticsProvider
```

**Implements**: `datafusion_physical_plan::operator_statistics::StatisticsProvider`

**Derives**: Debug, Default

**via `datafusion_physical_plan::operator_statistics::StatisticsProvider`**

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Statistics provider for [`UnionExec`](crate::union::UnionExec).

Sums row counts across all inputs.

---

## StatisticsProvider

`trait` · `datafusion_physical_plan::operator_statistics::StatisticsProvider`

```rust
trait StatisticsProvider: Debug + Send + Sync
```

**Implementors** (9)

- `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider`
- `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider`

**Methods** (1)

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Customize statistics computation for [`ExecutionPlan`] nodes.

Implementations can handle specific operator types or override default
estimation logic. The chain of providers is traversed until one returns
[`StatisticsResult::Computed`].

# Implementing a Custom Provider

```ignore
#[derive(Debug)]
struct MyStatisticsProvider;

impl StatisticsProvider for MyStatisticsProvider {
    fn compute_statistics(
        &self,
        plan: &dyn ExecutionPlan,
        child_stats: &[ExtendedStatistics],
    ) -> Result<StatisticsResult> {
        if let Some(my_exec) = plan.downcast_ref::<MyCustomExec>() {
            // Custom logic for MyCustomExec
            Ok(StatisticsResult::Computed(/* ... */))
        } else {
            // Let next provider handle it
            Ok(StatisticsResult::Delegate)
        }
    }
}
```

---
