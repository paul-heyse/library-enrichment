# QueryPlanner

`datafusion_session::planner::QueryPlanner`

```rust
trait QueryPlanner: Any + Debug
```

Also reachable as `datafusion::execution::context::QueryPlanner`, `datafusion_session::QueryPlanner`

Prose: [`api/datafusion_session.planner.md`](../api/datafusion_session.planner.md#queryplanner) · records: [`model/datafusion_session.planner.json`](../model/datafusion_session.planner.json)

## Required

Every implementation must supply these.

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

## Implementors (2)

Read one before writing your own.

- `datafusion_ffi::query_planner::ForeignQueryPlanner`
- `datafusion_session::planner::UnsupportedQueryPlanner`

## Demonstrated by 2 upstream example(s)

- [`corpus/examples/dataframe/cache_factory.rs`](../corpus/examples/dataframe/cache_factory.rs)
- [`corpus/examples/relation_planner/table_sample.rs`](../corpus/examples/relation_planner/table_sample.rs)

## Documentation

A planner that creates a physical plan for a query.
