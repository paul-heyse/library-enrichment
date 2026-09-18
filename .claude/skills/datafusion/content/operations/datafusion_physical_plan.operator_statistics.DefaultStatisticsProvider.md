# `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.DefaultStatisticsProvider.json).

<a id="op-5664b61a0d6c36c14eab5141"></a>
## DefaultStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct DefaultStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Default statistics provider that delegates to each operator's built-in
`partition_statistics` implementation.

<a id="op-11e50250a5dd6826587c9cf6"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, _child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider", "path": "DefaultStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [275, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc0262007f262418cf1b878f"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> DefaultStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider", "path": "DefaultStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 17], "end": [261, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e16804c56b239b7fa2845361"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::DefaultStatisticsProvider", "path": "DefaultStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 10], "end": [261, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
