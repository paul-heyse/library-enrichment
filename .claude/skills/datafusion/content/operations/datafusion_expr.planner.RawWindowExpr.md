# `datafusion_expr::planner::RawWindowExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RawWindowExpr.json).

<a id="op-0c96b767447fb7f2e7d624f7"></a>
## RawWindowExpr

`struct` · `datafusion_expr::planner::RawWindowExpr` · datafusion-expr 55.1.0

```rust
struct RawWindowExpr
```

Source: `src/planner.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This structure is used by `WindowFunctionPlanner` to plan operators with
custom expressions.

<a id="op-7474f657d4aaa0489c7dde79"></a>
## args

`struct_field` · `datafusion_expr::planner::RawWindowExpr::args` · datafusion-expr 55.1.0

```rust
args: Vec<Expr>
```

Source: `src/planner.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ec79ff91da67dd7a36cc1cb"></a>
## clone

`function` · `datafusion_expr::planner::RawWindowExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RawWindowExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawWindowExpr", "path": "RawWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 17], "end": [335, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26b5860bd27c5ea34adf35fe"></a>
## distinct

`struct_field` · `datafusion_expr::planner::RawWindowExpr::distinct` · datafusion-expr 55.1.0

```rust
distinct: bool
```

Source: `src/planner.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49285baeaff6488ed1652638"></a>
## filter

`struct_field` · `datafusion_expr::planner::RawWindowExpr::filter` · datafusion-expr 55.1.0

```rust
filter: Option<Box<Expr>>
```

Source: `src/planner.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8c1cc4ce63423d78be99d01"></a>
## fmt

`function` · `datafusion_expr::planner::RawWindowExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawWindowExpr", "path": "RawWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 10], "end": [335, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55f33bc3b4ffe37761ebf385"></a>
## func_def

`struct_field` · `datafusion_expr::planner::RawWindowExpr::func_def` · datafusion-expr 55.1.0

```rust
func_def: WindowFunctionDefinition
```

Source: `src/planner.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81ffa6be16ae60b59a38f904"></a>
## null_treatment

`struct_field` · `datafusion_expr::planner::RawWindowExpr::null_treatment` · datafusion-expr 55.1.0

```rust
null_treatment: Option<expr::NullTreatment>
```

Source: `src/planner.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b29554c90862205b0fa77cf"></a>
## order_by

`struct_field` · `datafusion_expr::planner::RawWindowExpr::order_by` · datafusion-expr 55.1.0

```rust
order_by: Vec<SortExpr>
```

Source: `src/planner.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc18024d150cf0eae15eaa78"></a>
## partition_by

`struct_field` · `datafusion_expr::planner::RawWindowExpr::partition_by` · datafusion-expr 55.1.0

```rust
partition_by: Vec<Expr>
```

Source: `src/planner.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7881ab961cfe2b28e81ce700"></a>
## window_frame

`struct_field` · `datafusion_expr::planner::RawWindowExpr::window_frame` · datafusion-expr 55.1.0

```rust
window_frame: WindowFrame
```

Source: `src/planner.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
