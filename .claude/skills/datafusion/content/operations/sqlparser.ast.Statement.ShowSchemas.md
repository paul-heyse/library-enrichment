# `sqlparser::ast::Statement::ShowSchemas`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowSchemas.json).

<a id="op-8414755e5803935a0218ac01"></a>
## history

`struct_field` · `sqlparser::ast::Statement::ShowSchemas::history` · sqlparser 0.62.0

```rust
history: bool
```

Source: `src/ast/mod.rs:4188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when history information was requested.

<a id="op-332cc57ee01b58e39af5226a"></a>
## show_options

`struct_field` · `sqlparser::ast::Statement::ShowSchemas::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:4190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for `SHOW SCHEMAS`.

<a id="op-7a9c2e4273b3b7cd3ad13417"></a>
## terse

`struct_field` · `sqlparser::ast::Statement::ShowSchemas::terse` · sqlparser 0.62.0

```rust
terse: bool
```

Source: `src/ast/mod.rs:4186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when terse (compact) output was requested.
