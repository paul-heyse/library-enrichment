# `datafusion_optimizer::filter_null_join_keys`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.filter_null_join_keys.json`](../model/datafusion_optimizer.filter_null_join_keys.json)

## FilterNullJoinKeys

`struct` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys`

```rust
struct FilterNullJoinKeys
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

The FilterNullJoinKeys rule will identify joins with equi-join conditions
where the join key is nullable and then insert an `IsNotNull` filter on the nullable side since null values
can never match.

---
