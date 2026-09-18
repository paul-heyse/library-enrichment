# `datafusion_physical_plan::windows::bounded_window_agg_exec::WindowStateObserver`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.bounded_window_agg_exec.WindowStateObserver.json).

<a id="op-ee828df69445bcd4873293f4"></a>
## WindowStateObserver

`trait` · `datafusion_physical_plan::windows::bounded_window_agg_exec::WindowStateObserver` · datafusion-physical-plan 55.1.0

```rust
trait WindowStateObserver: Send + Sync
```

Source: `src/windows/bounded_window_agg_exec.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Callback receiver for per-partition window state.

`state` is the result of [`Accumulator::state`], which is a `&mut self`
call whose trait doc states "this function should not be called twice."
Several built-in aggregates (`median`, `percentile_cont`, `string_agg`,
`min_max_bytes`/`min_max_struct`) `std::mem::take` their internal
buffers to build that state — so `state` is a destructive read, not a
snapshot. The exec fires this at most once per group; a callee that
needs the value beyond the callback must retain it (e.g. clone into
owned storage).

[`Accumulator::state`]: datafusion_expr::Accumulator::state

<a id="op-a38fc2622c4d229cf0d9d06e"></a>
## finalize_window_aggregate

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::WindowStateObserver::finalize_window_aggregate` · datafusion-physical-plan 55.1.0

```rust
fn finalize_window_aggregate(&self, partition_idx: usize, window_expr: &Arc<dyn WindowExpr>, partition_key: &PartitionKey, state: Vec<ScalarValue>) -> Result<()>
```

Source: `src/windows/bounded_window_agg_exec.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Invoked once per (output-partition-index, window-expression,
PARTITION BY tuple) as each PARTITION BY group closes, for every
aggregate window expression on the exec. Non-aggregate window
functions (e.g. `row_number`, `rank`, `lead`/`lag`) do not fire this
callback.

# Arguments

* `partition_idx` - Output partition index of the [`BoundedWindowAggExec`](../operations/datafusion_physical_plan.windows.bounded_window_agg_exec.BoundedWindowAggExec.md#op-4f455d56121cc7d9007e9454)
  stream firing this callback.
* `window_expr` - The window expression whose state just closed.
* `partition_key` - The PARTITION BY tuple that just closed.
* `state` - [`Accumulator::state`] for the closed group of
  `window_expr`. See the trait-level doc for the destructive-read
  contract.

[`Accumulator::state`]: datafusion_expr::Accumulator::state
