# `datafusion_expr::expr_rewriter::normalize_col_with_schemas_and_ambiguity_check`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.normalize_col_with_schemas_and_ambiguity_check.json).

<a id="op-f59f810d5d26325e8090a22b"></a>
## normalize_col_with_schemas_and_ambiguity_check

`function` · `datafusion_expr::expr_rewriter::normalize_col_with_schemas_and_ambiguity_check` · datafusion-expr 55.1.0

```rust
fn normalize_col_with_schemas_and_ambiguity_check(expr: Expr, schemas: &[&[&datafusion_common::DFSchema]], using_columns: &[std::collections::HashSet<datafusion_common::Column>]) -> datafusion_common::Result<Expr>
```

Source: `src/expr_rewriter/mod.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`Column::normalize_with_schemas_and_ambiguity_check`] for usage

Unresolved upstream links (retained, not inferred): ``Column::normalize_with_schemas_and_ambiguity_check``.
