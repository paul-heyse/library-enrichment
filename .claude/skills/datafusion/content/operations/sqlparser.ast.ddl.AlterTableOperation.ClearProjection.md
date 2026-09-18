# `sqlparser::ast::ddl::AlterTableOperation::ClearProjection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.ClearProjection.json).

<a id="op-e3897069701dae2ed1456da0"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ClearProjection::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified.

<a id="op-fe55c2f11bfaecb51b08c3c1"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ClearProjection::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the projection to clear.

<a id="op-c55f55e3a663238604eff57b"></a>
## partition

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::ClearProjection::partition` · sqlparser 0.62.0

```rust
partition: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition name to operate on.
