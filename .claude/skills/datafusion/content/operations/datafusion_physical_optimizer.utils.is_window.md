# `datafusion_physical_optimizer::utils::is_window`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.is_window.json).

<a id="op-cc0c1efefa70719aa2b64ae4"></a>
## is_window

`function` · `datafusion_physical_optimizer::utils::is_window` · datafusion-physical-optimizer 55.1.0

```rust
fn is_window(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool
```

Source: `src/utils.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Checks whether the given operator is a window;
i.e. either a [`WindowAggExec`](../operations/datafusion_physical_plan.windows.window_agg_exec.WindowAggExec.md#op-77190b45ce73475e081f8d1d) or a [`BoundedWindowAggExec`](../operations/datafusion_physical_plan.windows.bounded_window_agg_exec.BoundedWindowAggExec.md#op-4f455d56121cc7d9007e9454).
