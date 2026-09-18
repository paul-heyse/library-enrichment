# `datafusion_functions_nested::planner::NestedFunctionPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.planner.NestedFunctionPlanner.json).

<a id="op-14b36e188e00bec8b3ca406c"></a>
## NestedFunctionPlanner

`struct` · `datafusion_functions_nested::planner::NestedFunctionPlanner` · datafusion-functions-nested 55.1.0

```rust
struct NestedFunctionPlanner
```

Source: `src/planner.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d97e88e3dcab38cf9528a1a8"></a>
## fmt

`function` · `datafusion_functions_nested::planner::NestedFunctionPlanner::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::planner::NestedFunctionPlanner", "path": "NestedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ef3b81e0b82e00d83e84c3"></a>
## plan_array_literal

`function` · `datafusion_functions_nested::planner::NestedFunctionPlanner::plan_array_literal` · datafusion-functions-nested 55.1.0

```rust
fn plan_array_literal(&self, exprs: Vec<Expr>, _schema: &DFSchema) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::planner::NestedFunctionPlanner", "path": "NestedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [123, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b74e549fe3581740e2134ff9"></a>
## plan_binary_op

`function` · `datafusion_functions_nested::planner::NestedFunctionPlanner::plan_binary_op` · datafusion-functions-nested 55.1.0

```rust
fn plan_binary_op(&self, expr: RawBinaryExpr, schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::planner::NestedFunctionPlanner", "path": "NestedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [123, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f73406c5972cae157eb8864c"></a>
## plan_make_map

`function` · `datafusion_functions_nested::planner::NestedFunctionPlanner::plan_make_map` · datafusion-functions-nested 55.1.0

```rust
fn plan_make_map(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::planner::NestedFunctionPlanner", "path": "NestedFunctionPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [123, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
