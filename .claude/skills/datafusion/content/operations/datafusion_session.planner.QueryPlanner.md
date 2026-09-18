# `datafusion_session::planner::QueryPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.planner.QueryPlanner.json).

<a id="op-d105e63a68841dd69dcdcc42"></a>
## QueryPlanner

`trait` · `datafusion_session::planner::QueryPlanner` · datafusion-session 55.1.0

```rust
trait QueryPlanner: Any + Debug
```

Source: `src/planner.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A planner that creates a physical plan for a query.

<a id="op-5fef6c6dc1819f8ba019a7cb"></a>
## create_physical_plan

`function` · `datafusion_session::planner::QueryPlanner::create_physical_plan` · datafusion-session 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/planner.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Given a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da), create an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) suitable for execution
