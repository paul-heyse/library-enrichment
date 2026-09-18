# `sqlparser::ast::ddl::AlterTableOperation::Algorithm`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.Algorithm.json).

<a id="op-8382f1515d8960b15633fb05"></a>
## algorithm

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::Algorithm::algorithm` · sqlparser 0.62.0

```rust
algorithm: AlterTableAlgorithm
```

Source: `src/ast/ddl.rs:496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The algorithm to use for the alter operation (MySQL-specific).

<a id="op-d26a287b274c126e760edaa3"></a>
## equals

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::Algorithm::equals` · sqlparser 0.62.0

```rust
equals: bool
```

Source: `src/ast/ddl.rs:494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `=` sign was used (`ALGORITHM = ...`).
