# `datafusion_physical_expr::window::standard`

Crate `datafusion-physical-expr` · 1 public items · structured records in [`model/datafusion_physical_expr.window.standard.json`](../model/datafusion_physical_expr.window.standard.json)

## StandardWindowExpr

`struct` · `datafusion_physical_expr::window::standard::StandardWindowExpr`

Also reachable as `datafusion_physical_expr::window::StandardWindowExpr`, `datafusion_physical_plan::windows::StandardWindowExpr`

```rust
struct StandardWindowExpr
```

**Implements**: `datafusion_physical_expr::window::window_expr::WindowExpr`

**Derives**: Debug

**Methods** (3)

```rust
fn add_equal_orderings(&self, eq_properties: &mut EquivalenceProperties) -> Result<()>
fn get_standard_func_expr(&self) -> &Arc<dyn StandardWindowFunctionExpr>
fn new(expr: Arc<dyn StandardWindowFunctionExpr>, partition_by: &[Arc<dyn PhysicalExpr>], order_by: &[PhysicalSortExpr], window_frame: Arc<WindowFrame>) -> Self
```

**via `datafusion_physical_expr::window::window_expr::WindowExpr`**

```rust
fn as_any(&self) -> &dyn Any
fn create_window_fn(&self) -> Result<WindowFn>
fn evaluate(&self, batch: &RecordBatch) -> Result<ArrayRef>
fn evaluate_stateful(&self, partition_batches: &PartitionBatches, window_agg_state: &mut PartitionWindowAggStates, _eval_ctx: &WindowEvalContext<'_>) -> Result<()>
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self) -> Result<FieldRef>
fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>
fn get_window_frame(&self) -> &Arc<WindowFrame>
fn name(&self) -> &str
fn order_by(&self) -> &[PhysicalSortExpr]
fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>]
fn uses_bounded_memory(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.standard.StandardWindowExpr.md).


A window expr that takes the form of a [`StandardWindowFunctionExpr`].

---
