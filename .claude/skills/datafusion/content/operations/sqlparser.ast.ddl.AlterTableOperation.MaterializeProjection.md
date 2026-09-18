# `sqlparser::ast::ddl::AlterTableOperation::MaterializeProjection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.MaterializeProjection.json).

<a id="op-24c74842723107cf77e2497a"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::MaterializeProjection::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified.

<a id="op-f1e8a9b4ee10f5515a3eb53d"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::MaterializeProjection::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the projection to materialize.

<a id="op-0baaa320232d487722979d9c"></a>
## partition

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::MaterializeProjection::partition` · sqlparser 0.62.0

```rust
partition: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition name to operate on.
