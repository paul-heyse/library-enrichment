# `sqlparser::ast::GrantObjects::Function`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.GrantObjects.Function.json).

<a id="op-e15209d7fcd0430db3803aca"></a>
## arg_types

`struct_field` · `sqlparser::ast::GrantObjects::Function::arg_types` · sqlparser 0.62.0

```rust
arg_types: Vec<DataType>
```

Source: `src/ast/mod.rs:7620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional argument types for overloaded functions.

<a id="op-9e3e89deb1234cf2d89c0ba1"></a>
## name

`struct_field` · `sqlparser::ast::GrantObjects::Function::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:7618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function name.
