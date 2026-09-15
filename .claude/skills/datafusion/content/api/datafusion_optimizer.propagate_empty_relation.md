# `datafusion_optimizer::propagate_empty_relation`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.propagate_empty_relation.json`](../model/datafusion_optimizer.propagate_empty_relation.json)

## PropagateEmptyRelation

`struct` · `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation`

```rust
struct PropagateEmptyRelation
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

Optimization rule that bottom-up to eliminate plan by propagating empty_relation.

---
