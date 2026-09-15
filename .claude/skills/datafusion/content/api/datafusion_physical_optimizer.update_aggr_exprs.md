# `datafusion_physical_optimizer::update_aggr_exprs`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.update_aggr_exprs.json`](../model/datafusion_physical_optimizer.update_aggr_exprs.json)

## OptimizeAggregateOrder

`struct` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder`

```rust
struct OptimizeAggregateOrder
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

This optimizer rule checks ordering requirements of aggregate expressions.

There are 3 kinds of aggregators in terms of ordering requirements:
- `AggregateOrderSensitivity::Insensitive`, meaning that ordering is not
  important.
- `AggregateOrderSensitivity::HardRequirement`, meaning that the aggregator
  requires a specific ordering.
- `AggregateOrderSensitivity::Beneficial`, meaning that the aggregator can
  handle unordered input, but can run more efficiently if its input conforms
  to a specific ordering.

This rule analyzes aggregate expressions of type `Beneficial` to see whether
their input ordering requirements are satisfied. If this is the case, the
aggregators are modified to run in a more efficient mode.

---
