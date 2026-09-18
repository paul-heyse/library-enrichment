# `sqlparser::ast::Statement::ShowTables`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ShowTables.json).

<a id="op-67a1f01b1bded66397c4399b"></a>
## extended

`struct_field` · `sqlparser::ast::Statement::ShowTables::extended` · sqlparser 0.62.0

```rust
extended: bool
```

Source: `src/ast/mod.rs:4214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when extended information should be shown.

<a id="op-914dbd02cbc69e95e555aea9"></a>
## external

`struct_field` · `sqlparser::ast::Statement::ShowTables::external` · sqlparser 0.62.0

```rust
external: bool
```

Source: `src/ast/mod.rs:4218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when external tables should be included.

<a id="op-361a60d8389cac84b418be67"></a>
## full

`struct_field` · `sqlparser::ast::Statement::ShowTables::full` · sqlparser 0.62.0

```rust
full: bool
```

Source: `src/ast/mod.rs:4216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when a full listing was requested.

<a id="op-3a030c1d94c9defe34e8bcf1"></a>
## history

`struct_field` · `sqlparser::ast::Statement::ShowTables::history` · sqlparser 0.62.0

```rust
history: bool
```

Source: `src/ast/mod.rs:4212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when history rows are requested.

<a id="op-cb5581e6adb20d5e209bcf30"></a>
## show_options

`struct_field` · `sqlparser::ast::Statement::ShowTables::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:4220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for `SHOW` statements.

<a id="op-7723c39111d08fe183509c9c"></a>
## terse

`struct_field` · `sqlparser::ast::Statement::ShowTables::terse` · sqlparser 0.62.0

```rust
terse: bool
```

Source: `src/ast/mod.rs:4210`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when terse output format was requested (compact listing).
