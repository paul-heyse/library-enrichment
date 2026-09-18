# `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.ProjectionStatisticsProvider.json).

<a id="op-c2b09bcaeb037f1e11a4b411"></a>
## ProjectionStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct ProjectionStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics provider for [`ProjectionExec`](crate::projection::ProjectionExec)
that uses pre-computed enhanced child statistics from the registry walk.

Maps enhanced child column statistics to output columns based on the
projection expressions, preserving NDV and other statistics through
column references.

<a id="op-2b93ea81ad121dc10a375f49"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider", "path": "ProjectionStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [608, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04ddb11289df80091308af20"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> ProjectionStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider", "path": "ProjectionStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [581, 17], "end": [581, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d45c397f96117753e9c74c8d"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ProjectionStatisticsProvider", "path": "ProjectionStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [581, 10], "end": [581, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
