# `sqlparser::ast::ddl::AlterSchemaOperation::AddReplica`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterSchemaOperation.AddReplica.json).

<a id="op-8cb9aec4fa154c715dc3d77b"></a>
## options

`struct_field` · `sqlparser::ast::ddl::AlterSchemaOperation::AddReplica::options` · sqlparser 0.62.0

```rust
options: Option<Vec<ast::SqlOption>>
```

Source: `src/ast/ddl.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional options for the replica.

<a id="op-6c35b321c945e806f015502f"></a>
## replica

`struct_field` · `sqlparser::ast::ddl::AlterSchemaOperation::AddReplica::replica` · sqlparser 0.62.0

```rust
replica: ast::Ident
```

Source: `src/ast/ddl.rs:3807`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The replica to add.
