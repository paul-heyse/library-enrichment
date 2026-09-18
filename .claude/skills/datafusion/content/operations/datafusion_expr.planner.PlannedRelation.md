# `datafusion_expr::planner::PlannedRelation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.PlannedRelation.json).

<a id="op-7fa39351f5455389501d8f76"></a>
## PlannedRelation

`struct` · `datafusion_expr::planner::PlannedRelation` · datafusion-expr 55.1.0

```rust
struct PlannedRelation
```

Source: `src/planner.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Result of planning a relation with [`RelationPlanner`](../operations/datafusion_expr.planner.RelationPlanner.md#op-1be858ef3f752319489e5fc4)

<a id="op-173bb9b5771e5acb6e72774f"></a>
## alias

`struct_field` · `datafusion_expr::planner::PlannedRelation::alias` · datafusion-expr 55.1.0

```rust
alias: Option<sqlparser::ast::TableAlias>
```

Source: `src/planner.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional table alias for the relation

<a id="op-cdc02df9ecd1abcc043607dc"></a>
## clone

`function` · `datafusion_expr::planner::PlannedRelation::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> PlannedRelation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::PlannedRelation", "path": "PlannedRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 17], "end": [358, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-441fa9d09864bd5c0606ef94"></a>
## fmt

`function` · `datafusion_expr::planner::PlannedRelation::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::PlannedRelation", "path": "PlannedRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 10], "end": [358, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a973edbf168fd189f64e5f98"></a>
## new

`function` · `datafusion_expr::planner::PlannedRelation::new` · datafusion-expr 55.1.0

```rust
fn new(plan: LogicalPlan, alias: Option<TableAlias>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::PlannedRelation", "path": "PlannedRelation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 1], "end": [372, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:369`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `PlannedRelation` with the given plan and alias

<a id="op-e309ada9d9b1e51cc4c50be6"></a>
## plan

`struct_field` · `datafusion_expr::planner::PlannedRelation::plan` · datafusion-expr 55.1.0

```rust
plan: logical_plan::LogicalPlan
```

Source: `src/planner.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan for the relation
