# `datafusion_expr::planner::RelationPlanning`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RelationPlanning.json).

<a id="op-fa8b2c11c2cbfd80bbd32e1e"></a>
## RelationPlanning

`enum` · `datafusion_expr::planner::RelationPlanning` · datafusion-expr 55.1.0

```rust
enum RelationPlanning
```

Source: `src/planner.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Result of attempting to plan a relation with extension planners

<a id="op-b971bf6e9fa586023e0b39df"></a>
## Original

`variant` · `datafusion_expr::planner::RelationPlanning::Original` · datafusion-expr 55.1.0

```rust
Original
```

Source: `src/planner.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No extension planner handled the relation, return it for default processing

<a id="op-724b4812d480aa3188056238"></a>
## Planned

`variant` · `datafusion_expr::planner::RelationPlanning::Planned` · datafusion-expr 55.1.0

```rust
Planned
```

Source: `src/planner.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The relation was successfully planned by an extension planner

<a id="op-c36cd8137a2d3c384493d63e"></a>
## fmt

`function` · `datafusion_expr::planner::RelationPlanning::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RelationPlanning", "path": "RelationPlanning"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 10], "end": [376, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
