# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::PlanWithKeyRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.PlanWithKeyRequirements.json).

<a id="op-f30c30156ac28c8de9700efe"></a>
## PlanWithKeyRequirements

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::PlanWithKeyRequirements` · datafusion-physical-optimizer 55.1.0

```rust
type PlanWithKeyRequirements = datafusion_physical_plan::tree_node::PlanContext<Vec<std::sync::Arc<dyn PhysicalExpr>>>
```

Source: `src/ensure_requirements/enforce_distribution.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Keeps track of parent required key orderings.
