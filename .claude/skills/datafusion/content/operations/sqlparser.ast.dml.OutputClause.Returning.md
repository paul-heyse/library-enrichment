# `sqlparser::ast::dml::OutputClause::Returning`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.OutputClause.Returning.json).

<a id="op-53a61b5e08e4bda9521e5f24"></a>
## returning_token

`struct_field` · `sqlparser::ast::dml::OutputClause::Returning::returning_token` · sqlparser 0.62.0

```rust
returning_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `RETURNING` token that starts the sub-expression.

<a id="op-9df3db7354d5e0a1a4d5f339"></a>
## select_items

`struct_field` · `sqlparser::ast::dml::OutputClause::Returning::select_items` · sqlparser 0.62.0

```rust
select_items: Vec<super::SelectItem>
```

Source: `src/ast/dml.rs:773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The select items to return
