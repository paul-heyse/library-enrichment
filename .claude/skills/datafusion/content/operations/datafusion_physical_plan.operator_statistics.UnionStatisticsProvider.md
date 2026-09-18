# `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.UnionStatisticsProvider.json).

<a id="op-57284633242dfde6dceb8223"></a>
## UnionStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct UnionStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:928`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics provider for [`UnionExec`](crate::union::UnionExec).

Sums row counts across all inputs.

<a id="op-ab7137ab0c03642372f41b8f"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::UnionStatisticsProvider", "path": "UnionStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [930, 1], "end": [961, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:931`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75ba274772804b5e6a559244"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> UnionStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::UnionStatisticsProvider", "path": "UnionStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [927, 17], "end": [927, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:927`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43bee018fa872636a1fb3cdc"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::UnionStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::UnionStatisticsProvider", "path": "UnionStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [927, 10], "end": [927, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:927`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
