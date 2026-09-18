# `datafusion_optimizer::unions_to_filter`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.unions_to_filter.json`](../model/datafusion_optimizer.unions_to_filter.json)

## UnionsToFilter

`struct` · `datafusion_optimizer::unions_to_filter::UnionsToFilter`

```rust
struct UnionsToFilter
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.unions_to_filter.UnionsToFilter.md).


---
