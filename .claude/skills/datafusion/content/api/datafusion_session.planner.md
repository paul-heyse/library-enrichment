# `datafusion_session::planner`

Crate `datafusion-session` · 4 public items · structured records in [`model/datafusion_session.planner.json`](../model/datafusion_session.planner.json)

## UnsupportedQueryPlanner

`struct` · `datafusion_session::planner::UnsupportedQueryPlanner`

Also reachable as `datafusion::execution::context::UnsupportedQueryPlanner`, `datafusion_session::UnsupportedQueryPlanner`

```rust
struct UnsupportedQueryPlanner
```

**Implements**: `datafusion_session::planner::QueryPlanner`

**Derives**: Debug, Default

**via `datafusion_session::planner::QueryPlanner`**

```rust
async fn create_physical_plan(&self, _logical_plan: &LogicalPlan, _session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.planner.UnsupportedQueryPlanner.md).


A query planner that reports that planning is not implemented.

[`Session`] implementations that do not expose a query planner can return
this planner explicitly.

---

## ExtensionPlanner

`trait` · `datafusion_session::planner::ExtensionPlanner`

Also reachable as `datafusion::physical_planner::ExtensionPlanner`, `datafusion_session::ExtensionPlanner`

```rust
trait ExtensionPlanner
```

**Methods** (2)

```rust
async fn plan_extension(&self, planner: &dyn PhysicalPlanner, node: &dyn UserDefinedLogicalNode, logical_inputs: &[&LogicalPlan], physical_inputs: &[Arc<dyn ExecutionPlan>], session: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Option<Arc<dyn ExecutionPlan>>>
async fn plan_table_scan(&self, _planner: &dyn PhysicalPlanner, _scan: &TableScan, _session: &dyn Session, _planning_ctx: &PhysicalPlanningContext) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.planner.ExtensionPlanner.md).


This trait exposes the ability to plan an [`ExecutionPlan`] out of a [`LogicalPlan`].

---

## PhysicalPlanner

`trait` · `datafusion_session::planner::PhysicalPlanner`

Also reachable as `datafusion::physical_planner::PhysicalPlanner`, `datafusion_session::PhysicalPlanner`

```rust
trait PhysicalPlanner: Send + Sync
```

**Implementors** (1)

- `datafusion::physical_planner::DefaultPhysicalPlanner`

**Methods** (2)

```rust
fn create_physical_expr(&self, expr: &Expr, input_dfschema: &DFSchema, session: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Arc<dyn PhysicalExpr>>
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.planner.PhysicalPlanner.md).


Physical query planner that converts a [`LogicalPlan`] to an
[`ExecutionPlan`] suitable for execution.

---

## QueryPlanner

`trait` · `datafusion_session::planner::QueryPlanner`

Also reachable as `datafusion::execution::context::QueryPlanner`, `datafusion_session::QueryPlanner`

```rust
trait QueryPlanner: Any + Debug
```

**Implementors** (2)

- `datafusion_ffi::query_planner::ForeignQueryPlanner`
- `datafusion_session::planner::UnsupportedQueryPlanner`

**Methods** (1)

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.planner.QueryPlanner.md).


A planner that creates a physical plan for a query.

---
