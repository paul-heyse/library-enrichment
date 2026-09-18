# `sqlparser::ast::Statement::ShowCreate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowCreate.json).

<a id="op-6a40797351bfc2ace9cc60ef"></a>
## obj_name

`struct_field` · `sqlparser::ast::Statement::ShowCreate::obj_name` · sqlparser 0.62.0

```rust
obj_name: ObjectName
```

Source: `src/ast/mod.rs:4137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the object to show create statement for.

<a id="op-74cf260afe85115eb0b641a6"></a>
## obj_type

`struct_field` · `sqlparser::ast::Statement::ShowCreate::obj_type` · sqlparser 0.62.0

```rust
obj_type: ShowCreateObject
```

Source: `src/ast/mod.rs:4135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of object being shown (TABLE, VIEW, etc.).
