# RelationPlanner

`datafusion_expr::planner::RelationPlanner`

```rust
trait RelationPlanner: Debug + Send + Sync
```

Prose: [`api/datafusion_expr.planner.md`](../api/datafusion_expr.planner.md#relationplanner) · records: [`model/datafusion_expr.planner.json`](../model/datafusion_expr.planner.json)

## Required

Every implementation must supply these.

```rust
fn plan_relation(&self, relation: TableFactor, context: &mut dyn RelationPlannerContext) -> Result<RelationPlanning>
```

## Demonstrated by 3 upstream example(s)

- [`corpus/examples/relation_planner/match_recognize.rs`](../corpus/examples/relation_planner/match_recognize.rs)
- [`corpus/examples/relation_planner/pivot_unpivot.rs`](../corpus/examples/relation_planner/pivot_unpivot.rs)
- [`corpus/examples/relation_planner/table_sample.rs`](../corpus/examples/relation_planner/table_sample.rs)

## Documentation

Customize planning SQL table factors to [`LogicalPlan`]s.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql
