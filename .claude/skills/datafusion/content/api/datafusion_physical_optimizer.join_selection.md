# `datafusion_physical_optimizer::join_selection`

Crate `datafusion-physical-optimizer` · 3 public items · structured records in [`model/datafusion_physical_optimizer.join_selection.json`](../model/datafusion_physical_optimizer.join_selection.json)

## hash_join_swap_subrule

`function` · `datafusion_physical_optimizer::join_selection::hash_join_swap_subrule`

```rust
fn hash_join_swap_subrule(input: std::sync::Arc<dyn ExecutionPlan>, _config_options: &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.join_selection.hash_join_swap_subrule.md).


This subrule will swap build/probe sides of a hash join depending on whether
one of its inputs may produce an infinite stream of records. The rule ensures
that the left (build) side of the hash join always operates on an input stream
that will produce a finite set of records. If the left side can not be chosen
to be "finite", the join sides stay the same as the original query.
```text
For example, this rule makes the following transformation:



          +--------------+              +--------------+
          |              |  unbounded   |              |
   Left   | Infinite     |    true      | Hash         |\true
          | Data source  |--------------| Repartition  | \   +--------------+       +--------------+
          |              |              |              |  \  |              |       |              |
          +--------------+              +--------------+   - |  Hash Join   |-------| Projection   |
                                                           - |              |       |              |
          +--------------+              +--------------+  /  +--------------+       +--------------+
          |              |  unbounded   |              | /
   Right  | Finite       |    false     | Hash         |/false
          | Data Source  |--------------| Repartition  |
          |              |              |              |
          +--------------+              +--------------+



          +--------------+              +--------------+
          |              |  unbounded   |              |
   Left   | Finite       |    false     | Hash         |\false
          | Data source  |--------------| Repartition  | \   +--------------+       +--------------+
          |              |              |              |  \  |              | true  |              | true
          +--------------+              +--------------+   - |  Hash Join   |-------| Projection   |-----
                                                           - |              |       |              |
          +--------------+              +--------------+  /  +--------------+       +--------------+
          |              |  unbounded   |              | /
   Right  | Infinite     |    true      | Hash         |/true
          | Data Source  |--------------| Repartition  |
          |              |              |              |
          +--------------+              +--------------+
```

---

## JoinSelection

`struct` · `datafusion_physical_optimizer::join_selection::JoinSelection`

```rust
struct JoinSelection
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
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.join_selection.JoinSelection.md).


The [`JoinSelection`] rule tries to modify a given plan so that it can
accommodate infinite sources and optimize joins in the plan according to
available statistical information, if there is any.

---

## PipelineFixerSubrule

`type_alias` · `datafusion_physical_optimizer::join_selection::PipelineFixerSubrule`

```rust
type PipelineFixerSubrule = dyn Fn(std::sync::Arc<dyn ExecutionPlan>, &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.join_selection.PipelineFixerSubrule.md).


Pipeline-fixing join selection subrule.

---
