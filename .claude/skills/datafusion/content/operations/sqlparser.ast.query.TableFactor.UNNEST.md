# `sqlparser::ast::query::TableFactor::UNNEST`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.UNNEST.json).

<a id="op-0aa20cf8c2b23749d248a71a"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::UNNEST::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the UNNEST table (e.g. `UNNEST(...) AS t`).

<a id="op-bb81ad6bdc7b828db4d2060e"></a>
## array_exprs

`struct_field` · `sqlparser::ast::query::TableFactor::UNNEST::array_exprs` · sqlparser 0.62.0

```rust
array_exprs: Vec<Expr>
```

Source: `src/ast/query.rs:1544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expressions producing the arrays to be unnested.

<a id="op-9578ecafba1623d410d583bd"></a>
## with_offset

`struct_field` · `sqlparser::ast::query::TableFactor::UNNEST::with_offset` · sqlparser 0.62.0

```rust
with_offset: bool
```

Source: `src/ast/query.rs:1546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `WITH OFFSET` was specified to include element offsets.

<a id="op-e22a66215dc4a221b398a4b4"></a>
## with_offset_alias

`struct_field` · `sqlparser::ast::query::TableFactor::UNNEST::with_offset_alias` · sqlparser 0.62.0

```rust
with_offset_alias: Option<Ident>
```

Source: `src/ast/query.rs:1548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the offset column when `WITH OFFSET` is used.

<a id="op-577997a62ef8e3885c966d9d"></a>
## with_ordinality

`struct_field` · `sqlparser::ast::query::TableFactor::UNNEST::with_ordinality` · sqlparser 0.62.0

```rust
with_ordinality: bool
```

Source: `src/ast/query.rs:1550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `WITH ORDINALITY` was specified to include ordinality.
