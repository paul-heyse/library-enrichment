# `datafusion_expr::planner::RawFieldAccessExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RawFieldAccessExpr.json).

<a id="op-a93d4058b51c70a9d498ab77"></a>
## RawFieldAccessExpr

`struct` · `datafusion_expr::planner::RawFieldAccessExpr` · datafusion-expr 55.1.0

```rust
struct RawFieldAccessExpr
```

Source: `src/planner.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An expression with GetFieldAccess to plan

This structure is used by [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd) to plan operators with
custom expressions.

<a id="op-206dafe32e34a195595459f5"></a>
## clone

`function` · `datafusion_expr::planner::RawFieldAccessExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RawFieldAccessExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawFieldAccessExpr", "path": "RawFieldAccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 17], "end": [305, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5fafa87723279cd84a8b30c"></a>
## expr

`struct_field` · `datafusion_expr::planner::RawFieldAccessExpr::expr` · datafusion-expr 55.1.0

```rust
expr: Expr
```

Source: `src/planner.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f466b5de32aae6611cddb15"></a>
## field_access

`struct_field` · `datafusion_expr::planner::RawFieldAccessExpr::field_access` · datafusion-expr 55.1.0

```rust
field_access: GetFieldAccess
```

Source: `src/planner.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4d79da7b48d903a398507d2"></a>
## fmt

`function` · `datafusion_expr::planner::RawFieldAccessExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawFieldAccessExpr", "path": "RawFieldAccessExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 10], "end": [305, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
