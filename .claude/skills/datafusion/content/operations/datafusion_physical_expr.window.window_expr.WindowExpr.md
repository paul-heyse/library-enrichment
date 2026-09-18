# `datafusion_physical_expr::window::window_expr::WindowExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.window_expr.WindowExpr.json).

<a id="op-4da3201a9ce53f61166a06ec"></a>
## WindowExpr

`trait` · `datafusion_physical_expr::window::window_expr::WindowExpr` · datafusion-physical-expr 55.1.0

```rust
trait WindowExpr: Send + Sync + Debug
```

Source: `src/window/window_expr.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

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

<a id="op-679132df7ebf4e59750e249d"></a>
## all_expressions

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::all_expressions` · datafusion-physical-expr 55.1.0

```rust
fn all_expressions(&self) -> WindowPhysicalExpressions
```

Source: `src/window/window_expr.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns all expressions used in the [`WindowExpr`](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec).
These expressions are (1) function arguments, (2) partition by expressions, (3) order by expressions.

<a id="op-ee8bcc09254e44c02b45ef8b"></a>
## as_any

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::as_any` · datafusion-physical-expr 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/window/window_expr.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the window expression as [`Any`] so that it can be
downcast to a specific implementation.

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-eaba4206f4ded63fed96ec17"></a>
## create_window_fn

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::create_window_fn` · datafusion-physical-expr 55.1.0

```rust
fn create_window_fn(&self) -> Result<WindowFn>
```

Source: `src/window/window_expr.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a new instance of the window function evaluator.

Returns `WindowFn::Builtin` for built-in window functions (e.g., ROW_NUMBER, RANK)
or `WindowFn::Aggregate` for aggregate window functions (e.g., SUM, AVG).

<a id="op-601594e9a8dd17b8ce37f373"></a>
## evaluate

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ArrayRef>
```

Source: `src/window/window_expr.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluate the window function values against the batch

<a id="op-27ab1d96ce6db434b888e499"></a>
## evaluate_args

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::evaluate_args` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_args(&self, batch: &RecordBatch) -> Result<Vec<ArrayRef>>
```

Source: `src/window/window_expr.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluate the window function arguments against the batch and return
array ref, normally the resulting `Vec` is a single element one.

<a id="op-c57bbb50bf0f0a07a0cb171c"></a>
## evaluate_stateful

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::evaluate_stateful` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_stateful(&self, _partition_batches: &PartitionBatches, _window_agg_state: &mut PartitionWindowAggStates, _eval_ctx: &WindowEvalContext<'_>) -> Result<()>
```

Source: `src/window/window_expr.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluate the window function against the batch. This function facilitates
stateful, bounded-memory implementations.

`eval_ctx` carries stream-level (cross-partition) information; see
[`WindowEvalContext`](../operations/datafusion_physical_expr.window.window_expr.WindowEvalContext.md#op-d3b1685bd5f4a21c388c2b2b).

<a id="op-08547b61d120c0cee382336d"></a>
## expressions

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::expressions` · datafusion-physical-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Source: `src/window/window_expr.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Expressions that are passed to the WindowAccumulator.
Functions which take a single input argument, such as `sum`, return a single [`datafusion_expr::expr::Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc),
others (e.g. `cov`) return many.

<a id="op-bf999080473083694f2175ec"></a>
## field

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self) -> Result<FieldRef>
```

Source: `src/window/window_expr.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The field of the final result of this window function.

<a id="op-d4747005cdf6346cf53f61e4"></a>
## get_reverse_expr

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::get_reverse_expr` · datafusion-physical-expr 55.1.0

```rust
fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>
```

Source: `src/window/window_expr.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the reverse expression of this [WindowExpr](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec).

<a id="op-688308551ba4c85c0a11272e"></a>
## get_window_frame

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::get_window_frame` · datafusion-physical-expr 55.1.0

```rust
fn get_window_frame(&self) -> &Arc<WindowFrame>
```

Source: `src/window/window_expr.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the window frame of this [WindowExpr](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec).

<a id="op-6d1cc342866e68ff4db3fa5a"></a>
## name

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/window/window_expr.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Human readable name such as `"MIN(c2)"` or `"RANK()"`. The default
implementation returns placeholder text.

<a id="op-de7bf42351393799f6c4525f"></a>
## order_by

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::order_by` · datafusion-physical-expr 55.1.0

```rust
fn order_by(&self) -> &[PhysicalSortExpr]
```

Source: `src/window/window_expr.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Expressions that's from the window function's order by clause, empty if absent

<a id="op-ad384d36143f34f2b4c13959"></a>
## order_by_columns

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::order_by_columns` · datafusion-physical-expr 55.1.0

```rust
fn order_by_columns(&self, batch: &RecordBatch) -> Result<Vec<SortColumn>>
```

Source: `src/window/window_expr.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get order by columns, empty if absent

<a id="op-1f38044395056497fc0009e1"></a>
## partition_by

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::partition_by` · datafusion-physical-expr 55.1.0

```rust
fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>]
```

Source: `src/window/window_expr.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Expressions that's from the window function's partition by clause, empty if absent

<a id="op-fde311fc829f2709e8086961"></a>
## uses_bounded_memory

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::uses_bounded_memory` · datafusion-physical-expr 55.1.0

```rust
fn uses_bounded_memory(&self) -> bool
```

Source: `src/window/window_expr.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a flag indicating whether this [WindowExpr](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec) can run with
bounded memory.

<a id="op-8f97d77011775fa179c474a5"></a>
## with_new_expressions

`function` · `datafusion_physical_expr::window::window_expr::WindowExpr::with_new_expressions` · datafusion-physical-expr 55.1.0

```rust
fn with_new_expressions(&self, _args: Vec<Arc<dyn PhysicalExpr>>, _partition_bys: Vec<Arc<dyn PhysicalExpr>>, _order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<Arc<dyn WindowExpr>>
```

Source: `src/window/window_expr.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Rewrites [`WindowExpr`](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec), with new expressions given. The argument should be consistent
with the return value of the [`WindowExpr::all_expressions`](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-679132df7ebf4e59750e249d) method.
Returns `Some(Arc<dyn WindowExpr>)` if re-write is supported, otherwise returns `None`.
