# `sqlparser::ast::dcl::AlterRoleOperation::Set`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.AlterRoleOperation.Set.json).

<a id="op-faedcf81e552d3374beadd9c"></a>
## config_name

`struct_field` · `sqlparser::ast::dcl::AlterRoleOperation::Set::config_name` · sqlparser 0.62.0

```rust
config_name: ast::ObjectName
```

Source: `src/ast/dcl.rs:176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Configuration name to set.

<a id="op-42053aff2cb7dba5fbe64cbb"></a>
## config_value

`struct_field` · `sqlparser::ast::dcl::AlterRoleOperation::Set::config_value` · sqlparser 0.62.0

```rust
config_value: SetConfigValue
```

Source: `src/ast/dcl.rs:178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value to assign to the configuration.

<a id="op-49703a7db1db650303f60e86"></a>
## in_database

`struct_field` · `sqlparser::ast::dcl::AlterRoleOperation::Set::in_database` · sqlparser 0.62.0

```rust
in_database: Option<ast::ObjectName>
```

Source: `src/ast/dcl.rs:180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional database scope for the setting.
