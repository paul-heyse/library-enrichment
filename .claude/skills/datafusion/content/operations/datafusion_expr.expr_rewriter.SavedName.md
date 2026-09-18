# `datafusion_expr::expr_rewriter::SavedName`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.SavedName.json).

<a id="op-d9a83165db924830638e4dd4"></a>
## SavedName

`enum` · `datafusion_expr::expr_rewriter::SavedName` · datafusion-expr 55.1.0

```rust
enum SavedName
```

Source: `src/expr_rewriter/mod.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If the qualified name of an expression is remembered, it will be preserved
when rewriting the expression

<a id="op-e3ff78801ae3bc21ce9eb31d"></a>
## None

`variant` · `datafusion_expr::expr_rewriter::SavedName::None` · datafusion-expr 55.1.0

```rust
None
```

Source: `src/expr_rewriter/mod.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Name is not preserved

<a id="op-7444871d6b5cd5149c3e2692"></a>
## Saved

`variant` · `datafusion_expr::expr_rewriter::SavedName::Saved` · datafusion-expr 55.1.0

```rust
Saved
```

Source: `src/expr_rewriter/mod.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Saved qualified name to be preserved

<a id="op-c6a79aed345957cf33b9f8d2"></a>
## fmt

`function` · `datafusion_expr::expr_rewriter::SavedName::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_rewriter::SavedName", "path": "SavedName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 10], "end": [309, 15], "filename": "src/expr_rewriter/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr_rewriter/mod.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0353cb3c6b96c56e13ee2887"></a>
## restore

`function` · `datafusion_expr::expr_rewriter::SavedName::restore` · datafusion-expr 55.1.0

```rust
fn restore(self, expr: Expr) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_rewriter::SavedName", "path": "SavedName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [377, 2], "filename": "src/expr_rewriter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_rewriter/mod.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Ensures the qualified name of the rewritten expression is preserved
