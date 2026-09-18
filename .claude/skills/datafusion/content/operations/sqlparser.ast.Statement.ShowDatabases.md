# `sqlparser::ast::Statement::ShowDatabases`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowDatabases.json).

<a id="op-5a928fc954d4318fd5cb69c1"></a>
## history

`struct_field` · `sqlparser::ast::Statement::ShowDatabases::history` · sqlparser 0.62.0

```rust
history: bool
```

Source: `src/ast/mod.rs:4168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when history information was requested.

<a id="op-1fa1f151e98113276cdf045c"></a>
## show_options

`struct_field` · `sqlparser::ast::Statement::ShowDatabases::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:4170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for `SHOW DATABASES`.

<a id="op-c76d7bc85b7959bf43d86f3c"></a>
## terse

`struct_field` · `sqlparser::ast::Statement::ShowDatabases::terse` · sqlparser 0.62.0

```rust
terse: bool
```

Source: `src/ast/mod.rs:4166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when terse output format was requested.
