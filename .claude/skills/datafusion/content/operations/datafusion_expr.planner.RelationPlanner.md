# `datafusion_expr::planner::RelationPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RelationPlanner.json).

<a id="op-1be858ef3f752319489e5fc4"></a>
## RelationPlanner

`trait` · `datafusion_expr::planner::RelationPlanner` · datafusion-expr 55.1.0

```rust
trait RelationPlanner: Debug + Send + Sync
```

Source: `src/planner.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Customize planning SQL table factors to [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql

<a id="op-f3111bf4dab5ec0b429c6cd0"></a>
## plan_relation

`function` · `datafusion_expr::planner::RelationPlanner::plan_relation` · datafusion-expr 55.1.0

```rust
fn plan_relation(&self, relation: TableFactor, context: &mut dyn RelationPlannerContext) -> Result<RelationPlanning>
```

Source: `src/planner.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan a table factor into a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

Returning [`RelationPlanning::Planned`](../operations/datafusion_expr.planner.RelationPlanning.md#op-724b4812d480aa3188056238) short-circuits further planning and uses the
provided plan. Returning [`RelationPlanning::Original`](../operations/datafusion_expr.planner.RelationPlanning.md#op-b971bf6e9fa586023e0b39df) allows the next registered planner,
or DataFusion's default logic, to handle the relation.
