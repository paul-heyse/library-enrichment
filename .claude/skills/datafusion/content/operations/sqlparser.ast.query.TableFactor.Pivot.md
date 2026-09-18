# `sqlparser::ast::query::TableFactor::Pivot`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.Pivot.json).

<a id="op-2ef23627d88cf5474e827284"></a>
## aggregate_functions

`struct_field` · `sqlparser::ast::query::TableFactor::Pivot::aggregate_functions` · sqlparser 0.62.0

```rust
aggregate_functions: Vec<ExprWithAlias>
```

Source: `src/ast/query.rs:1622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Aggregate expressions used as pivot values (optionally aliased).

<a id="op-b15976ad41111549e99b7750"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::Pivot::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1630`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the pivoted table.

<a id="op-7084cdc5d92fed6dbaf0c150"></a>
## default_on_null

`struct_field` · `sqlparser::ast::query::TableFactor::Pivot::default_on_null` · sqlparser 0.62.0

```rust
default_on_null: Option<Expr>
```

Source: `src/ast/query.rs:1628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional expression providing a default when a pivot produces NULL.

<a id="op-c6b03b3039ea55a967b4a4fe"></a>
## table

`struct_field` · `sqlparser::ast::query::TableFactor::Pivot::table` · sqlparser 0.62.0

```rust
table: Box<TableFactor>
```

Source: `src/ast/query.rs:1620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The input table to pivot.

<a id="op-f12088f5e6444b5784c5de25"></a>
## value_column

`struct_field` · `sqlparser::ast::query::TableFactor::Pivot::value_column` · sqlparser 0.62.0

```rust
value_column: Vec<Expr>
```

Source: `src/ast/query.rs:1624`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns producing the values to be pivoted.

<a id="op-5080dc925b7efa403cb75ddb"></a>
## value_source

`struct_field` · `sqlparser::ast::query::TableFactor::Pivot::value_source` · sqlparser 0.62.0

```rust
value_source: PivotValueSource
```

Source: `src/ast/query.rs:1626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Source of pivot values (e.g. list of literals or columns).
