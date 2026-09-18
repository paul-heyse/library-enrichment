# `datafusion_functions_nested::planner::FieldAccessPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.planner.FieldAccessPlanner.json).

<a id="op-e24a8b12f86b800b62b339b1"></a>
## FieldAccessPlanner

`struct` · `datafusion_functions_nested::planner::FieldAccessPlanner` · datafusion-functions-nested 55.1.0

```rust
struct FieldAccessPlanner
```

Source: `src/planner.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22a68275e3f3fdcb99c4660d"></a>
## fmt

`function` · `datafusion_functions_nested::planner::FieldAccessPlanner::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::planner::FieldAccessPlanner", "path": "FieldAccessPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 10], "end": [125, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc03aef03bc62f400567c27"></a>
## plan_field_access

`function` · `datafusion_functions_nested::planner::FieldAccessPlanner::plan_field_access` · datafusion-functions-nested 55.1.0

```rust
fn plan_field_access(&self, expr: RawFieldAccessExpr, schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::planner::FieldAccessPlanner", "path": "FieldAccessPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [192, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_expr::planner::ExprPlanner", "path": "ExprPlanner"}, "trait_path": "datafusion_expr::planner::ExprPlanner"}`

Source: `src/planner.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
