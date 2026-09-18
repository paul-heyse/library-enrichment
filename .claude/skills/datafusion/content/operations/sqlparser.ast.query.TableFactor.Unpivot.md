# `sqlparser::ast::query::TableFactor::Unpivot`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.Unpivot.json).

<a id="op-96c0b1869a6229a68badcf57"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::Unpivot::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the resulting table.

<a id="op-aea8dd4d4d409cb74b1b42d2"></a>
## columns

`struct_field` · `sqlparser::ast::query::TableFactor::Unpivot::columns` · sqlparser 0.62.0

```rust
columns: Vec<ExprWithAlias>
```

Source: `src/ast/query.rs:1651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns or expressions to unpivot, optionally aliased.

<a id="op-6e9ae96899135146523706b6"></a>
## name

`struct_field` · `sqlparser::ast::query::TableFactor::Unpivot::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:1649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifier used for the generated column name.

<a id="op-a448f8161bde53851b6ba21d"></a>
## null_inclusion

`struct_field` · `sqlparser::ast::query::TableFactor::Unpivot::null_inclusion` · sqlparser 0.62.0

```rust
null_inclusion: Option<NullInclusion>
```

Source: `src/ast/query.rs:1653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to include or exclude NULLs during unpivot.

<a id="op-960b4de77895c7ecdc141638"></a>
## table

`struct_field` · `sqlparser::ast::query::TableFactor::Unpivot::table` · sqlparser 0.62.0

```rust
table: Box<TableFactor>
```

Source: `src/ast/query.rs:1645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The input table to unpivot.

<a id="op-3c631193921800bc42e907b2"></a>
## value

`struct_field` · `sqlparser::ast::query::TableFactor::Unpivot::value` · sqlparser 0.62.0

```rust
value: Expr
```

Source: `src/ast/query.rs:1647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression producing the unpivoted value.
