# `sqlparser::ast::query::TableFactor::TableFunction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.TableFunction.json).

<a id="op-477ee785bc6795f07f30302f"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::TableFunction::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the table function result.

<a id="op-fe272b3caed1ac9098472340"></a>
## expr

`struct_field` · `sqlparser::ast::query::TableFactor::TableFunction::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:1513`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression representing the table function call.
