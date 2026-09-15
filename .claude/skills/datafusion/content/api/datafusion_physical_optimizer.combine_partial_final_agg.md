# `datafusion_physical_optimizer::combine_partial_final_agg`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.combine_partial_final_agg.json`](../model/datafusion_physical_optimizer.combine_partial_final_agg.json)

## CombinePartialFinalAggregate

`struct` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate`

```rust
struct CombinePartialFinalAggregate
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

CombinePartialFinalAggregate optimizer rule combines the adjacent Partial and Final AggregateExecs
into a Single AggregateExec if their grouping exprs and aggregate exprs equal.

This rule should be applied after the `EnsureRequirements` rule (which
handles both distribution and sorting enforcement).

---
