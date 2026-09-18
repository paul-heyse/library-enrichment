# `sqlparser::ast::Statement::ExplainTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.ExplainTable.json).

<a id="op-c1c5634652bee9a2fa620173"></a>
## describe_alias

`struct_field` · `sqlparser::ast::Statement::ExplainTable::describe_alias` · sqlparser 0.62.0

```rust
describe_alias: DescribeAlias
```

Source: `src/ast/mod.rs:4587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXPLAIN | DESC | DESCRIBE`

<a id="op-40536bdadc442898a34dfd73"></a>
## has_table_keyword

`struct_field` · `sqlparser::ast::Statement::ExplainTable::has_table_keyword` · sqlparser 0.62.0

```rust
has_table_keyword: bool
```

Source: `src/ast/mod.rs:4594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake and ClickHouse support `DESC|DESCRIBE TABLE <table_name>` syntax

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/desc-table.html)
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/describe-table)

<a id="op-87b5bf9e62c0893d5b8b6f00"></a>
## hive_format

`struct_field` · `sqlparser::ast::Statement::ExplainTable::hive_format` · sqlparser 0.62.0

```rust
hive_format: Option<HiveDescribeFormat>
```

Source: `src/ast/mod.rs:4589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive style `FORMATTED | EXTENDED`

<a id="op-fc443ff839753a8e39d4eb27"></a>
## table_name

`struct_field` · `sqlparser::ast::Statement::ExplainTable::table_name` · sqlparser 0.62.0

```rust
table_name: ObjectName
```

Source: `src/ast/mod.rs:4597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name
