# `sqlparser::ast::Statement::ShowCatalogs`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowCatalogs.json).

<a id="op-025230dd9c70dbcabaec9154"></a>
## history

`struct_field` · `sqlparser::ast::Statement::ShowCatalogs::history` · sqlparser 0.62.0

```rust
history: bool
```

Source: `src/ast/mod.rs:4157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when history information was requested.

<a id="op-44e9375c9289ee9f8aba1fe1"></a>
## show_options

`struct_field` · `sqlparser::ast::Statement::ShowCatalogs::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:4159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for `SHOW CATALOGS`.

<a id="op-78e69b0f0d2f5e7e6e3c3341"></a>
## terse

`struct_field` · `sqlparser::ast::Statement::ShowCatalogs::terse` · sqlparser 0.62.0

```rust
terse: bool
```

Source: `src/ast/mod.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when terse output format was requested.
