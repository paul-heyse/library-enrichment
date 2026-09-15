# `datafusion_physical_optimizer::pushdown_sort`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.pushdown_sort.json`](../model/datafusion_physical_optimizer.pushdown_sort.json)

## PushdownSort

`struct` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort`

```rust
struct PushdownSort
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

A PhysicalOptimizerRule that attempts to push down sort requirements to data sources.

See module-level documentation for details.

---
