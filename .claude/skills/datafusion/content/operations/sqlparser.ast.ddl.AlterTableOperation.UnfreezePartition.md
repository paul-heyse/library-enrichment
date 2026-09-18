# `sqlparser::ast::ddl::AlterTableOperation::UnfreezePartition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.UnfreezePartition.json).

<a id="op-25bde330ea4ce6fbe492c544"></a>
## partition

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::UnfreezePartition::partition` · sqlparser 0.62.0

```rust
partition: Partition
```

Source: `src/ast/ddl.rs:262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Partition to unfreeze.

<a id="op-90f68010324f4bbd8c912e0f"></a>
## with_name

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::UnfreezePartition::with_name` · sqlparser 0.62.0

```rust
with_name: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional name associated with the unfreeze operation.
