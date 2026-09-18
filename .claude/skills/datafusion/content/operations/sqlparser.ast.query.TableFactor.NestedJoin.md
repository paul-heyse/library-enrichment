# `sqlparser::ast::query::TableFactor::NestedJoin`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.NestedJoin.json).

<a id="op-38074c899a835013055b4ff0"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::NestedJoin::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the nested join.

<a id="op-426e829be4fc81ad577ce763"></a>
## table_with_joins

`struct_field` · `sqlparser::ast::query::TableFactor::NestedJoin::table_with_joins` · sqlparser 0.62.0

```rust
table_with_joins: Box<TableWithJoins>
```

Source: `src/ast/query.rs:1608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The nested join expression contained in parentheses.
