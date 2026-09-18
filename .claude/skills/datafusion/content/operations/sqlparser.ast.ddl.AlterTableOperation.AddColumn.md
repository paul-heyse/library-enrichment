# `sqlparser::ast::ddl::AlterTableOperation::AddColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.AddColumn.json).

<a id="op-c9e6b218116ea6f128babbea"></a>
## column_def

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddColumn::column_def` · sqlparser 0.62.0

```rust
column_def: ColumnDef
```

Source: `src/ast/ddl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

<column_def>.

<a id="op-f00bb254f8c09e388c162a40"></a>
## column_keyword

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddColumn::column_keyword` · sqlparser 0.62.0

```rust
column_keyword: bool
```

Source: `src/ast/ddl.rs:138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[COLUMN]`.

<a id="op-1149f6ae71bd32131b7c7d15"></a>
## column_position

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddColumn::column_position` · sqlparser 0.62.0

```rust
column_position: Option<ast::MySQLColumnPosition>
```

Source: `src/ast/ddl.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL `ALTER TABLE` only  [FIRST | AFTER column_name]

<a id="op-5de7b770ce5856cbca402da1"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddColumn::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[IF NOT EXISTS]`
