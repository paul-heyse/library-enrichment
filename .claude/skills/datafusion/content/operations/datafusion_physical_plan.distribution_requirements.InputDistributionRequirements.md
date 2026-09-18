# `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.distribution_requirements.InputDistributionRequirements.json).

<a id="op-e1bc2ab71ce298f90cee462e"></a>
## InputDistributionRequirements

`struct` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements` · datafusion-physical-plan 55.1.0

```rust
struct InputDistributionRequirements
```

Source: `src/distribution_requirements.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Distribution requirements for an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673)'s inputs.

[`InputDistributionRequirements`](../operations/datafusion_physical_plan.distribution_requirements.InputDistributionRequirements.md#op-e1bc2ab71ce298f90cee462e) describes what distribution an operator
requires from each child.

- [`Self::new`](../operations/datafusion_physical_plan.distribution_requirements.InputDistributionRequirements.md#op-4e9281fb04d244913e5c2d8f) describes independent per-child requirements.
- [`Self::co_partitioned`](../operations/datafusion_physical_plan.distribution_requirements.InputDistributionRequirements.md#op-8a12ab3d2332a0e1e62db7d1) additionally requires child partitions with the
  same index to cover compatible key ranges.

For a single-input aggregate:

```text
AggregateExec
  child 0 requirement: KeyPartitioned(group_exprs)
```

each input partition can aggregate its own key domain independently.

For a partitioned join:

```text
HashJoinExec
  child 0 requirement: KeyPartitioned(left_keys)
  child 1 requirement: KeyPartitioned(right_keys)

  partition 0: join(left partition 0, right partition 0)
  partition 1: join(left partition 1, right partition 1)
  partition 2: join(left partition 2, right partition 2)
```

each child must satisfy its own key requirement. In addition, matching
partition indexes must be safe to process together.

<a id="op-00f656588ede0bf98fccd160"></a>
## child_distribution

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::child_distribution` · datafusion-physical-plan 55.1.0

```rust
fn child_distribution(&self, child_idx: usize) -> Option<&Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [304, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the distribution requirement for a child.

<a id="op-64b0d2f6bde5d43d3fb0a39d"></a>
## child_satisfaction

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::child_satisfaction` · datafusion-physical-plan 55.1.0

```rust
fn child_satisfaction(&self, child_idx: usize, child: &dyn ExecutionPlan, options: ChildSatisfactionOptions) -> Result<PartitioningSatisfaction>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [304, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns how a child satisfies its distribution requirement.

This preserves the requirement set's satisfaction policy.

<a id="op-82882a934d5252364bc6eb11"></a>
## clone

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 17], "end": [58, 22], "filename": "src/distribution_requirements.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/distribution_requirements.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a12ab3d2332a0e1e62db7d1"></a>
## co_partitioned

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::co_partitioned` · datafusion-physical-plan 55.1.0

```rust
fn co_partitioned(per_child: Vec<Distribution>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [304, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a requirement that all children are co-partitioned.

Each child must satisfy its own [`Distribution`](../operations/datafusion_physical_expr.partitioning.Distribution.md#op-1570134c2bb176dfcdc9c836). Matching partition
indexes are processed together:

```text
left:  Range(left.a ASC,  split_points=[10, 20])
right: Range(right.x ASC, split_points=[10, 20])

partition 0 from both sides contains keys before 10
partition 1 from both sides contains keys in [10, 20)
partition 2 from both sides contains keys at/after 20
```

If the split points differ, partition `i` from one side no longer covers
the same key range as partition `i` from the other side.

<a id="op-143fe409288b22324b6b7144"></a>
## fmt

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/distribution_requirements.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/distribution_requirements.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5f33e590a3de3fee4398e22"></a>
## into_per_child

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::into_per_child` · datafusion-physical-plan 55.1.0

```rust
fn into_per_child(self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [304, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the per-child distribution requirements.

WARNING: This intentionally drops any grouped relationship.

<a id="op-4e9281fb04d244913e5c2d8f"></a>
## new

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::new` · datafusion-physical-plan 55.1.0

```rust
fn new(per_child: Vec<Distribution>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [304, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create independent per-child requirements.

<a id="op-e132087858320f7115ae7ac1"></a>
## per_child_distributions

`function` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements::per_child_distributions` · datafusion-physical-plan 55.1.0

```rust
fn per_child_distributions(&self) -> impl ExactSizeIterator<Item = &Distribution> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::InputDistributionRequirements", "path": "InputDistributionRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [304, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the per-child distribution requirements.
