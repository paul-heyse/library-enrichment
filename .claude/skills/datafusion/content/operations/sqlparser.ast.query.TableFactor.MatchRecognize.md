# `sqlparser::ast::query::TableFactor::MatchRecognize`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.MatchRecognize.json).

<a id="op-ff47760704e9d91e9b6a9ebc"></a>
## after_match_skip

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::after_match_skip` · sqlparser 0.62.0

```rust
after_match_skip: Option<AfterMatchSkip>
```

Source: `src/ast/query.rs:1672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AFTER MATCH SKIP <option>`

<a id="op-f26ae1ae116396ce56fb7d6d"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias for the table.

<a id="op-f1b0543020bf71b169707962"></a>
## measures

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::measures` · sqlparser 0.62.0

```rust
measures: Vec<Measure>
```

Source: `src/ast/query.rs:1668`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MEASURES <expr> [AS] <alias> [, ... ]`

<a id="op-ca0f68d9f26a2015a815c7be"></a>
## order_by

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::order_by` · sqlparser 0.62.0

```rust
order_by: Vec<OrderByExpr>
```

Source: `src/ast/query.rs:1666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ORDER BY <expr> [, ... ]`

<a id="op-b88afc22f39c5c2bcfa143d0"></a>
## partition_by

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::partition_by` · sqlparser 0.62.0

```rust
partition_by: Vec<Expr>
```

Source: `src/ast/query.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PARTITION BY <expr> [, ... ]`

<a id="op-b361c31b50167f2bcdb508d3"></a>
## pattern

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::pattern` · sqlparser 0.62.0

```rust
pattern: MatchRecognizePattern
```

Source: `src/ast/query.rs:1674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PATTERN ( <pattern> )`

<a id="op-cc1b90d174ee81f86a0c3537"></a>
## rows_per_match

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::rows_per_match` · sqlparser 0.62.0

```rust
rows_per_match: Option<RowsPerMatch>
```

Source: `src/ast/query.rs:1670`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ONE ROW PER MATCH | ALL ROWS PER MATCH [ <option> ]`

<a id="op-2db87c870deabbbbcd6e62ba"></a>
## symbols

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::symbols` · sqlparser 0.62.0

```rust
symbols: Vec<SymbolDefinition>
```

Source: `src/ast/query.rs:1676`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DEFINE <symbol> AS <expr> [, ... ]`

<a id="op-3605c03a418ef123e6cd143f"></a>
## table

`struct_field` · `sqlparser::ast::query::TableFactor::MatchRecognize::table` · sqlparser 0.62.0

```rust
table: Box<TableFactor>
```

Source: `src/ast/query.rs:1662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The input table to apply `MATCH_RECOGNIZE` on.
