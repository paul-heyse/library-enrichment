# `sqlparser::ast::ddl::AlterTableOperation::DropColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.DropColumn.json).

<a id="op-0683696e53f8efb51b589685"></a>
## column_names

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropColumn::column_names` · sqlparser 0.62.0

```rust
column_names: Vec<ast::Ident>
```

Source: `src/ast/ddl.rs:225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Names of columns to drop.

<a id="op-d581fc6db244c14553fd1864"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropColumn::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional drop behavior for the column removal.

<a id="op-cb089707c97489d1424181b3"></a>
## has_column_keyword

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropColumn::has_column_keyword` · sqlparser 0.62.0

```rust
has_column_keyword: bool
```

Source: `src/ast/ddl.rs:223`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `COLUMN` keyword was present.

<a id="op-0a2c454f8cd5f647c07450da"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropColumn::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified for the columns.
