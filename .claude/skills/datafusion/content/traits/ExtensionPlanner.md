# ExtensionPlanner

`datafusion_session::planner::ExtensionPlanner`

```rust
trait ExtensionPlanner
```

Also reachable as `datafusion::physical_planner::ExtensionPlanner`, `datafusion_session::ExtensionPlanner`

Prose: [`api/datafusion_session.planner.md`](../api/datafusion_session.planner.md#extensionplanner) · records: [`model/datafusion_session.planner.json`](../model/datafusion_session.planner.json)

## Required

Every implementation must supply these.

```rust
async fn plan_extension(&self, planner: &dyn PhysicalPlanner, node: &dyn UserDefinedLogicalNode, logical_inputs: &[&LogicalPlan], physical_inputs: &[Arc<dyn ExecutionPlan>], session: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
async fn plan_table_scan(&self, _planner: &dyn PhysicalPlanner, _scan: &TableScan, _session: &dyn Session, _planning_ctx: &PhysicalPlanningContext) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

## Demonstrated by 2 upstream example(s)

- [`corpus/examples/dataframe/cache_factory.rs`](../corpus/examples/dataframe/cache_factory.rs)
- [`corpus/examples/relation_planner/table_sample.rs`](../corpus/examples/relation_planner/table_sample.rs)

## Documentation

This trait exposes the ability to plan an [`ExecutionPlan`] out of a [`LogicalPlan`].
