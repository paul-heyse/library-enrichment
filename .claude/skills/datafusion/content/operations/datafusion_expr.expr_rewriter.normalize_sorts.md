# `datafusion_expr::expr_rewriter::normalize_sorts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.normalize_sorts.json).

<a id="op-3b967459329bb4e08d75ff8c"></a>
## normalize_sorts

`function` · `datafusion_expr::expr_rewriter::normalize_sorts` · datafusion-expr 55.1.0

```rust
fn normalize_sorts(sorts: impl IntoIterator<Item = impl Into<expr::Sort>>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<expr::Sort>>
```

Source: `src/expr_rewriter/mod.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
