# `sqlparser::ast::Statement::LockTables`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.LockTables.json).

<a id="op-12c72f141ec8c8b1a6b82ce4"></a>
## tables

`struct_field` · `sqlparser::ast::Statement::LockTables::tables` · sqlparser 0.62.0

```rust
tables: Vec<LockTable>
```

Source: `src/ast/mod.rs:4730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of tables to lock with modes.
