# `datafusion_expr::utils::expand_qualified_wildcard`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.expand_qualified_wildcard.json).

<a id="op-a6a9852b3548795ef7340ac4"></a>
## expand_qualified_wildcard

`function` · `datafusion_expr::utils::expand_qualified_wildcard` · datafusion-expr 55.1.0

```rust
fn expand_qualified_wildcard(qualifier: &datafusion_common::TableReference, schema: &datafusion_common::DFSchema, wildcard_options: Option<&expr::WildcardOptions>) -> datafusion_common::Result<Vec<Expr>>
```

Source: `src/utils.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Resolves an `Expr::Wildcard` to a collection of qualified `Expr::Column`'s.
