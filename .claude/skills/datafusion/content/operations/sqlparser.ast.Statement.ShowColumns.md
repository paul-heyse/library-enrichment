# `sqlparser::ast::Statement::ShowColumns`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowColumns.json).

<a id="op-c8bfa59fcd6c516cd0f4a6b0"></a>
## extended

`struct_field` · `sqlparser::ast::Statement::ShowColumns::extended` · sqlparser 0.62.0

```rust
extended: bool
```

Source: `src/ast/mod.rs:4144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when extended column information was requested.

<a id="op-6d41a025ae155b7c7dc096c0"></a>
## full

`struct_field` · `sqlparser::ast::Statement::ShowColumns::full` · sqlparser 0.62.0

```rust
full: bool
```

Source: `src/ast/mod.rs:4146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when full column details were requested.

<a id="op-4df7e6a4676f31e7a287757c"></a>
## show_options

`struct_field` · `sqlparser::ast::Statement::ShowColumns::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:4148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for `SHOW COLUMNS`.
