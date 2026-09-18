# `sqlparser::ast::Statement::OptimizeTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.OptimizeTable.json).

<a id="op-9f3ac74faeb14afbd2f095f0"></a>
## deduplicate

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::deduplicate` · sqlparser 0.62.0

```rust
deduplicate: Option<Deduplicate>
```

Source: `src/ast/mod.rs:4789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional deduplication settings.
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-6a8e576d34f27188bdd52f5e"></a>
## has_table_keyword

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::has_table_keyword` · sqlparser 0.62.0

```rust
has_table_keyword: bool
```

Source: `src/ast/mod.rs:4777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `TABLE` keyword was present (ClickHouse uses `OPTIMIZE TABLE`, Databricks uses `OPTIMIZE`).

<a id="op-1b0b6f4433fc18f21cf11d4f"></a>
## include_final

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::include_final` · sqlparser 0.62.0

```rust
include_final: bool
```

Source: `src/ast/mod.rs:4786`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `FINAL` was specified.
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-f3f6d464e178d0114b7168e8"></a>
## name

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:4775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name to optimize.

<a id="op-578aa277630cf735d67396d6"></a>
## on_cluster

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::on_cluster` · sqlparser 0.62.0

```rust
on_cluster: Option<Ident>
```

Source: `src/ast/mod.rs:4780`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional cluster identifier.
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-e5fff792479173d92d46c37c"></a>
## partition

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::partition` · sqlparser 0.62.0

```rust
partition: Option<Partition>
```

Source: `src/ast/mod.rs:4783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition spec.
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-6f2e0406fc793710914b700d"></a>
## predicate

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::predicate` · sqlparser 0.62.0

```rust
predicate: Option<Expr>
```

Source: `src/ast/mod.rs:4792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional WHERE predicate.
[Databricks](https://docs.databricks.com/en/sql/language-manual/delta-optimize.html)

<a id="op-a0948255b202ab450309efe8"></a>
## zorder

`struct_field` · `sqlparser::ast::Statement::OptimizeTable::zorder` · sqlparser 0.62.0

```rust
zorder: Option<Vec<Expr>>
```

Source: `src/ast/mod.rs:4795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional ZORDER BY columns.
[Databricks](https://docs.databricks.com/en/sql/language-manual/delta-optimize.html)
