# `datafusion_physical_expr::window::aggregate`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.window.aggregate.json`](../model/datafusion_physical_expr.window.aggregate.json)

## PlainAggregateWindowExpr

`struct` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr`

Also reachable as `datafusion_physical_expr::window::PlainAggregateWindowExpr`, `datafusion_physical_plan::windows::PlainAggregateWindowExpr`

```rust
struct PlainAggregateWindowExpr
```

**Implements**: `datafusion_physical_expr::window::window_expr::WindowExpr`

**Derives**: Debug

**Methods** (3)

```rust
fn add_equal_orderings(&self, eq_properties: &mut EquivalenceProperties, window_expr_index: usize) -> Result<()>
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
```

A window expr that takes the form of an aggregate function.

See comments on [`WindowExpr`] for more details.

---
