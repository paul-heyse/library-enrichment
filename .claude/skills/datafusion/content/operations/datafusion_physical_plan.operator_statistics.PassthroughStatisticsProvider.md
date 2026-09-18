# `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.PassthroughStatisticsProvider.json).

<a id="op-644756eff17c7a9b7fd6c3a5"></a>
## PassthroughStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct PassthroughStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics provider for single-input operators with
[`CardinalityEffect::Equal`](crate::execution_plan::CardinalityEffect::Equal).

These operators (Sort, Repartition, CoalescePartitions, etc.) don't
transform statistics, so we pass through the enhanced child stats directly.
This avoids the fallback calling `partition_statistics(None)` which would
trigger a redundant internal recursion with raw (non-enhanced) stats.

<a id="op-21c836c754b9539b9999889b"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider", "path": "PassthroughStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [645, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77c4ad6a84f18a9ea1748514"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> PassthroughStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider", "path": "PassthroughStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [617, 17], "end": [617, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:617`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bd450d2f2adc3ac24a569bf"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::PassthroughStatisticsProvider", "path": "PassthroughStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [617, 10], "end": [617, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:617`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
