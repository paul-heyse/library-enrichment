# `sqlparser::ast::query::PipeOperator::Pivot`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Pivot.json).

<a id="op-c59b335299df361fa0c3ec8e"></a>
## aggregate_functions

`struct_field` · `sqlparser::ast::query::PipeOperator::Pivot::aggregate_functions` · sqlparser 0.62.0

```rust
aggregate_functions: Vec<ExprWithAlias>
```

Source: `src/ast/query.rs:3312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Aggregate functions to compute during pivot.

<a id="op-c5deecff3554c6fa5f47f6c3"></a>
## alias

`struct_field` · `sqlparser::ast::query::PipeOperator::Pivot::alias` · sqlparser 0.62.0

```rust
alias: Option<Ident>
```

Source: `src/ast/query.rs:3318`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the output.

<a id="op-3348e8480931cf8d5e552637"></a>
## value_column

`struct_field` · `sqlparser::ast::query::PipeOperator::Pivot::value_column` · sqlparser 0.62.0

```rust
value_column: Vec<Ident>
```

Source: `src/ast/query.rs:3314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column(s) that provide the pivot values.

<a id="op-0a5c3b09d893ec82c0662212"></a>
## value_source

`struct_field` · `sqlparser::ast::query::PipeOperator::Pivot::value_source` · sqlparser 0.62.0

```rust
value_source: PivotValueSource
```

Source: `src/ast/query.rs:3316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The source of pivot values (literal list or subquery).
