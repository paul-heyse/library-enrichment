# `sqlparser::ast::GrantObjects::Procedure`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.GrantObjects.Procedure.json).

<a id="op-83664dd0a744ed55bd8bd7ef"></a>
## arg_types

`struct_field` · `sqlparser::ast::GrantObjects::Procedure::arg_types` · sqlparser 0.62.0

```rust
arg_types: Vec<DataType>
```

Source: `src/ast/mod.rs:7608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional argument types for overloaded procedures.

<a id="op-23dbf137bec5467ce1e78262"></a>
## name

`struct_field` · `sqlparser::ast::GrantObjects::Procedure::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:7606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The procedure name.
