# `datafusion_physical_optimizer::projection_pushdown`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.projection_pushdown.json`](../model/datafusion_physical_optimizer.projection_pushdown.json)

## ProjectionPushdown

`struct` · `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown`

```rust
struct ProjectionPushdown
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

This rule inspects `ProjectionExec`'s in the given physical plan and tries to
remove or swap with its child.

Furthermore, tries to push down projections from nested loop join filters that only depend on
one side of the join. By pushing these projections down, functions that only depend on one side
of the join must be evaluated for the cartesian product of the two sides.

---
