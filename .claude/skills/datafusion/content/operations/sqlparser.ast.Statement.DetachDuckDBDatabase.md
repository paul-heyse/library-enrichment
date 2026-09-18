# `sqlparser::ast::Statement::DetachDuckDBDatabase`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.DetachDuckDBDatabase.json).

<a id="op-1b91859ab22824e48e17e672"></a>
## database

`struct_field` · `sqlparser::ast::Statement::DetachDuckDBDatabase::database` · sqlparser 0.62.0

```rust
database: bool
```

Source: `src/ast/mod.rs:3909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` if the syntax used `DETACH DATABASE` rather than `DETACH`.

<a id="op-ef2f25ab90259ae4e9a10265"></a>
## database_alias

`struct_field` · `sqlparser::ast::Statement::DetachDuckDBDatabase::database_alias` · sqlparser 0.62.0

```rust
database_alias: Ident
```

Source: `src/ast/mod.rs:3911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alias of the database to detach.

<a id="op-59dd02b6a21d5c45764d16f8"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::DetachDuckDBDatabase::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:3907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF EXISTS` was present.
