# `datafusion_physical_plan::aggregates`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.json).

<a id="op-d4cc306244974d635bf8ea56"></a>
## aggregates

`module` · `datafusion_physical_plan::aggregates` · datafusion-physical-plan 55.1.0

```rust
mod aggregates
```

Source: `src/aggregates/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Aggregate functionality

# Aggregate planning

DataFusion selects different aggregate implementations (streams) based on the
query shape and configuration. This section provides an overview of the
available stream variants.

See each stream's documentation for details.

## 1. Two-stage hash aggregation

Two-stage hash aggregation is used for regular parallel execution.

The input passes through three execution operators to produce the final
aggregation result:

1. Partial aggregation reads the input and produces partial states. It
   aggregates independently within each partition, which usually reduces
   cardinality before the later shuffle.
2. Hash repartitioning on the group keys sends all partial states for each
   group to the same output partition for final aggregation.
3. Final aggregation reads the partial states, combines them, and emits the
   final results.

```text
AggregateExec (final)
  RepartitionExec (hash by group keys)
    AggregateExec (partial)
```

See [`PartialHashAggregateStream`] and [`FinalHashAggregateStream`] for details.

### Ordering optimization

When the input is ordered by the group key, an ordered fast path is used. It
uses a similar two-stage hash aggregation with an early-emission optimization.

```text
AggregateExec (final, ordered)
  RepartitionExec (hash by group keys, order-preserving)
    AggregateExec (partial, ordered)
```

See [`OrderedPartialAggregateStream`] and [`OrderedFinalAggregateStream`] for
details.

Related configuration:

- [`datafusion.execution.target_partitions`](datafusion_common::config::ExecutionOptions::target_partitions)
- [`datafusion.optimizer.repartition_aggregations`](datafusion_common::config::OptimizerOptions::repartition_aggregations)
- [`datafusion.optimizer.prefer_existing_sort`](datafusion_common::config::OptimizerOptions::prefer_existing_sort)

## 2. Single-stage hash aggregation

When there is a single partition, or the aggregation input is already
key-partitioned (e.g., a data source has existing range partitioning),
`Single` mode aggregation is used.

It takes raw input and directly produces the final result.

```text
AggregateExec (mode=Single or SinglePartitioned)
  input
```

See [`SingleHashAggregateStream`] for details.

Related configuration:

- [`datafusion.execution.target_partitions`](datafusion_common::config::ExecutionOptions::target_partitions)
- [`datafusion.optimizer.repartition_aggregations`](datafusion_common::config::OptimizerOptions::repartition_aggregations)

## 3. Aggregation without grouping expressions

A global aggregate maintains one accumulator set per input partition rather
than a hash table of groups. Partial stages compute local states and a final
stage combines them into one output row:

```text
AggregateExec (final, no-grouping)
  CoalescePartitionsExec
    AggregateExec (partial, no-grouping)
```

Every stage without grouping expressions uses [`AggregateStream`]. This path
is selected before the grouped-stream migration setting is considered.

## 4. Grouped TopK aggregation

When a query only needs the best `N` groups, retaining every group in a hash
table and sorting them afterward does unnecessary work. The optimizer pushes
the sort limit and direction into the aggregate:

```text
SortExec (fetch=N)
  AggregateExec (limit=N, order=...)
    input
```

[`GroupedTopKAggregateStream`] keeps a bounded priority map for a single group
key. It supports group-by-only queries and compatible `MIN` or `MAX`
aggregates. An unordered group-by-only soft limit instead stays on the normal
hash aggregation path.

Related configuration:

- [`datafusion.optimizer.enable_topk_aggregation`](datafusion_common::config::OptimizerOptions::enable_topk_aggregation)
- [`datafusion.optimizer.enable_distinct_aggregation_soft_limit`](datafusion_common::config::OptimizerOptions::enable_distinct_aggregation_soft_limit)

## 5. Partial-reduce hash aggregation

This implementation will not be planned by DataFusion SQL interface, it must be
manually constructed at [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) level.

This mode is useful in a distributed setting.

See [`PartialReduceHashAggregateStream`] for details.

## 6. Fallback grouped hash aggregation

[`GroupedHashAggregateStream`] is the legacy implementation for several of the
stream types above. It is being incrementally migrated to separate streams.

See the issue for details: <https://github.com/apache/datafusion/issues/22710>

Unresolved upstream links (retained, not inferred): ``AggregateStream``, ``GroupedHashAggregateStream``, ``FinalHashAggregateStream``, ``OrderedPartialAggregateStream``, ``SingleHashAggregateStream``, ``PartialHashAggregateStream``, ``OrderedFinalAggregateStream``, ``PartialReduceHashAggregateStream``, ``GroupedTopKAggregateStream``.
