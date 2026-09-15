# `datafusion_physical_plan::distribution_requirements`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.distribution_requirements.json`](../model/datafusion_physical_plan.distribution_requirements.json)

## ChildSatisfactionOptions

`struct` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions`

Also reachable as `datafusion::physical_plan::ChildSatisfactionOptions`, `datafusion_physical_plan::ChildSatisfactionOptions`

```rust
struct ChildSatisfactionOptions
```

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn allow_subset(&self) -> bool
fn new() -> Self
fn with_allow_subset(self, allow_subset: bool) -> Self
```

Options for checking child distribution satisfaction.

---

## InputDistributionRequirements

`struct` · `datafusion_physical_plan::distribution_requirements::InputDistributionRequirements`

Also reachable as `datafusion::physical_plan::InputDistributionRequirements`, `datafusion_physical_plan::InputDistributionRequirements`

```rust
struct InputDistributionRequirements
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn child_distribution(&self, child_idx: usize) -> Option<&Distribution>
fn child_satisfaction(&self, child_idx: usize, child: &dyn ExecutionPlan, options: ChildSatisfactionOptions) -> Result<PartitioningSatisfaction>
fn co_partitioned(per_child: Vec<Distribution>) -> Self
fn into_per_child(self) -> Vec<Distribution>
fn new(per_child: Vec<Distribution>) -> Self
fn per_child_distributions(&self) -> impl ExactSizeIterator<Item = &Distribution> + '_
```

Distribution requirements for an [`ExecutionPlan`]'s inputs.

[`InputDistributionRequirements`] describes what distribution an operator
requires from each child.

- [`Self::new`] describes independent per-child requirements.
- [`Self::co_partitioned`] additionally requires child partitions with the
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

---
