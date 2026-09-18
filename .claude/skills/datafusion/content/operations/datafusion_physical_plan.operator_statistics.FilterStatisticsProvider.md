# `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.FilterStatisticsProvider.json).

<a id="op-59066bd11e2065860e644852"></a>
## FilterStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct FilterStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics provider for [`FilterExec`](crate::filter::FilterExec) that uses
pre-computed enhanced child statistics from the registry walk.

Unlike the default provider (which calls `partition_statistics` and gets raw
child stats), this provider receives enhanced child stats that may include
NDV overrides injected at the scan level. It applies the same selectivity
estimation logic as `FilterExec::statistics_helper`, then additionally
adjusts each column's `distinct_count` using [`ndv_after_selectivity`](../operations/datafusion_physical_plan.operator_statistics.ndv_after_selectivity.md#op-020c90a37459d60ca0e9690a) based
on the computed selectivity ratio.

<a id="op-5519377303a13b6e07ecabd0"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::FilterStatisticsProvider", "path": "FilterStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [528, 1], "end": [573, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:529`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39a80201bfaab4146a07ed28"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> FilterStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::FilterStatisticsProvider", "path": "FilterStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 17], "end": [525, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e40de51add056bce303d0cb"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::FilterStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::FilterStatisticsProvider", "path": "FilterStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 10], "end": [525, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
