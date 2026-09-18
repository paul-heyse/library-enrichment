# `sqlparser::ast::Statement::AttachDuckDBDatabase`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.AttachDuckDBDatabase.json).

<a id="op-464d622614476be51c3a36e9"></a>
## attach_options

`struct_field` · `sqlparser::ast::Statement::AttachDuckDBDatabase::attach_options` · sqlparser 0.62.0

```rust
attach_options: Vec<AttachDuckDBDatabaseOption>
```

Source: `src/ast/mod.rs:3898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific attach options (e.g., `READ_ONLY`).

<a id="op-308222a6928c064bf057f142"></a>
## database

`struct_field` · `sqlparser::ast::Statement::AttachDuckDBDatabase::database` · sqlparser 0.62.0

```rust
database: bool
```

Source: `src/ast/mod.rs:3892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` if the syntax used `ATTACH DATABASE` rather than `ATTACH`.

<a id="op-efbda5edd7d8b211c7bfc8c2"></a>
## database_alias

`struct_field` · `sqlparser::ast::Statement::AttachDuckDBDatabase::database_alias` · sqlparser 0.62.0

```rust
database_alias: Option<Ident>
```

Source: `src/ast/mod.rs:3896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias assigned to the attached database.

<a id="op-554100139acbc763e0f43485"></a>
## database_path

`struct_field` · `sqlparser::ast::Statement::AttachDuckDBDatabase::database_path` · sqlparser 0.62.0

```rust
database_path: Ident
```

Source: `src/ast/mod.rs:3894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path identifier to the database file being attached.

<a id="op-67412a16778e684f498f408d"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::AttachDuckDBDatabase::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:3890`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF NOT EXISTS` was present.
