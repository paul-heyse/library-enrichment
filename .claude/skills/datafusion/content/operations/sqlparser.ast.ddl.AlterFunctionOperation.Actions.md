# `sqlparser::ast::ddl::AlterFunctionOperation::Actions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterFunctionOperation.Actions.json).

<a id="op-c4dc8709cab05d6d113b309c"></a>
## actions

`struct_field` · `sqlparser::ast::ddl::AlterFunctionOperation::Actions::actions` · sqlparser 0.62.0

```rust
actions: Vec<AlterFunctionAction>
```

Source: `src/ast/ddl.rs:5430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more function actions.

<a id="op-f4a2a2ae70f7890d04c06c01"></a>
## restrict

`struct_field` · `sqlparser::ast::ddl::AlterFunctionOperation::Actions::restrict` · sqlparser 0.62.0

```rust
restrict: bool
```

Source: `src/ast/ddl.rs:5432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `RESTRICT` is present.
