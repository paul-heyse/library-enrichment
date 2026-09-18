# `sqlparser::ast::query::SelectItem::ExprWithAliases`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SelectItem.ExprWithAliases.json).

<a id="op-20317ecade332e3c025c07ae"></a>
## aliases

`struct_field` · `sqlparser::ast::query::SelectItem::ExprWithAliases::aliases` · sqlparser 0.62.0

```rust
aliases: Vec<Ident>
```

Source: `src/ast/query.rs:882`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of aliases for the expression.

<a id="op-df76cd8220f9dc3adbc8a963"></a>
## expr

`struct_field` · `sqlparser::ast::query::SelectItem::ExprWithAliases::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression being projected.
