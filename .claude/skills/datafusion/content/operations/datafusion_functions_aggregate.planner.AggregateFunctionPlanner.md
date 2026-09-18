# `datafusion_functions_aggregate::planner::AggregateFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.planner.AggregateFunctionPlanner.json).

<a id="op-1382f293ffc7da5832657414"></a>
## AggregateFunctionPlanner

`struct` · `datafusion_functions_aggregate::planner::AggregateFunctionPlanner` · datafusion-functions-aggregate 55.1.0

```rust
struct AggregateFunctionPlanner
```

Source: `src/planner.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdf94d0906274353999a79a4"></a>
## fmt

`function` · `datafusion_functions_aggregate::planner::AggregateFunctionPlanner::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::planner::AggregateFunctionPlanner", "path": "AggregateFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eb59409879dfe74d3498de1"></a>
## plan_aggregate

`function` · `datafusion_functions_aggregate::planner::AggregateFunctionPlanner::plan_aggregate` · datafusion-functions-aggregate 55.1.0

```rust
fn plan_aggregate(&self, raw_expr: RawAggregateExpr) -> Result<PlannerResult<RawAggregateExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::planner::AggregateFunctionPlanner", "path": "AggregateFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [116, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
