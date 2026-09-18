# `datafusion_physical_plan::windows::get_window_mode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.get_window_mode.json).

<a id="op-10d08891b46de7b10e46256d"></a>
## get_window_mode

`function` · `datafusion_physical_plan::windows::get_window_mode` · datafusion-physical-plan 55.1.0

```rust
fn get_window_mode(partitionby_exprs: &[std::sync::Arc<dyn PhysicalExpr>], orderby_keys: &[expressions::PhysicalSortExpr], input: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<Option<(bool, InputOrderMode)>>
```

Source: `src/windows/mod.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Compares physical ordering (output ordering of the `input` operator) with
`partitionby_exprs` and `orderby_keys` to decide whether existing ordering
is sufficient to run the current window operator.
- A `None` return value indicates that we can not remove the sort in question
  (input ordering is not sufficient to run current window executor).
- A `Some((bool, InputOrderMode))` value indicates that the window operator
  can run with existing input ordering, so we can remove `SortExec` before it.

The `bool` field in the return value represents whether we should reverse window
operator to remove `SortExec` before it. The `InputOrderMode` field represents
the mode this window operator should work in to accommodate the existing ordering.
