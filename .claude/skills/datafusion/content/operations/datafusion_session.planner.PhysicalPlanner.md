# `datafusion_session::planner::PhysicalPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.planner.PhysicalPlanner.json).

<a id="op-76d5bdb77a53d51c64c680ce"></a>
## PhysicalPlanner

`trait` · `datafusion_session::planner::PhysicalPlanner` · datafusion-session 55.1.0

```rust
trait PhysicalPlanner: Send + Sync
```

Source: `src/planner.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Physical query planner that converts a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) to an
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) suitable for execution.

<a id="op-7238ed4d05e3ceb7f9870a38"></a>
## create_physical_expr

`function` · `datafusion_session::planner::PhysicalPlanner::create_physical_expr` · datafusion-session 55.1.0

```rust
fn create_physical_expr(&self, expr: &Expr, input_dfschema: &DFSchema, session: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/planner.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a physical expression from a logical expression
suitable for evaluation

`expr`: the expression to convert

`input_dfschema`: the logical plan schema for evaluating `expr`

`planning_ctx`: the [`PhysicalPlanningContext`](../operations/datafusion_expr.physical_planning_context.PhysicalPlanningContext.md#op-6c42198b3422be2c46b9cf8b) used to resolve
`Expr::ScalarSubquery` nodes. During physical planning the planner
threads the context of the plan currently being converted to a physical
plan (for example into [`ExtensionPlanner::plan_extension`](../operations/datafusion_session.planner.ExtensionPlanner.md#op-8778415278afe45a93d263cd), which
should forward it here). Callers creating physical expressions outside
of a plan should pass `&PhysicalPlanningContext::default()`.

<a id="op-840aadf8fa288cd156adbcf1"></a>
## create_physical_plan

`function` · `datafusion_session::planner::PhysicalPlanner::create_physical_plan` · datafusion-session 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/planner.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a physical plan from a logical plan
