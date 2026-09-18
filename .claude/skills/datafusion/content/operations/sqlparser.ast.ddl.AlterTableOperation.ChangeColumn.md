# `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.ChangeColumn.json).

<a id="op-cae52161b00d9f0e1d208b9d"></a>
## column_position

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn::column_position` · sqlparser 0.62.0

```rust
column_position: Option<ast::MySQLColumnPosition>
```

Source: `src/ast/ddl.rs:401`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific column position (`FIRST`/`AFTER`).

<a id="op-dab472e5c1e3ef8280f1ce80"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:397`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New data type for the column.

<a id="op-f0842dcf6860657f4274bf27"></a>
## new_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn::new_name` · sqlparser 0.62.0

```rust
new_name: ast::Ident
```

Source: `src/ast/ddl.rs:395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New column name.

<a id="op-f6973f29ab27120b2bd8067c"></a>
## old_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn::old_name` · sqlparser 0.62.0

```rust
old_name: ast::Ident
```

Source: `src/ast/ddl.rs:393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Old column name.

<a id="op-b43ebbfddbd57b73c1930104"></a>
## options

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn::options` · sqlparser 0.62.0

```rust
options: Vec<ColumnOption>
```

Source: `src/ast/ddl.rs:399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column options to apply after the change.
