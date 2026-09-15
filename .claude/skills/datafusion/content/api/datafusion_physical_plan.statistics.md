# `datafusion_physical_plan::statistics`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.statistics.json`](../model/datafusion_physical_plan.statistics.json)

## ChildStats

`enum` · `datafusion_physical_plan::statistics::ChildStats`

Also reachable as `datafusion::physical_plan::ChildStats`, `datafusion_physical_plan::ChildStats`

```rust
enum ChildStats
```

**Variants**: `At`, `Skip`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Directive returned by [`ExecutionPlan::child_stats_requests`] describing
how the [`StatisticsContext`] should obtain each child's statistics.

---

## StatisticsArgs

`struct` · `datafusion_physical_plan::statistics::StatisticsArgs`

Also reachable as `datafusion::physical_plan::StatisticsArgs`, `datafusion_physical_plan::StatisticsArgs`

```rust
struct StatisticsArgs
```

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn new() -> Self
fn partition(&self) -> Option<usize>
fn set_partition(&mut self, partition: Option<usize>)
fn with_partition(self, partition: Option<usize>) -> Self
```

Arguments passed to [`ExecutionPlan::statistics_from_inputs`] carrying
external information that operators can use when computing their
statistics.

---

## StatisticsContext

`struct` · `datafusion_physical_plan::statistics::StatisticsContext`

Also reachable as `datafusion::physical_plan::StatisticsContext`, `datafusion_physical_plan::StatisticsContext`

```rust
struct StatisticsContext
```

**Derives**: Default

**Methods** (3)

```rust
fn compute(&self, plan: &dyn ExecutionPlan, args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn new() -> Self
fn reset_cache(&self)
```

Owns the bottom-up traversal and per-walk memoization cache for statistics
computation. Call [`StatisticsContext::compute`] to walk a plan tree.

---
