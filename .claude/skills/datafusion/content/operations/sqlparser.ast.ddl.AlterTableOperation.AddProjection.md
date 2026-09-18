# `sqlparser::ast::ddl::AlterTableOperation::AddProjection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.AddProjection.json).

<a id="op-4e03fb445fc05e7855918961"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddProjection::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-92e6f751ea7bb56ee2be07ab"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddProjection::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the projection to add.

<a id="op-5ea0372d820c02ca8511d5f7"></a>
## select

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddProjection::select` · sqlparser 0.62.0

```rust
select: ast::ProjectionSelect
```

Source: `src/ast/ddl.rs:156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The projection's select clause.
