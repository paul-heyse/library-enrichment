# `sqlparser::ast::ddl::AlterTableOperation::AutoIncrement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.AutoIncrement.json).

<a id="op-4c1e371d2f8cdcbc41c6303c"></a>
## equals

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AutoIncrement::equals` · sqlparser 0.62.0

```rust
equals: bool
```

Source: `src/ast/ddl.rs:517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `=` sign was used (`AUTO_INCREMENT = ...`).

<a id="op-3a01e85a7425448ba49d7e1e"></a>
## value

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AutoIncrement::value` · sqlparser 0.62.0

```rust
value: ast::ValueWithSpan
```

Source: `src/ast/ddl.rs:519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value to set for the auto-increment counter.
