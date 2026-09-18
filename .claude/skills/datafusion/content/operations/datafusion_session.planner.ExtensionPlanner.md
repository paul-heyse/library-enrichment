# `datafusion_session::planner::ExtensionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.planner.ExtensionPlanner.json).

<a id="op-b97e1eb479e5c979b3058c3c"></a>
## ExtensionPlanner

`trait` · `datafusion_session::planner::ExtensionPlanner` · datafusion-session 55.1.0

```rust
trait ExtensionPlanner
```

Source: `src/planner.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

This trait exposes the ability to plan an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) out of a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

<a id="op-8778415278afe45a93d263cd"></a>
## plan_extension

`function` · `datafusion_session::planner::ExtensionPlanner::plan_extension` · datafusion-session 55.1.0

```rust
async fn plan_extension(&self, planner: &dyn PhysicalPlanner, node: &dyn UserDefinedLogicalNode, logical_inputs: &[&LogicalPlan], physical_inputs: &[Arc<dyn ExecutionPlan>], session: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Source: `src/planner.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a physical plan for a [`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432).

`input_dfschema`: the logical plan schema for the inputs to this node

Returns an error when the planner knows how to plan the concrete
implementation of `node` but errors while doing so.

Returns `None` when the planner does not know how to plan the
`node` and wants to delegate the planning to another
[`ExtensionPlanner`](../operations/datafusion_session.planner.ExtensionPlanner.md#op-b97e1eb479e5c979b3058c3c).

`planning_ctx` is the [`PhysicalPlanningContext`](../operations/datafusion_expr.physical_planning_context.PhysicalPlanningContext.md#op-6c42198b3422be2c46b9cf8b) of the plan subtree
currently being converted to a physical plan. Forward it to
[`PhysicalPlanner::create_physical_expr`](../operations/datafusion_session.planner.PhysicalPlanner.md#op-7238ed4d05e3ceb7f9870a38) when creating this node's
physical expressions so that scalar subqueries resolve against the same
subquery state as the rest of the plan.

<a id="op-5e8c8b38b5d60f79e22f6309"></a>
## plan_table_scan

`function` · `datafusion_session::planner::ExtensionPlanner::plan_table_scan` · datafusion-session 55.1.0

```rust
async fn plan_table_scan(&self, _planner: &dyn PhysicalPlanner, _scan: &TableScan, _session: &dyn Session, _planning_ctx: &PhysicalPlanningContext) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Source: `src/planner.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a physical plan for a [`LogicalPlan::TableScan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-3b721470f2fbd687699e58a8).

This is useful for planning valid [`TableSource`]s that are not `TableProvider`s.

Returns:
* `Ok(Some(plan))` if the planner knows how to plan the `scan`
* `Ok(None)` if the planner does not know how to plan the `scan` and wants to delegate the planning to another [`ExtensionPlanner`](../operations/datafusion_session.planner.ExtensionPlanner.md#op-b97e1eb479e5c979b3058c3c)
* `Err` if the planner knows how to plan the `scan` but errors while doing so

# Example

```rust,ignore
use std::sync::Arc;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::logical_expr::TableScan;
use datafusion::catalog::Session;
use datafusion::error::Result;
use datafusion_session::{ExtensionPlanner, PhysicalPlanner};
use async_trait::async_trait;

// Your custom table source type
struct MyCustomTableSource { /* ... */ }

// Your custom execution plan
struct MyCustomExec { /* ... */ }

struct MyExtensionPlanner;

#[async_trait]
impl ExtensionPlanner for MyExtensionPlanner {
    async fn plan_extension(
        &self,
        _planner: &dyn PhysicalPlanner,
        _node: &dyn UserDefinedLogicalNode,
        _logical_inputs: &[&LogicalPlan],
        _physical_inputs: &[Arc<dyn ExecutionPlan>],
        _session: &dyn Session,
        _planning_ctx: &PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        Ok(None)
    }

    async fn plan_table_scan(
        &self,
        _planner: &dyn PhysicalPlanner,
        scan: &TableScan,
        _session: &dyn Session,
        _planning_ctx: &PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        // Check if this is your custom table source
        if scan.source.is::<MyCustomTableSource>() {
            // Create a custom execution plan for your table source
            let exec = MyCustomExec::new(
                scan.table_name.clone(),
                Arc::clone(scan.projected_schema.inner()),
            );
            Ok(Some(Arc::new(exec)))
        } else {
            // Return None to let other extension planners handle it
            Ok(None)
        }
    }
}
```

[`TableSource`]: datafusion_expr::TableSource
