# `sqlparser::ast::query::TableFactor::Table`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.Table.json).

<a id="op-a88ebbad4b0b30ff7f835db7"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::Table::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the table (e.g. `table AS t`).

<a id="op-3e87b48c0bc197f4c225dc9d"></a>
## args

`struct_field` · `sqlparser::ast::query::TableFactor::Table::args` · sqlparser 0.62.0

```rust
args: Option<TableFunctionArgs>
```

Source: `src/ast/query.rs:1478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arguments of a table-valued function, as supported by Postgres
and MSSQL. Note that deprecated MSSQL `FROM foo (NOLOCK)` syntax
will also be parsed as `args`.

This field's value is `Some(v)`, where `v` is a (possibly empty)
vector of arguments, in the case of a table-valued function call,
whereas it's `None` in the case of a regular table name.

<a id="op-85e69e7354efed5f5c550300"></a>
## index_hints

`struct_field` · `sqlparser::ast::query::TableFactor::Table::index_hints` · sqlparser 0.62.0

```rust
index_hints: Vec<TableIndexHints>
```

Source: `src/ast/query.rs:1497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional index hints(mysql)
See: <https://dev.mysql.com/doc/refman/8.4/en/index-hints.html>

<a id="op-0d98710de3b1c1b315bcc777"></a>
## json_path

`struct_field` · `sqlparser::ast::query::TableFactor::Table::json_path` · sqlparser 0.62.0

```rust
json_path: Option<JsonPath>
```

Source: `src/ast/query.rs:1491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional PartiQL JsonPath: <https://partiql.org/dql/from.html>

<a id="op-0a64c6c93d7f3e39cf748b15"></a>
## name

`struct_field` · `sqlparser::ast::query::TableFactor::Table::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/query.rs:1468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table or relation name.

<a id="op-ac689bf40bf731a81dc94a93"></a>
## partitions

`struct_field` · `sqlparser::ast::query::TableFactor::Table::partitions` · sqlparser 0.62.0

```rust
partitions: Vec<Ident>
```

Source: `src/ast/query.rs:1489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[Partition selection](https://dev.mysql.com/doc/refman/8.0/en/partitioning-selection.html), supported by MySQL.

<a id="op-909df6e387f0ae3ffb8ae9c9"></a>
## sample

`struct_field` · `sqlparser::ast::query::TableFactor::Table::sample` · sqlparser 0.62.0

```rust
sample: Option<TableSampleKind>
```

Source: `src/ast/query.rs:1494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table sample modifier
See: <https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#sample-clause>

<a id="op-ed8d786cd607b6020bb523fb"></a>
## version

`struct_field` · `sqlparser::ast::query::TableFactor::Table::version` · sqlparser 0.62.0

```rust
version: Option<TableVersion>
```

Source: `src/ast/query.rs:1483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional version qualifier to facilitate table time-travel, as
supported by BigQuery and MSSQL.

<a id="op-32e062bf4aebe2bd80edfbb2"></a>
## with_hints

`struct_field` · `sqlparser::ast::query::TableFactor::Table::with_hints` · sqlparser 0.62.0

```rust
with_hints: Vec<Expr>
```

Source: `src/ast/query.rs:1480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL-specific `WITH (...)` hints such as NOLOCK.

<a id="op-283be79d0fecff211589ffd4"></a>
## with_ordinality

`struct_field` · `sqlparser::ast::query::TableFactor::Table::with_ordinality` · sqlparser 0.62.0

```rust
with_ordinality: bool
```

Source: `src/ast/query.rs:1487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For example, `SELECT * FROM generate_series(1, 10) WITH ORDINALITY AS t(a, b);`
[WITH ORDINALITY](https://www.postgresql.org/docs/current/functions-srf.html), supported by Postgres.
