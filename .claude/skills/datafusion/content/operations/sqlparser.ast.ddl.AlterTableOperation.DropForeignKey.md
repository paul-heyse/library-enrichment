# `sqlparser::ast::ddl::AlterTableOperation::DropForeignKey`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.DropForeignKey.json).

<a id="op-63eb9a015dc75af35b81e43f"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropForeignKey::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional drop behavior for the foreign key.

<a id="op-056404fcb7183e305bc151b4"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropForeignKey::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Foreign key symbol/name to drop.
