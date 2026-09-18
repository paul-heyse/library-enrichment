# `datafusion_expr::expr_rewriter::NamePreserver`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.NamePreserver.json).

<a id="op-9e562b963e28b6f1bf788bfb"></a>
## NamePreserver

`struct` · `datafusion_expr::expr_rewriter::NamePreserver` · datafusion-expr 55.1.0

```rust
struct NamePreserver
```

Source: `src/expr_rewriter/mod.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Handles ensuring the name of rewritten expressions is not changed.

This is important when optimizing plans to ensure the output
schema of plan nodes don't change after optimization.
For example, if an expression `1 + 2` is rewritten to `3`, the name of the
expression should be preserved: `3 as "1 + 2"`

See <https://github.com/apache/datafusion/issues/3555> for details

<a id="op-2d1cad4e2f2c223c886a4153"></a>
## new

`function` · `datafusion_expr::expr_rewriter::NamePreserver::new` · datafusion-expr 55.1.0

```rust
fn new(plan: &LogicalPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_rewriter::NamePreserver", "path": "NamePreserver"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [360, 2], "filename": "src/expr_rewriter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_rewriter/mod.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new NamePreserver for rewriting the `expr` that is part of the specified plan

<a id="op-c6011fef568d634bab077455"></a>
## new_for_projection

`function` · `datafusion_expr::expr_rewriter::NamePreserver::new_for_projection` · datafusion-expr 55.1.0

```rust
fn new_for_projection() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_rewriter::NamePreserver", "path": "NamePreserver"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [360, 2], "filename": "src/expr_rewriter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_rewriter/mod.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new NamePreserver for rewriting the `expr`s in `Projection`

This will use aliases

<a id="op-a4453ce6ebdb03f5b6b65d78"></a>
## save

`function` · `datafusion_expr::expr_rewriter::NamePreserver::save` · datafusion-expr 55.1.0

```rust
fn save(&self, expr: &Expr) -> SavedName
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_rewriter::NamePreserver", "path": "NamePreserver"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [360, 2], "filename": "src/expr_rewriter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_rewriter/mod.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
