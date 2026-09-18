# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::DistributionContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.DistributionContext.json).

<a id="op-fe24bd62f27dcde3fcf50e83"></a>
## DistributionContext

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::DistributionContext` · datafusion-physical-optimizer 55.1.0

```rust
type DistributionContext = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Source: `src/ensure_requirements/enforce_distribution.rs:1548`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Keeps track of distribution changing operators (like `RepartitionExec`,
`SortPreservingMergeExec`, `CoalescePartitionsExec`) and their ancestors.
Using this information, we can optimize distribution of the plan if/when
necessary.
