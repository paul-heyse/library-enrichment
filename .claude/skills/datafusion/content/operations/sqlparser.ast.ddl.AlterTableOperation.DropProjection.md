# `sqlparser::ast::ddl::AlterTableOperation::DropProjection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.DropProjection.json).

<a id="op-df6cb615ffa705de3f75ef64"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropProjection::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified.

<a id="op-659ea21fcf75986c90494135"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropProjection::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the projection to drop.
