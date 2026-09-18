# `sqlparser::ast::ddl::AlterTableOperation::FreezePartition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.FreezePartition.json).

<a id="op-3d6e47792cf76c0518e5136c"></a>
## partition

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::FreezePartition::partition` · sqlparser 0.62.0

```rust
partition: Partition
```

Source: `src/ast/ddl.rs:253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Partition to freeze.

<a id="op-390da755af3505c77191194f"></a>
## with_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::FreezePartition::with_name` · sqlparser 0.62.0

```rust
with_name: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:255`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional name for the freeze operation.
