# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown`

Crate `datafusion-physical-optimizer` · 4 public items · structured records in [`model/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.json`](../model/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.json)

## assign_initial_requirements

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::assign_initial_requirements`

```rust
fn assign_initial_requirements(sort_push_down: &mut SortPushDown)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.assign_initial_requirements.md).


Assigns the ordering requirement of the root node to the its children.

---

## pushdown_sorts

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::pushdown_sorts`

```rust
fn pushdown_sorts(sort_push_down: SortPushDown) -> datafusion_common::Result<SortPushDown>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.pushdown_sorts.md).


Tries to push down the sort requirements as far as possible, if decides a `SortExec` is unnecessary removes it.

---

## ParentRequirements

`struct` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements`

```rust
struct ParentRequirements
```

**Derives**: Clone, Debug, Default

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.ParentRequirements.md).


"Data class" used by sort pushdown (now driven from `EnsureRequirements`)
to push down [`SortExec`] in the plan. In some cases the total
computational cost is reduced by pushing down `SortExec`s through certain
executors. The object carries the parent required ordering, the (optional)
`fetch` value of the parent node, and the parent's distribution requirement
(used by the distribution-aware pushdown path) as its data.

---

## SortPushDown

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::SortPushDown`

```rust
type SortPushDown = datafusion_physical_plan::tree_node::PlanContext<ParentRequirements>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.SortPushDown.md).


---
