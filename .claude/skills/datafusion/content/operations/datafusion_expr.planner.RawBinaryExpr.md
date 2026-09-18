# `datafusion_expr::planner::RawBinaryExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RawBinaryExpr.json).

<a id="op-7ad59ce40ae28cfe78958aec"></a>
## RawBinaryExpr

`struct` · `datafusion_expr::planner::RawBinaryExpr` · datafusion-expr 55.1.0

```rust
struct RawBinaryExpr
```

Source: `src/planner.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An operator with two arguments to plan

Note `left` and `right` are DataFusion [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)s but the `op` is the SQL AST
operator.

This structure is used by [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd) to plan operators with
custom expressions.

<a id="op-8ee0d9f870a8b7dd9cf3fad3"></a>
## clone

`function` · `datafusion_expr::planner::RawBinaryExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RawBinaryExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawBinaryExpr", "path": "RawBinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 17], "end": [291, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20cc0534ccef6e57c42d9a95"></a>
## fmt

`function` · `datafusion_expr::planner::RawBinaryExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawBinaryExpr", "path": "RawBinaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 10], "end": [291, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c29491a3adfeec347882714"></a>
## left

`struct_field` · `datafusion_expr::planner::RawBinaryExpr::left` · datafusion-expr 55.1.0

```rust
left: Expr
```

Source: `src/planner.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39f288eb220ec16a5d055ac1"></a>
## op

`struct_field` · `datafusion_expr::planner::RawBinaryExpr::op` · datafusion-expr 55.1.0

```rust
op: sqlparser::ast::BinaryOperator
```

Source: `src/planner.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-851bf7150bce9156b4e96872"></a>
## right

`struct_field` · `datafusion_expr::planner::RawBinaryExpr::right` · datafusion-expr 55.1.0

```rust
right: Expr
```

Source: `src/planner.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
