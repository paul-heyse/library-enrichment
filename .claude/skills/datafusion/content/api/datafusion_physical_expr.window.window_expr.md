# `datafusion_physical_expr::window::window_expr`

Crate `datafusion-physical-expr` · 8 public items · structured records in [`model/datafusion_physical_expr.window.window_expr.json`](../model/datafusion_physical_expr.window.window_expr.json)

## WindowFn

`enum` · `datafusion_physical_expr::window::window_expr::WindowFn`

```rust
enum WindowFn
```

**Variants**: `Builtin`, `Aggregate`

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.WindowFn.md).


---

## WindowEvalContext

`struct` · `datafusion_physical_expr::window::window_expr::WindowEvalContext`

Also reachable as `datafusion_physical_expr::window::WindowEvalContext`

```rust
struct WindowEvalContext<'a>
```

**Fields**: `most_recent_row`

**Derives**: Clone, Copy, Debug, Default

**Methods** (1)

```rust
fn with_most_recent_row(self, batch: Option<&'a RecordBatch>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.WindowEvalContext.md).


Stream-level context passed to [`WindowExpr::evaluate_stateful`].

This carries information that spans all partitions of the input, as
opposed to the per-partition state in [`PartitionBatches`] and
[`PartitionWindowAggStates`]. It is `non_exhaustive` so that fields can
be added without breaking implementors; construct it with
[`Default::default`] and the `with_*` builder methods.

---

## WindowPhysicalExpressions

`struct` · `datafusion_physical_expr::window::window_expr::WindowPhysicalExpressions`

```rust
struct WindowPhysicalExpressions
```

**Fields**: `args`, `partition_by_exprs`, `order_by_exprs`

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.WindowPhysicalExpressions.md).


Stores the physical expressions used inside the `WindowExpr`.

---

## WindowState

`struct` · `datafusion_physical_expr::window::window_expr::WindowState`

Also reachable as `datafusion_physical_expr::window::WindowState`

```rust
struct WindowState
```

**Fields**: `state`, `window_fn`, `published`

**Derives**: Debug

**Methods** (1)

```rust
fn aggregate_state(&mut self) -> Result<Option<Vec<ScalarValue>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.WindowState.md).


---

## WindowExpr

`trait` · `datafusion_physical_expr::window::window_expr::WindowExpr`

Also reachable as `datafusion::physical_plan::WindowExpr`, `datafusion_physical_expr::window::WindowExpr`, `datafusion_physical_plan::WindowExpr`, `datafusion_physical_plan::execution_plan::WindowExpr`, `datafusion_physical_plan::windows::WindowExpr`

```rust
trait WindowExpr: Send + Sync + Debug
```

**Implementors** (3)

- `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr`
- `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr`
- `datafusion_physical_expr::window::standard::StandardWindowExpr`

**Methods** (16)

```rust
fn all_expressions(&self) -> WindowPhysicalExpressions
fn as_any(&self) -> &dyn Any
fn create_window_fn(&self) -> Result<WindowFn>
fn evaluate(&self, batch: &RecordBatch) -> Result<ArrayRef>
fn evaluate_args(&self, batch: &RecordBatch) -> Result<Vec<ArrayRef>>
fn evaluate_stateful(&self, _partition_batches: &PartitionBatches, _window_agg_state: &mut PartitionWindowAggStates, _eval_ctx: &WindowEvalContext<'_>) -> Result<()>
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self) -> Result<FieldRef>
fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>
fn get_window_frame(&self) -> &Arc<WindowFrame>
fn name(&self) -> &str
fn order_by(&self) -> &[PhysicalSortExpr]
fn order_by_columns(&self, batch: &RecordBatch) -> Result<Vec<SortColumn>>
fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>]
fn uses_bounded_memory(&self) -> bool
fn with_new_expressions(&self, _args: Vec<Arc<dyn PhysicalExpr>>, _partition_bys: Vec<Arc<dyn PhysicalExpr>>, _order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<Arc<dyn WindowExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md).


Common trait for [window function] implementations

# Aggregate Window Expressions

These expressions take the form

```text
OVER({ROWS | RANGE| GROUPS} BETWEEN UNBOUNDED PRECEDING AND ...)
```

For example, cumulative window frames uses `PlainAggregateWindowExpr`.

# Non Aggregate Window Expressions

The expressions have the form

```text
OVER({ROWS | RANGE| GROUPS} BETWEEN M {PRECEDING| FOLLOWING} AND ...)
```

For example, sliding window frames use [`SlidingAggregateWindowExpr`].

[window function]: https://en.wikipedia.org/wiki/Window_function_(SQL)
[`PlainAggregateWindowExpr`]: crate::window::PlainAggregateWindowExpr
[`SlidingAggregateWindowExpr`]: crate::window::SlidingAggregateWindowExpr

---

## PartitionBatches

`type_alias` · `datafusion_physical_expr::window::window_expr::PartitionBatches`

Also reachable as `datafusion_physical_expr::window::PartitionBatches`

```rust
type PartitionBatches = indexmap::IndexMap<PartitionKey, datafusion_expr::window_state::PartitionBatchState, datafusion_common::hash_utils::RandomState>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.PartitionBatches.md).


The IndexMap (i.e. an ordered HashMap) where record batches are separated for each partition.

---

## PartitionKey

`type_alias` · `datafusion_physical_expr::window::window_expr::PartitionKey`

Also reachable as `datafusion_physical_expr::window::PartitionKey`

```rust
type PartitionKey = Vec<datafusion_common::ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.PartitionKey.md).


Key for IndexMap for each unique partition

For instance, if window frame is `OVER(PARTITION BY a,b)`,
PartitionKey would consist of unique `[a,b]` pairs

---

## PartitionWindowAggStates

`type_alias` · `datafusion_physical_expr::window::window_expr::PartitionWindowAggStates`

Also reachable as `datafusion_physical_expr::window::PartitionWindowAggStates`

```rust
type PartitionWindowAggStates = indexmap::IndexMap<PartitionKey, WindowState, datafusion_common::hash_utils::RandomState>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.window.window_expr.PartitionWindowAggStates.md).


---
