# `sqlparser::ast::ddl::AlterTableOperation::AlterColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.AlterColumn.json).

<a id="op-8b14470b8184f6357ab88d7a"></a>
## column_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AlterColumn::column_name` · sqlparser 0.62.0

```rust
column_name: ast::Ident
```

Source: `src/ast/ddl.rs:429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The column to alter.

<a id="op-db6ef12c4c01911bdf6d62ed"></a>
## op

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AlterColumn::op` · sqlparser 0.62.0

```rust
op: AlterColumnOperation
```

Source: `src/ast/ddl.rs:431`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operation to apply to the column.
