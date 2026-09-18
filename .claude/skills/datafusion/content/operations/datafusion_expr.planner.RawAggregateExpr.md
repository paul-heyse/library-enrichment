# `datafusion_expr::planner::RawAggregateExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RawAggregateExpr.json).

<a id="op-f82bbdd6c5fe362bd84e2df6"></a>
## RawAggregateExpr

`struct` · `datafusion_expr::planner::RawAggregateExpr` · datafusion-expr 55.1.0

```rust
struct RawAggregateExpr
```

Source: `src/planner.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This structure is used by `AggregateFunctionPlanner` to plan operators with
custom expressions.

<a id="op-0aa0cbbda2990072cb073fff"></a>
## args

`struct_field` · `datafusion_expr::planner::RawAggregateExpr::args` · datafusion-expr 55.1.0

```rust
args: Vec<Expr>
```

Source: `src/planner.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b9f58e46a4b6740f8c72db6"></a>
## clone

`function` · `datafusion_expr::planner::RawAggregateExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RawAggregateExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawAggregateExpr", "path": "RawAggregateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 17], "end": [323, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9cfbc27cdcd8833df60c177"></a>
## distinct

`struct_field` · `datafusion_expr::planner::RawAggregateExpr::distinct` · datafusion-expr 55.1.0

```rust
distinct: bool
```

Source: `src/planner.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06c88aba5137625147723113"></a>
## filter

`struct_field` · `datafusion_expr::planner::RawAggregateExpr::filter` · datafusion-expr 55.1.0

```rust
filter: Option<Box<Expr>>
```

Source: `src/planner.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ebcd423fa2939b1d139c2c4"></a>
## fmt

`function` · `datafusion_expr::planner::RawAggregateExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawAggregateExpr", "path": "RawAggregateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 10], "end": [323, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b67e59da666739c3263972d4"></a>
## func

`struct_field` · `datafusion_expr::planner::RawAggregateExpr::func` · datafusion-expr 55.1.0

```rust
func: std::sync::Arc<AggregateUDF>
```

Source: `src/planner.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-173a6fe7434c7a6b67204c2c"></a>
## null_treatment

`struct_field` · `datafusion_expr::planner::RawAggregateExpr::null_treatment` · datafusion-expr 55.1.0

```rust
null_treatment: Option<expr::NullTreatment>
```

Source: `src/planner.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a922b280ba5acdc6b899f98"></a>
## order_by

`struct_field` · `datafusion_expr::planner::RawAggregateExpr::order_by` · datafusion-expr 55.1.0

```rust
order_by: Vec<SortExpr>
```

Source: `src/planner.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
