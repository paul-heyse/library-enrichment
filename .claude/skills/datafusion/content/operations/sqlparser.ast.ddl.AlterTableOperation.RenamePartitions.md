# `sqlparser::ast::ddl::AlterTableOperation::RenamePartitions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.RenamePartitions.json).

<a id="op-0d2bb3ace1328f8eaefbf3d4"></a>
## new_partitions

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::RenamePartitions::new_partitions` · sqlparser 0.62.0

```rust
new_partitions: Vec<ast::Expr>
```

Source: `src/ast/ddl.rs:353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New partition expressions corresponding to the old ones.

<a id="op-f6422e6d636ae4c431860f9a"></a>
## old_partitions

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::RenamePartitions::old_partitions` · sqlparser 0.62.0

```rust
old_partitions: Vec<ast::Expr>
```

Source: `src/ast/ddl.rs:351`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Old partition expressions to be renamed.
