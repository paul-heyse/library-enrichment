# `datafusion_physical_optimizer::limited_distinct_aggregation`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.limited_distinct_aggregation.json`](../model/datafusion_physical_optimizer.limited_distinct_aggregation.json)

## LimitedDistinctAggregation

`struct` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation`

```rust
struct LimitedDistinctAggregation
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

An optimizer rule that passes a `limit` hint into grouped aggregations which don't require all
rows in the group to be processed for correctness. Example queries fitting this description are:
- `SELECT distinct l_orderkey FROM lineitem LIMIT 10;`
- `SELECT l_orderkey FROM lineitem GROUP BY l_orderkey LIMIT 10;`

---
