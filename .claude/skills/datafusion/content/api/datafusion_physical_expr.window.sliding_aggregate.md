# `datafusion_physical_expr::window::sliding_aggregate`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.window.sliding_aggregate.json`](../model/datafusion_physical_expr.window.sliding_aggregate.json)

## SlidingAggregateWindowExpr

`struct` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr`

Also reachable as `datafusion_physical_expr::window::SlidingAggregateWindowExpr`

```rust
struct SlidingAggregateWindowExpr
```

**Implements**: `datafusion_physical_expr::window::window_expr::WindowExpr`

**Derives**: Debug

**Methods** (2)

```rust
fn get_aggregate_expr(&self) -> &AggregateFunctionExpr
fn new(aggregate: Arc<AggregateFunctionExpr>, partition_by: &[Arc<dyn PhysicalExpr>], order_by: &[PhysicalSortExpr], window_frame: Arc<WindowFrame>, filter: Option<Arc<dyn PhysicalExpr>>) -> Self
```

**via `datafusion_physical_expr::window::window_expr::WindowExpr`**

```rust
fn as_any(&self) -> &dyn Any
fn create_window_fn(&self) -> Result<WindowFn>
fn evaluate(&self, batch: &RecordBatch) -> Result<ArrayRef>
fn evaluate_stateful(&self, partition_batches: &PartitionBatches, window_agg_state: &mut PartitionWindowAggStates, eval_ctx: &WindowEvalContext<'_>) -> Result<()>
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self) -> Result<FieldRef>
fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>
fn get_window_frame(&self) -> &Arc<WindowFrame>
fn name(&self) -> &str
fn order_by(&self) -> &[PhysicalSortExpr]
fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>]
fn uses_bounded_memory(&self) -> bool
fn with_new_expressions(&self, args: Vec<Arc<dyn PhysicalExpr>>, partition_bys: Vec<Arc<dyn PhysicalExpr>>, order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<Arc<dyn WindowExpr>>
```

A window expr that takes the form of an aggregate function that
can be incrementally computed over sliding windows.

See comments on [`WindowExpr`] for more details.

---
