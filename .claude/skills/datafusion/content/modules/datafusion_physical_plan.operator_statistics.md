# `datafusion_physical_plan::operator_statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.json).

<a id="op-4a39e5c7429ec59fa523cba3"></a>
## operator_statistics

`module` · `datafusion_physical_plan::operator_statistics` · datafusion-physical-plan 55.1.0

```rust
mod operator_statistics
```

Source: `src/operator_statistics/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Pluggable statistics propagation for physical plans.

This module provides an extensible mechanism for computing statistics
on [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) nodes, following the chain of responsibility pattern
similar to `RelationPlanner` for SQL parsing.

# Overview

The default implementation delegates to each operator's built-in
`partition_statistics`. Users can register custom [`StatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.StatisticsProvider.md#op-b3050eb3d6b996dd9e336fb5)
implementations to:

1. Provide statistics for custom [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) implementations
2. Override default estimation with advanced approaches (e.g., histograms)
3. Plug in domain-specific knowledge for better cardinality estimation

# Architecture

- [`StatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.StatisticsProvider.md#op-b3050eb3d6b996dd9e336fb5): Chain element that computes statistics for specific operators
- [`StatisticsRegistry`](../operations/datafusion_physical_plan.operator_statistics.StatisticsRegistry.md#op-058fa5423a9fbd82dda192ea): Chains providers, lives in SessionState
- [`ExtendedStatistics`](../operations/datafusion_physical_plan.operator_statistics.ExtendedStatistics.md#op-b8de662ae83c9d7215d8ad00): Statistics with type-safe custom extensions

# Built-in Providers

The following providers are included and can be registered in this order:

1. [`FilterStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.FilterStatisticsProvider.md#op-59066bd11e2065860e644852) - selectivity-based filter estimation
2. [`ProjectionStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.ProjectionStatisticsProvider.md#op-c2b09bcaeb037f1e11a4b411) - column mapping through projections
3. [`PassthroughStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.PassthroughStatisticsProvider.md#op-644756eff17c7a9b7fd6c3a5) - passthrough for cardinality-preserving operators
4. [`AggregateStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.AggregateStatisticsProvider.md#op-d28aa37ea36e0bf183ef455b) - NDV-based GROUP BY cardinality estimation
5. [`JoinStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.JoinStatisticsProvider.md#op-192cd9d26ef10e0d08f3f4ef) - NDV-based join output estimation (hash, sort-merge, cross)
6. [`LimitStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.LimitStatisticsProvider.md#op-146faf3a043b4d34b5169fa8) - caps output at the fetch limit (local and global)
7. [`UnionStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.UnionStatisticsProvider.md#op-57284633242dfde6dceb8223) - sums input row counts
8. [`DefaultStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.DefaultStatisticsProvider.md#op-5664b61a0d6c36c14eab5141) - fallback to `partition_statistics(None)`

# Relationship to [#20184](https://github.com/apache/datafusion/issues/20184)

This module performs its own bottom-up tree walk in [`StatisticsRegistry::compute`](../operations/datafusion_physical_plan.operator_statistics.StatisticsRegistry.md#op-bbf9cc3b14d694b23398d7a8),
separate from the walk optimizer rules do via `transform_up`. This means existing
rules that call `partition_statistics` directly bypass the registry.

[#20184](https://github.com/apache/datafusion/issues/20184) adds a `child_stats`
parameter to `partition_statistics`. Once it lands, the registry can feed enriched
**base** [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) into operators' built-in `partition_statistics` calls,
removing redundancy for the base-stats path (row counts, column stats). However,
the separate registry walk is still required for [`ExtendedStatistics`](../operations/datafusion_physical_plan.operator_statistics.ExtendedStatistics.md#op-b8de662ae83c9d7215d8ad00) extension
propagation: `partition_statistics` returns `Arc<Statistics>`, so extensions
(histograms, sketches, etc.) are stripped at that boundary and can only flow
through the registry walk.

If [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) itself were extended to carry a type-erased extension map
(similar to [`ExtendedStatistics`](../operations/datafusion_physical_plan.operator_statistics.ExtendedStatistics.md#op-b8de662ae83c9d7215d8ad00)), the registry walk could be dropped entirely:
extensions would flow naturally through `partition_statistics(child_stats)` and
the registry would become a pure chain-of-responsibility on top of the existing
traversal with no separate walk needed.

# Example

```ignore
use datafusion_physical_plan::operator_statistics::*;

// Create registry with default provider
let mut registry = StatisticsRegistry::new();

// Register custom provider (higher priority)
registry.register(Arc::new(MyHistogramProvider));

// Compute statistics through the chain
let stats = registry.compute(plan.as_ref())?;
```
