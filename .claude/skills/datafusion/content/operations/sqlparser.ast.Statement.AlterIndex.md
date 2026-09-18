# `sqlparser::ast::Statement::AlterIndex`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.AlterIndex.json).

<a id="op-11cbf9d2900c74a6293a1855"></a>
## name

`struct_field` · `sqlparser::ast::Statement::AlterIndex::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:3778`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the index to alter.

<a id="op-437282626023cf06e1bbb581"></a>
## operation

`struct_field` · `sqlparser::ast::Statement::AlterIndex::operation` · sqlparser 0.62.0

```rust
operation: AlterIndexOperation
```

Source: `src/ast/mod.rs:3780`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operation to perform on the index.
