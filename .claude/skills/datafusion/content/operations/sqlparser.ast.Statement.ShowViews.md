# `sqlparser::ast::Statement::ShowViews`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowViews.json).

<a id="op-9db202cd5acf4fa2f942bc55"></a>
## materialized

`struct_field` · `sqlparser::ast::Statement::ShowViews::materialized` · sqlparser 0.62.0

```rust
materialized: bool
```

Source: `src/ast/mod.rs:4229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when materialized views should be included.

<a id="op-d64dfb3292169640e1571ffb"></a>
## show_options

`struct_field` · `sqlparser::ast::Statement::ShowViews::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:4231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for `SHOW` statements.

<a id="op-6d90398ce699e8bf9f6effa3"></a>
## terse

`struct_field` · `sqlparser::ast::Statement::ShowViews::terse` · sqlparser 0.62.0

```rust
terse: bool
```

Source: `src/ast/mod.rs:4227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when terse output format was requested.
