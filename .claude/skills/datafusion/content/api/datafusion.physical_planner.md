# `datafusion::physical_planner`

Crate `datafusion` · 6 public items · structured records in [`model/datafusion.physical_planner.json`](../model/datafusion.physical_planner.json)

## create_aggregate_expr_and_maybe_filter

`function` · `datafusion::physical_planner::create_aggregate_expr_and_maybe_filter`

> **Deprecated** — use LoweredAggregateBuilder

```rust
fn create_aggregate_expr_and_maybe_filter(e: &logical_expr::Expr, logical_input_schema: &datafusion_common::DFSchema, physical_input_schema: &arrow::datatypes::Schema, execution_props: &execution::context::ExecutionProps) -> error::Result<(std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>, Option<std::sync::Arc<dyn PhysicalExpr>>, Vec<datafusion_physical_expr::PhysicalSortExpr>)>
```

Create an aggregate expression from a logical expression or an alias

---

## create_aggregate_expr_with_name_and_maybe_filter

`function` · `datafusion::physical_planner::create_aggregate_expr_with_name_and_maybe_filter`

> **Deprecated** — use LoweredAggregateBuilder

```rust
fn create_aggregate_expr_with_name_and_maybe_filter(e: &logical_expr::Expr, name: Option<String>, human_display: String, logical_input_schema: &datafusion_common::DFSchema, physical_input_schema: &arrow::datatypes::Schema, execution_props: &execution::context::ExecutionProps) -> error::Result<(std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>, Option<std::sync::Arc<dyn PhysicalExpr>>, Vec<datafusion_physical_expr::PhysicalSortExpr>)>
```

Create an aggregate expression with a name from a logical expression

---

## create_window_expr

`function` · `datafusion::physical_planner::create_window_expr`

```rust
fn create_window_expr(e: &logical_expr::Expr, logical_schema: &datafusion_common::DFSchema, execution_props: &execution::context::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> error::Result<std::sync::Arc<dyn WindowExpr>>
```

Create a window expression from a logical expression or an alias

See [`create_physical_expr`] for details on the `planning_ctx` argument.

---

## create_window_expr_with_name

`function` · `datafusion::physical_planner::create_window_expr_with_name`

```rust
fn create_window_expr_with_name(e: &logical_expr::Expr, name: impl Into<String>, logical_schema: &datafusion_common::DFSchema, execution_props: &execution::context::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> error::Result<std::sync::Arc<dyn WindowExpr>>
```

Create a window expression with a name from a logical expression

See [`create_physical_expr`] for details on the `planning_ctx` argument.

---

## is_window_frame_bound_valid

`function` · `datafusion::physical_planner::is_window_frame_bound_valid`

```rust
fn is_window_frame_bound_valid(window_frame: &datafusion_expr::WindowFrame) -> bool
```

Check if window bounds are valid after schema information is available, and
window_frame bounds are casted to the corresponding column type.
queries like:
OVER (ORDER BY a RANGES BETWEEN 3 PRECEDING AND 5 PRECEDING)
OVER (ORDER BY a RANGES BETWEEN INTERVAL '3 DAY' PRECEDING AND '5 DAY' PRECEDING)  are rejected

---

## DefaultPhysicalPlanner

`struct` · `datafusion::physical_planner::DefaultPhysicalPlanner`

```rust
struct DefaultPhysicalPlanner
```

**Implements**: `datafusion_session::planner::PhysicalPlanner`

**Derives**: Default

**Methods** (2)

```rust
fn optimize_physical_plan<F>(&self, plan: Arc<dyn ExecutionPlan>, session_state: &dyn Session, observer: F) -> Result<Arc<dyn ExecutionPlan>> where F: FnMut(&dyn ExecutionPlan, &dyn PhysicalOptimizerRule)
fn with_extension_planners(extension_planners: Vec<Arc<dyn ExtensionPlanner + Send + Sync>>) -> Self
```

**via `datafusion_session::planner::PhysicalPlanner`**

```rust
fn create_physical_expr(&self, expr: &Expr, input_dfschema: &DFSchema, session_state: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Arc<dyn PhysicalExpr>>
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session_state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Default single node physical query planner that converts a
`LogicalPlan` to an `ExecutionPlan` suitable for execution.

This planner first flattens the `LogicalPlan` tree with a depth-first
traversal. It then builds the physical plan from the leaves to the root.
Up to [`planning_concurrency`] tasks execute concurrently.

[`planning_concurrency`]: crate::config::ExecutionOptions::planning_concurrency

---
