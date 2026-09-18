# `datafusion_physical_plan::windows`

Crate `datafusion-physical-plan` · 7 public items · structured records in [`model/datafusion_physical_plan.windows.json`](../model/datafusion_physical_plan.windows.json)

## create_udwf_window_expr

`function` · `datafusion_physical_plan::windows::create_udwf_window_expr`

```rust
fn create_udwf_window_expr(fun: &std::sync::Arc<datafusion_expr::WindowUDF>, args: &[std::sync::Arc<dyn PhysicalExpr>], input_schema: &arrow::datatypes::Schema, name: String, ignore_nulls: bool) -> datafusion_common::Result<std::sync::Arc<dyn StandardWindowFunctionExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.create_udwf_window_expr.md).


Creates a `StandardWindowFunctionExpr` suitable for a user defined window function

---

## create_window_expr

`function` · `datafusion_physical_plan::windows::create_window_expr`

```rust
fn create_window_expr(fun: &datafusion_expr::WindowFunctionDefinition, name: String, args: &[std::sync::Arc<dyn PhysicalExpr>], partition_by: &[std::sync::Arc<dyn PhysicalExpr>], order_by: &[expressions::PhysicalSortExpr], window_frame: std::sync::Arc<datafusion_expr::WindowFrame>, input_schema: arrow::datatypes::SchemaRef, ignore_nulls: bool, distinct: bool, filter: Option<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn WindowExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.create_window_expr.md).


Create a physical expression for window function

---

## get_best_fitting_window

`function` · `datafusion_physical_plan::windows::get_best_fitting_window`

```rust
fn get_best_fitting_window(window_exprs: &[std::sync::Arc<dyn WindowExpr>], input: &std::sync::Arc<dyn ExecutionPlan>, physical_partition_keys: &[std::sync::Arc<dyn PhysicalExpr>], state_observer: Option<std::sync::Arc<dyn WindowStateObserver>>) -> datafusion_common::Result<Option<std::sync::Arc<dyn ExecutionPlan>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.get_best_fitting_window.md).


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

---

## get_ordered_partition_by_indices

`function` · `datafusion_physical_plan::windows::get_ordered_partition_by_indices`

```rust
fn get_ordered_partition_by_indices(partition_by_exprs: &[std::sync::Arc<dyn PhysicalExpr>], input: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<Vec<usize>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.get_ordered_partition_by_indices.md).


This function calculates the indices such that when partition by expressions reordered with the indices
resulting expressions define a preset for existing ordering.
For instance, if input is ordered by a, b, c and PARTITION BY b, a is used,
this vector will be [1, 0]. It means that when we iterate b, a columns with the order [1, 0]
resulting vector (a, b) is a preset of the existing ordering (a, b, c).

---

## get_window_mode

`function` · `datafusion_physical_plan::windows::get_window_mode`

```rust
fn get_window_mode(partitionby_exprs: &[std::sync::Arc<dyn PhysicalExpr>], orderby_keys: &[expressions::PhysicalSortExpr], input: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<Option<(bool, InputOrderMode)>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.get_window_mode.md).


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

---

## schema_add_window_field

`function` · `datafusion_physical_plan::windows::schema_add_window_field`

```rust
fn schema_add_window_field(args: &[std::sync::Arc<dyn PhysicalExpr>], schema: &arrow::datatypes::Schema, window_fn: &datafusion_expr::WindowFunctionDefinition, fn_name: &str) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::Schema>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.schema_add_window_field.md).


Build field from window function and add it into schema

---

## WindowUDFExpr

`struct` · `datafusion_physical_plan::windows::WindowUDFExpr`

```rust
struct WindowUDFExpr
```

**Implements**: `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn args(&self) -> &[Arc<dyn PhysicalExpr>]
fn fun(&self) -> &Arc<WindowUDF>
```

**via `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr`**

```rust
fn as_any(&self) -> &dyn std::any::Any
fn create_evaluator(&self) -> Result<Box<dyn PartitionEvaluator>>
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn field(&self) -> Result<FieldRef>
fn get_result_ordering(&self, schema: &SchemaRef) -> Option<PhysicalSortExpr>
fn limit_effect(&self) -> LimitEffect
fn name(&self) -> &str
fn reverse_expr(&self) -> Option<Arc<dyn StandardWindowFunctionExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.windows.WindowUDFExpr.md).


Implements [`StandardWindowFunctionExpr`] for [`WindowUDF`]

---
