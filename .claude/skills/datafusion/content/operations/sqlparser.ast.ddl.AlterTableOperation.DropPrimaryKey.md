# `sqlparser::ast::ddl::AlterTableOperation::DropPrimaryKey`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.DropPrimaryKey.json).

<a id="op-1e5b7d8c9eeefb5d4ea485c8"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropPrimaryKey::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional drop behavior for the primary key (`CASCADE`/`RESTRICT`).
