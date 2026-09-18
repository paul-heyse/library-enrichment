# `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.JoinStatisticsProvider.json).

<a id="op-192cd9d26ef10e0d08f3f4ef"></a>
## JoinStatisticsProvider

`struct` · `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider` · datafusion-physical-plan 55.1.0

```rust
struct JoinStatisticsProvider
```

Source: `src/operator_statistics/mod.rs:758`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics provider for equi-joins (hash join, sort-merge join) and cross joins.

For equi-joins, estimates output cardinality as
`left_rows * right_rows / product(max(left_ndv_i, right_ndv_i))`
across all join key columns (assuming independence between keys),
falling back to the Cartesian product when any key lacks NDV on both sides.
For cross joins, uses the exact Cartesian product.

The base inner-join estimate is then adjusted for the join type:
- Semi joins: capped at the preserved-side row count
- Anti joins: preserved-side minus matched rows (clamped to 0)
- Left/Right outer: at least as many rows as the preserved side
- Full outer: at least `left + right - inner_estimate`
- Left mark: exactly `left_rows` (one output row per left row)

Delegates when:
- The plan is not a supported join type
- Either input lacks row count information

<a id="op-41efd8a7395f10bb8261f49a"></a>
## compute_statistics

`function` · `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider::compute_statistics` · datafusion-physical-plan 55.1.0

```rust
fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics]) -> Result<StatisticsResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::JoinStatisticsProvider", "path": "JoinStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [760, 1], "end": [872, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsProvider", "path": "StatisticsProvider"}, "trait_path": "datafusion_physical_plan::operator_statistics::StatisticsProvider"}`

Source: `src/operator_statistics/mod.rs:761`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26ad5c9d3375220079985337"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> JoinStatisticsProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::JoinStatisticsProvider", "path": "JoinStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [757, 17], "end": [757, 24], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:757`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e655053c92d9bf01d33549c"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::JoinStatisticsProvider::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::JoinStatisticsProvider", "path": "JoinStatisticsProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [757, 10], "end": [757, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:757`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
