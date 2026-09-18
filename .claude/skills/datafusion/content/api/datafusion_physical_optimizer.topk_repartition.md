# `datafusion_physical_optimizer::topk_repartition`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.topk_repartition.json`](../model/datafusion_physical_optimizer.topk_repartition.json)

## TopKRepartition

`struct` · `datafusion_physical_optimizer::topk_repartition::TopKRepartition`

```rust
struct TopKRepartition
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Clone, Debug, Default

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

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.topk_repartition.TopKRepartition.md).


A physical optimizer rule that pushes TopK (Sort with fetch) past
hash repartition when the partition key is a prefix of the sort key.

See module-level documentation for details.

---
