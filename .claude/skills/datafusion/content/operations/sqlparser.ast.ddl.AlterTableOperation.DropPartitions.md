# `sqlparser::ast::ddl::AlterTableOperation::DropPartitions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.DropPartitions.json).

<a id="op-048ecabe1089920a027e6c2d"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropPartitions::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified for dropping partitions.

<a id="op-d991fa97a6e6c4e3054b0fa9"></a>
## partitions

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropPartitions::partitions` · sqlparser 0.62.0

```rust
partitions: Vec<ast::Expr>
```

Source: `src/ast/ddl.rs:373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Partitions to drop (expressions).
