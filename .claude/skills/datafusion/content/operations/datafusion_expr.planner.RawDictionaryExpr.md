# `datafusion_expr::planner::RawDictionaryExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.RawDictionaryExpr.json).

<a id="op-3babe7610638e6b532857db8"></a>
## RawDictionaryExpr

`struct` · `datafusion_expr::planner::RawDictionaryExpr` · datafusion-expr 55.1.0

```rust
struct RawDictionaryExpr
```

Source: `src/planner.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A Dictionary literal expression `{ key: value, ...}`

This structure is used by [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd) to plan operators with
custom expressions.

<a id="op-f0932eb0908484a590270a66"></a>
## clone

`function` · `datafusion_expr::planner::RawDictionaryExpr::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> RawDictionaryExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawDictionaryExpr", "path": "RawDictionaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 17], "end": [315, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-607f801a8ee1259b0e53fc52"></a>
## fmt

`function` · `datafusion_expr::planner::RawDictionaryExpr::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::planner::RawDictionaryExpr", "path": "RawDictionaryExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [315, 10], "end": [315, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-740b2cc77acef44e14a7243c"></a>
## keys

`struct_field` · `datafusion_expr::planner::RawDictionaryExpr::keys` · datafusion-expr 55.1.0

```rust
keys: Vec<Expr>
```

Source: `src/planner.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c56d124bf8882ffc3444abd"></a>
## values

`struct_field` · `datafusion_expr::planner::RawDictionaryExpr::values` · datafusion-expr 55.1.0

```rust
values: Vec<Expr>
```

Source: `src/planner.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
