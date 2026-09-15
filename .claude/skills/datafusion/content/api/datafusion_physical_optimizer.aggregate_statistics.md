# `datafusion_physical_optimizer::aggregate_statistics`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.aggregate_statistics.json`](../model/datafusion_physical_optimizer.aggregate_statistics.json)

## AggregateStatistics

`struct` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics`

```rust
struct AggregateStatistics
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

Optimizer that uses available statistics for aggregate functions

---
