# `sqlparser::ast::ddl::AlterTableOperation::ModifyColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.ModifyColumn.json).

<a id="op-767db17810eb652793f5059a"></a>
## col_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ModifyColumn::col_name` · sqlparser 0.62.0

```rust
col_name: ast::Ident
```

Source: `src/ast/ddl.rs:407`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column name to modify.

<a id="op-e303faad1d560cc6bb9aca38"></a>
## column_position

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ModifyColumn::column_position` · sqlparser 0.62.0

```rust
column_position: Option<ast::MySQLColumnPosition>
```

Source: `src/ast/ddl.rs:413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific column position (`FIRST`/`AFTER`).

<a id="op-028daec5b646dd76aaaf2f1c"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ModifyColumn::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New data type for the column.

<a id="op-2acc04d02f38228af13dbe62"></a>
## options

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ModifyColumn::options` · sqlparser 0.62.0

```rust
options: Vec<ColumnOption>
```

Source: `src/ast/ddl.rs:411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column options to set.
