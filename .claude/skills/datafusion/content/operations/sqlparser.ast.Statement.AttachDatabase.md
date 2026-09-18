# `sqlparser::ast::Statement::AttachDatabase`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.AttachDatabase.json).

<a id="op-b164926c02fa62525ecf321d"></a>
## database

`struct_field` · `sqlparser::ast::Statement::AttachDatabase::database` · sqlparser 0.62.0

```rust
database: bool
```

Source: `src/ast/mod.rs:3881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

true if the syntax is 'ATTACH DATABASE', false if it's just 'ATTACH'

<a id="op-38783874bec717f236956ba8"></a>
## database_file_name

`struct_field` · `sqlparser::ast::Statement::AttachDatabase::database_file_name` · sqlparser 0.62.0

```rust
database_file_name: Expr
```

Source: `src/ast/mod.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression that indicates the path to the database file

<a id="op-3ae64f72bcd4829b8cbe490f"></a>
## schema_name

`struct_field` · `sqlparser::ast::Statement::AttachDatabase::schema_name` · sqlparser 0.62.0

```rust
schema_name: Ident
```

Source: `src/ast/mod.rs:3877`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name to bind to the newly attached database
