# `datafusion_physical_plan::statistics::ChildStats`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.statistics.ChildStats.json).

<a id="op-99dec09dc125c07b5a5f515c"></a>
## ChildStats

`enum` · `datafusion_physical_plan::statistics::ChildStats` · datafusion-physical-plan 55.1.0

```rust
enum ChildStats
```

Source: `src/statistics.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Directive returned by [`ExecutionPlan::child_stats_requests`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-9b5f78775acc13c2573e8ec1) describing
how the [`StatisticsContext`](../operations/datafusion_physical_plan.statistics.StatisticsContext.md#op-37580381f9423ac5020b296f) should obtain each child's statistics.

<a id="op-d2332e33afc920d370c1f6cc"></a>
## At

`variant` · `datafusion_physical_plan::statistics::ChildStats::At` · datafusion-physical-plan 55.1.0

```rust
At
```

Source: `src/statistics.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Compute the child's statistics at this partition (`None` = overall).

<a id="op-609e8a7a86a094c171eeea8c"></a>
## Skip

`variant` · `datafusion_physical_plan::statistics::ChildStats::Skip` · datafusion-physical-plan 55.1.0

```rust
Skip
```

Source: `src/statistics.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Skip this child; the parent does not need its statistics. A placeholder
[`Statistics::new_unknown`] is supplied in its slot.

Unresolved upstream links (retained, not inferred): ``Statistics::new_unknown``.

<a id="op-17700c0f6e63c5be071068df"></a>
## clone

`function` · `datafusion_physical_plan::statistics::ChildStats::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ChildStats
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::ChildStats", "path": "ChildStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 17], "end": [110, 22], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-517d4b62aa15c4ad18af2b58"></a>
## eq

`function` · `datafusion_physical_plan::statistics::ChildStats::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &ChildStats) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::ChildStats", "path": "ChildStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 30], "end": [110, 39], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/statistics.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d4cd82c66910dc540409609"></a>
## fmt

`function` · `datafusion_physical_plan::statistics::ChildStats::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::ChildStats", "path": "ChildStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 10], "end": [110, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
