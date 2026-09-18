# `sqlparser::ast::Statement::DropConnector`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.DropConnector.json).

<a id="op-69ac41babedc0833a38bce71"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::DropConnector::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:3985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF EXISTS` was present.

<a id="op-bdd64ab666f22cbdb34ccc7a"></a>
## name

`struct_field` · `sqlparser::ast::Statement::DropConnector::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:3987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the connector to drop.
