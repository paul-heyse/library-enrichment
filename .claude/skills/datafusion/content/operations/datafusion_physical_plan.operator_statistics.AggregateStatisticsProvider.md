# `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.AggregateStatisticsProvider.json).

<a id="op-d28aa37ea36e0bf183ef455b"></a>
## AggregateStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct AggregateStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

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

<a id="op-65eca8051ae62aa69f6c3f89"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider", "path": "AggregateStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [668, 1], "end": [737, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:669`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4728f6be1619c6bf9527a645"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> AggregateStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider", "path": "AggregateStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [665, 17], "end": [665, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:665`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e17d8e4468d2f0ad6bb462c"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::AggregateStatisticsProvider", "path": "AggregateStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [665, 10], "end": [665, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:665`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
