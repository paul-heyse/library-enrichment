# `datafusion_optimizer::utils::log_plan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.utils.log_plan.json).

<a id="op-3004880b1f2b233f2a5c4e68"></a>
## log_plan

`function` · `datafusion_optimizer::utils::log_plan` · datafusion-optimizer 55.1.0

```rust
fn log_plan(description: &str, plan: &datafusion_expr::logical_plan::LogicalPlan)
```

Source: `src/utils.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Log the plan in debug/tracing mode after some part of the optimizer runs
