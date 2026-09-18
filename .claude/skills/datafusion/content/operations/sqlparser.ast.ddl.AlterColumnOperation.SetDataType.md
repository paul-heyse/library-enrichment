# `sqlparser::ast::ddl::AlterColumnOperation::SetDataType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterColumnOperation.SetDataType.json).

<a id="op-4feecfce32bf7d77d4076b25"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::AlterColumnOperation::SetDataType::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:1284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target data type for the column.

<a id="op-192e839a1b33cbbeaeda217f"></a>
## had_set

`struct_field` · `sqlparser::ast::ddl::AlterColumnOperation::SetDataType::had_set` · sqlparser 0.62.0

```rust
had_set: bool
```

Source: `src/ast/ddl.rs:1288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set to true if the statement includes the `SET DATA TYPE` keywords.

<a id="op-579a4eafb8577de97b68e88e"></a>
## using

`struct_field` · `sqlparser::ast::ddl::AlterColumnOperation::SetDataType::using` · sqlparser 0.62.0

```rust
using: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:1286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL-specific `USING <expr>` expression for conversion.
