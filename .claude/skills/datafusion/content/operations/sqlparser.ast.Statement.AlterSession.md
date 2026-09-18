# `sqlparser::ast::Statement::AlterSession`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.AlterSession.json).

<a id="op-ed77957bb98ecb122d0b0e48"></a>
## session_params

`struct_field` · `sqlparser::ast::Statement::AlterSession::session_params` · sqlparser 0.62.0

```rust
session_params: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:3869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The session parameters to set or unset

<a id="op-e944b4d312b8b1513c0f4e39"></a>
## set

`struct_field` · `sqlparser::ast::Statement::AlterSession::set` · sqlparser 0.62.0

```rust
set: bool
```

Source: `src/ast/mod.rs:3867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

true is to set for the session parameters, false is to unset
