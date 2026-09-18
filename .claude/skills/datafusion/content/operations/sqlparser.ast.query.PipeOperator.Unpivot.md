# `sqlparser::ast::query::PipeOperator::Unpivot`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Unpivot.json).

<a id="op-fdeba831c31c7d701b2cb050"></a>
## alias

`struct_field` · `sqlparser::ast::query::PipeOperator::Unpivot::alias` · sqlparser 0.62.0

```rust
alias: Option<Ident>
```

Source: `src/ast/query.rs:3336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the unpivot result.

<a id="op-4fbf41a3ff8f34a5ed0ceb18"></a>
## name_column

`struct_field` · `sqlparser::ast::query::PipeOperator::Unpivot::name_column` · sqlparser 0.62.0

```rust
name_column: Ident
```

Source: `src/ast/query.rs:3332`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column name holding the unpivoted column name.

<a id="op-bd6d66b14736ab34123f1965"></a>
## unpivot_columns

`struct_field` · `sqlparser::ast::query::PipeOperator::Unpivot::unpivot_columns` · sqlparser 0.62.0

```rust
unpivot_columns: Vec<Ident>
```

Source: `src/ast/query.rs:3334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns to unpivot.

<a id="op-dbe97fb403980a479c45e0f5"></a>
## value_column

`struct_field` · `sqlparser::ast::query::PipeOperator::Unpivot::value_column` · sqlparser 0.62.0

```rust
value_column: Ident
```

Source: `src/ast/query.rs:3330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Output column that will receive the unpivoted value.
