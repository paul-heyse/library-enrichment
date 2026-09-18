# `datafusion_physical_plan::windows::get_best_fitting_window`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.get_best_fitting_window.json).

<a id="op-5efd65d76291bf441c43fa29"></a>
## get_best_fitting_window

`function` · `datafusion_physical_plan::windows::get_best_fitting_window` · datafusion-physical-plan 55.1.0

```rust
fn get_best_fitting_window(window_exprs: &[std::sync::Arc<dyn WindowExpr>], input: &std::sync::Arc<dyn ExecutionPlan>, physical_partition_keys: &[std::sync::Arc<dyn PhysicalExpr>], state_observer: Option<std::sync::Arc<dyn WindowStateObserver>>) -> datafusion_common::Result<Option<std::sync::Arc<dyn ExecutionPlan>>>
```

Source: `src/windows/mod.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Constructs the best-fitting windowing operator (a `WindowAggExec` or a
`BoundedWindowExec`) for the given `input` according to the specifications
of `window_exprs` and `physical_partition_keys`. Here, best-fitting means
not requiring additional sorting and/or partitioning for the given input.
- A return value of `None` represents that there is no way to construct a
  windowing operator that doesn't need additional sorting/partitioning for
  the given input. Existing ordering should be changed to run the given
  windowing operation.
- A `Some(window exec)` value contains the optimal windowing operator (a
  `WindowAggExec` or a `BoundedWindowExec`) for the given input.
