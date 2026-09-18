# `datafusion_expr::utils::expand_wildcard`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.expand_wildcard.json).

<a id="op-f897bcce7668efd9da58b656"></a>
## expand_wildcard

`function` · `datafusion_expr::utils::expand_wildcard` · datafusion-expr 55.1.0

```rust
fn expand_wildcard(schema: &datafusion_common::DFSchema, plan: &LogicalPlan, wildcard_options: Option<&expr::WildcardOptions>) -> datafusion_common::Result<Vec<Expr>>
```

Source: `src/utils.rs:446`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Resolves an `Expr::Wildcard` to a collection of `Expr::Column`'s.
