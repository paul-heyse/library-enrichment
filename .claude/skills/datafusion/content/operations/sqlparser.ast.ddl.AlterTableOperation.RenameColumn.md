# `sqlparser::ast::ddl::AlterTableOperation::RenameColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.RenameColumn.json).

<a id="op-b964a9bd6e9b18b14198580f"></a>
## new_column_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::RenameColumn::new_column_name` · sqlparser 0.62.0

```rust
new_column_name: ast::Ident
```

Source: `src/ast/ddl.rs:382`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New column name.

<a id="op-38e6ddcd15821bd1d1661cdd"></a>
## old_column_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::RenameColumn::old_column_name` · sqlparser 0.62.0

```rust
old_column_name: ast::Ident
```

Source: `src/ast/ddl.rs:380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Existing column name to rename.
