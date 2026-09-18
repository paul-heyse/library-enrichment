# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_partitioned_join_keys`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.reorder_partitioned_join_keys.json).

<a id="op-4a9759393aeed9917d854816"></a>
## reorder_partitioned_join_keys

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::reorder_partitioned_join_keys` · datafusion-physical-optimizer 55.1.0

```rust
fn reorder_partitioned_join_keys<F>(join_plan: PlanWithKeyRequirements, on: &[(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)], sort_options: &[arrow::compute::SortOptions], join_constructor: &F) -> datafusion_common::error::Result<PlanWithKeyRequirements> where F: Fn((Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>, Vec<arrow::compute::SortOptions>)) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/ensure_requirements/enforce_distribution.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
