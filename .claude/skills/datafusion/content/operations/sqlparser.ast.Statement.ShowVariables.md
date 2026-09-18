# `sqlparser::ast::Statement::ShowVariables`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowVariables.json).

<a id="op-e88ed703732c01f998bd724a"></a>
## filter

`struct_field` · `sqlparser::ast::Statement::ShowVariables::filter` · sqlparser 0.62.0

```rust
filter: Option<ShowStatementFilter>
```

Source: `src/ast/mod.rs:4122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional filter for which variables to display.

<a id="op-da4946dcf6a5235e989f0b26"></a>
## global

`struct_field` · `sqlparser::ast::Statement::ShowVariables::global` · sqlparser 0.62.0

```rust
global: bool
```

Source: `src/ast/mod.rs:4124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `GLOBAL` scope was requested.

<a id="op-24b1af6ce622a54f3836225e"></a>
## session

`struct_field` · `sqlparser::ast::Statement::ShowVariables::session` · sqlparser 0.62.0

```rust
session: bool
```

Source: `src/ast/mod.rs:4126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `SESSION` scope was requested.
