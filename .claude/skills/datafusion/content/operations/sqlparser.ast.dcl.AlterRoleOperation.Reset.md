# `sqlparser::ast::dcl::AlterRoleOperation::Reset`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.AlterRoleOperation.Reset.json).

<a id="op-e2863a58c2b1079767e40da9"></a>
## config_name

`struct_field` · `sqlparser::ast::dcl::AlterRoleOperation::Reset::config_name` · sqlparser 0.62.0

```rust
config_name: ResetConfig
```

Source: `src/ast/dcl.rs:188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Configuration to reset.

<a id="op-296434ac400db1a70ee1fe52"></a>
## in_database

`struct_field` · `sqlparser::ast::dcl::AlterRoleOperation::Reset::in_database` · sqlparser 0.62.0

```rust
in_database: Option<ast::ObjectName>
```

Source: `src/ast/dcl.rs:190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional database scope for the reset.
