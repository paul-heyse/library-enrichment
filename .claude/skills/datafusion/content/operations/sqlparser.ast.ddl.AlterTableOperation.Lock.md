# `sqlparser::ast::ddl::AlterTableOperation::Lock`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.Lock.json).

<a id="op-05d0adaead11141e46e1e18e"></a>
## equals

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::Lock::equals` · sqlparser 0.62.0

```rust
equals: bool
```

Source: `src/ast/ddl.rs:506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `=` sign was used (`LOCK = ...`).

<a id="op-623af031d1b5c3d756fb69eb"></a>
## lock

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::Lock::lock` · sqlparser 0.62.0

```rust
lock: AlterTableLock
```

Source: `src/ast/ddl.rs:508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The locking behavior to apply (MySQL-specific).
