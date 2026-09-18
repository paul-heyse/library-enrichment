# `datafusion_physical_optimizer::topk_aggregation`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.topk_aggregation.json`](../model/datafusion_physical_optimizer.topk_aggregation.json)

## TopKAggregation

`struct` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation`

```rust
struct TopKAggregation
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

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.topk_aggregation.TopKAggregation.md).


An optimizer rule that passes a `limit` hint to aggregations if the whole result is not needed

---
