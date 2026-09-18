# `sqlparser::ast::Statement::Rollback`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Rollback.json).

<a id="op-a87958d55adfdd7dca075650"></a>
## chain

`struct_field` · `sqlparser::ast::Statement::Rollback::chain` · sqlparser 0.62.0

```rust
chain: bool
```

Source: `src/ast/mod.rs:4328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `AND [ NO ] CHAIN` was present.

<a id="op-cbb27e5354cb0447a92c721f"></a>
## savepoint

`struct_field` · `sqlparser::ast::Statement::Rollback::savepoint` · sqlparser 0.62.0

```rust
savepoint: Option<Ident>
```

Source: `src/ast/mod.rs:4330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional savepoint name to roll back to.
