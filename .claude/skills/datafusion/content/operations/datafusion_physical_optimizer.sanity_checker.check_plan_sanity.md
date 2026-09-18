# `datafusion_physical_optimizer::sanity_checker::check_plan_sanity`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.sanity_checker.check_plan_sanity.json).

<a id="op-2dbe2fc3c17062f45e0a4a63"></a>
## check_plan_sanity

`function` · `datafusion_physical_optimizer::sanity_checker::check_plan_sanity` · datafusion-physical-optimizer 55.1.0

```rust
fn check_plan_sanity(plan: &std::sync::Arc<dyn ExecutionPlan>, optimizer_options: &datafusion_common::config::OptimizerOptions) -> datafusion_common::Result<()>
```

Source: `src/sanity_checker.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Ensures that the plan is pipeline friendly and the order and
distribution requirements from its children are satisfied.
