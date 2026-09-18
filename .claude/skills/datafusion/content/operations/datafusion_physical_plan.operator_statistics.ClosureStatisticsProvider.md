# `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.ClosureStatisticsProvider.json).

<a id="op-449c59e5780d9a76484bd83b"></a>
## ClosureStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct ClosureStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:992`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A [`StatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.StatisticsProvider.md#op-b3050eb3d6b996dd9e336fb5) backed by a user-supplied closure.

Useful for injecting custom statistics in tests or for cardinality feedback
pipelines where real runtime statistics need to override plan estimates.
The closure receives the current plan node and its children's enhanced
statistics, returning a [`StatisticsResult`](../operations/datafusion_physical_plan.operator_statistics.StatisticsResult.md#op-e76cdd29b31ac6aaf7341110).

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

<a id="op-627ba0b377cbcf6b5f9b3047"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider", "path": "ClosureStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 1], "end": [1022, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:1015`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e04338e590a6eb991d8e1aa"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider", "path": "ClosureStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1008, 1], "end": [1012, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:1009`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbc865337dd69c8367e86357"></a>
## new

`function` · `datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider::new` · datafusion-physical-plan 55.1.0

```rust
fn new(f: impl Fn(&dyn ExecutionPlan, &[ExtendedStatistics]) -> Result<StatisticsResult> + Send + Sync + 'static) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ClosureStatisticsProvider", "path": "ClosureStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [996, 1], "end": [1006, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:998`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new provider from a closure.
