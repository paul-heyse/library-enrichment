# `datafusion_physical_optimizer::sanity_checker`

Crate `datafusion-physical-optimizer` · 3 public items · structured records in [`model/datafusion_physical_optimizer.sanity_checker.json`](../model/datafusion_physical_optimizer.sanity_checker.json)

## check_finiteness_requirements

`function` · `datafusion_physical_optimizer::sanity_checker::check_finiteness_requirements`

```rust
fn check_finiteness_requirements(input: &dyn ExecutionPlan, optimizer_options: &datafusion_common::config::OptimizerOptions) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.sanity_checker.check_finiteness_requirements.md).


This function propagates finiteness information and rejects any plan with
pipeline-breaking operators acting on infinite inputs.

---

## check_plan_sanity

`function` · `datafusion_physical_optimizer::sanity_checker::check_plan_sanity`

```rust
fn check_plan_sanity(plan: &std::sync::Arc<dyn ExecutionPlan>, optimizer_options: &datafusion_common::config::OptimizerOptions) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.sanity_checker.check_plan_sanity.md).


Ensures that the plan is pipeline friendly and the order and
distribution requirements from its children are satisfied.

---

## SanityCheckPlan

`struct` · `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan`

```rust
struct SanityCheckPlan
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
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.sanity_checker.SanityCheckPlan.md).


The SanityCheckPlan rule rejects the following query plans:
1. Invalid plans containing nodes whose order and/or distribution requirements
   are not satisfied by their children.
2. Plans that use pipeline-breaking operators on infinite input(s),
   it is impossible to execute such queries (they will never generate output nor finish)

---
