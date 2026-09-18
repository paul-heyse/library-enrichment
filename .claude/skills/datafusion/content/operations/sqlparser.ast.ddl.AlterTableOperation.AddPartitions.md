# `sqlparser::ast::ddl::AlterTableOperation::AddPartitions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.AddPartitions.json).

<a id="op-0378650d6cbf3dd5ee5fe43e"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddPartitions::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was present when adding partitions.

<a id="op-5d5449c48e796a96ce8a1993"></a>
## new_partitions

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddPartitions::new_partitions` · sqlparser 0.62.0

```rust
new_partitions: Vec<Partition>
```

Source: `src/ast/ddl.rs:368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New partitions to add.
