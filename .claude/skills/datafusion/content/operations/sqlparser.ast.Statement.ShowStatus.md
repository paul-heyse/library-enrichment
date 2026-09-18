# `sqlparser::ast::Statement::ShowStatus`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowStatus.json).

<a id="op-7be14e461162447c0e843b6e"></a>
## filter

`struct_field` · `sqlparser::ast::Statement::ShowStatus::filter` · sqlparser 0.62.0

```rust
filter: Option<ShowStatementFilter>
```

Source: `src/ast/mod.rs:4109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional filter for which status entries to display.

<a id="op-b78aec13e16ffd415dedc34a"></a>
## global

`struct_field` · `sqlparser::ast::Statement::ShowStatus::global` · sqlparser 0.62.0

```rust
global: bool
```

Source: `src/ast/mod.rs:4111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `GLOBAL` scope was requested.

<a id="op-217e475b54dfca663b2b7e41"></a>
## session

`struct_field` · `sqlparser::ast::Statement::ShowStatus::session` · sqlparser 0.62.0

```rust
session: bool
```

Source: `src/ast/mod.rs:4113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `SESSION` scope was requested.
