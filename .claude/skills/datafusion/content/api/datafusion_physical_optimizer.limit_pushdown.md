# `datafusion_physical_optimizer::limit_pushdown`

Crate `datafusion-physical-optimizer` · 3 public items · structured records in [`model/datafusion_physical_optimizer.limit_pushdown.json`](../model/datafusion_physical_optimizer.limit_pushdown.json)

## pushdown_limit_helper

`function` · `datafusion_physical_optimizer::limit_pushdown::pushdown_limit_helper`

```rust
fn pushdown_limit_helper(pushdown_plan: std::sync::Arc<dyn ExecutionPlan>, global_state: GlobalRequirements) -> datafusion_common::error::Result<(datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>, GlobalRequirements)>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.limit_pushdown.pushdown_limit_helper.md).


This function is the main helper function of the `LimitPushDown` rule.
The helper takes an `ExecutionPlan` and a global (algorithm) state which is
an instance of `GlobalRequirements` and modifies these parameters while
checking if the limits can be pushed down or not.

If a limit is encountered, a [`TreeNodeRecursion::Stop`] is returned. Otherwise,
return a [`TreeNodeRecursion::Continue`].

---

## GlobalRequirements

`struct` · `datafusion_physical_optimizer::limit_pushdown::GlobalRequirements`

```rust
struct GlobalRequirements
```

**Derives**: Clone, Debug, Default

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.limit_pushdown.GlobalRequirements.md).


This is a "data class" we use within the [`LimitPushdown`] rule to push
down limits in the plan. GlobalRequirements are hold as a rule-wide state
and holds the fetch and skip information. The struct also has a field named
satisfied which means if the "current" plan is valid in terms of limits or not.

For example: If the plan is satisfied with current fetch info, we decide to not add a LocalLimit

[`LimitPushdown`]: crate::limit_pushdown::LimitPushdown

---

## LimitPushdown

`struct` · `datafusion_physical_optimizer::limit_pushdown::LimitPushdown`

```rust
struct LimitPushdown
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.limit_pushdown.LimitPushdown.md).


This rule inspects [`ExecutionPlan`]'s and pushes down the fetch limit from
the parent to the child if applicable.

---
