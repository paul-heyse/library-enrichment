# `sqlparser::ast::Statement::NOTIFY`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.NOTIFY.json).

<a id="op-85bae205c857ab30e38d7db5"></a>
## channel

`struct_field` · `sqlparser::ast::Statement::NOTIFY::channel` · sqlparser 0.62.0

```rust
channel: Ident
```

Source: `src/ast/mod.rs:4825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Notification channel identifier.

<a id="op-c0b1a2744ccc33d42ab2f048"></a>
## payload

`struct_field` · `sqlparser::ast::Statement::NOTIFY::payload` · sqlparser 0.62.0

```rust
payload: Option<String>
```

Source: `src/ast/mod.rs:4827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional payload string.
