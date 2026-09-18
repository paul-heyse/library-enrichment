# `sqlparser::ast::ddl::AlterTableOperation::DropConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.DropConstraint.json).

<a id="op-e2eddf63a7fcc1ad50316788"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropConstraint::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional drop behavior (`CASCADE`/`RESTRICT`).

<a id="op-79d92ed520d61e07b0dc7698"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropConstraint::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF EXISTS` flag for dropping the constraint.

<a id="op-100f0a5ff8bdbaf92aa77088"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::DropConstraint::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the constraint to drop.
