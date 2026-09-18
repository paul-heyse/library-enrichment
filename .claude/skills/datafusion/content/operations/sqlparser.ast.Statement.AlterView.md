# `sqlparser::ast::Statement::AlterView`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.AlterView.json).

<a id="op-d6a01da30fa7907c646dec70"></a>
## columns

`struct_field` · `sqlparser::ast::Statement::AlterView::columns` · sqlparser 0.62.0

```rust
columns: Vec<Ident>
```

Source: `src/ast/mod.rs:3790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional new column list for the view.

<a id="op-1985e442f517fff5fbf01e7a"></a>
## name

`struct_field` · `sqlparser::ast::Statement::AlterView::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:3788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

View name being altered.

<a id="op-d4048a72a40989b68dd6c7bb"></a>
## query

`struct_field` · `sqlparser::ast::Statement::AlterView::query` · sqlparser 0.62.0

```rust
query: Box<Query>
```

Source: `src/ast/mod.rs:3792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Replacement query for the view definition.

<a id="op-7d6f6898de7911a64268f7a7"></a>
## with_options

`struct_field` · `sqlparser::ast::Statement::AlterView::with_options` · sqlparser 0.62.0

```rust
with_options: Vec<SqlOption>
```

Source: `src/ast/mod.rs:3794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional WITH options for the view.
