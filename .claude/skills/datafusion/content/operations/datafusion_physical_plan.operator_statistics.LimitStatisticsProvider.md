# `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.LimitStatisticsProvider.json).

<a id="op-146faf3a043b4d34b5169fa8"></a>
## LimitStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct LimitStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:880`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics provider for [`LocalLimitExec`](crate::limit::LocalLimitExec) and
[`GlobalLimitExec`](crate::limit::GlobalLimitExec).

Caps output row count at the limit value, accounting for any leading skip offset
in `GlobalLimitExec`.

<a id="op-68cd1ad42e54ba1640386f58"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::LimitStatisticsProvider", "path": "LimitStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [922, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:883`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b48558e772882db39e63f47"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> LimitStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::LimitStatisticsProvider", "path": "LimitStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 17], "end": [879, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998b52adb7551caba2a008fe"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::LimitStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::LimitStatisticsProvider", "path": "LimitStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 10], "end": [879, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:879`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
