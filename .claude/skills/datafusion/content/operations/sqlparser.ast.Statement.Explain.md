# `sqlparser::ast::Statement::Explain`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Explain.json).

<a id="op-5038c5e22556b722790a102d"></a>
## analyze

`struct_field` · `sqlparser::ast::Statement::Explain::analyze` · sqlparser 0.62.0

```rust
analyze: bool
```

Source: `src/ast/mod.rs:4606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Carry out the command and show actual run times and other statistics.

<a id="op-01db570e04043372b4662737"></a>
## describe_alias

`struct_field` · `sqlparser::ast::Statement::Explain::describe_alias` · sqlparser 0.62.0

```rust
describe_alias: DescribeAlias
```

Source: `src/ast/mod.rs:4604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXPLAIN | DESC | DESCRIBE`

<a id="op-a3f716789373153230128db4"></a>
## estimate

`struct_field` · `sqlparser::ast::Statement::Explain::estimate` · sqlparser 0.62.0

```rust
estimate: bool
```

Source: `src/ast/mod.rs:4616`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXPLAIN ESTIMATE`
[Clickhouse](https://clickhouse.com/docs/en/sql-reference/statements/explain#explain-estimate)

<a id="op-250729b15315e432c02e5292"></a>
## format

`struct_field` · `sqlparser::ast::Statement::Explain::format` · sqlparser 0.62.0

```rust
format: Option<AnalyzeFormatKind>
```

Source: `src/ast/mod.rs:4620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional output format of explain

<a id="op-e62ff6e93333fde22fc24cca"></a>
## options

`struct_field` · `sqlparser::ast::Statement::Explain::options` · sqlparser 0.62.0

```rust
options: Option<Vec<UtilityOption>>
```

Source: `src/ast/mod.rs:4622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres style utility options, `(analyze, verbose true)`

<a id="op-c489dc5008216c8589aaaff1"></a>
## query_plan

`struct_field` · `sqlparser::ast::Statement::Explain::query_plan` · sqlparser 0.62.0

```rust
query_plan: bool
```

Source: `src/ast/mod.rs:4613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXPLAIN QUERY PLAN`
Display the query plan without running the query.

[SQLite](https://sqlite.org/lang_explain.html)

<a id="op-8e5657c602c4b874cecec62b"></a>
## statement

`struct_field` · `sqlparser::ast::Statement::Explain::statement` · sqlparser 0.62.0

```rust
statement: Box<Statement>
```

Source: `src/ast/mod.rs:4618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A SQL query that specifies what to explain

<a id="op-fbbdf40bae524db501fd7bfc"></a>
## verbose

`struct_field` · `sqlparser::ast::Statement::Explain::verbose` · sqlparser 0.62.0

```rust
verbose: bool
```

Source: `src/ast/mod.rs:4608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Display additional information regarding the plan.
