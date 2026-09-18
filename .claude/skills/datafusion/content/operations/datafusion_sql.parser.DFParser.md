# `datafusion_sql::parser::DFParser`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.DFParser.json).

<a id="op-c4bfa9a3c20364cfab8a7ca0"></a>
## DFParser

`struct` · `datafusion_sql::parser::DFParser` · datafusion-sql 55.1.0

```rust
struct DFParser<'a>
```

Source: `src/parser.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DataFusion SQL Parser based on [`sqlparser`](../modules/sqlparser.md#op-209408c3e0722e98219d2290)

Parses DataFusion's SQL dialect, often delegating to [`sqlparser`](../modules/sqlparser.md#op-209408c3e0722e98219d2290)'s [`Parser`](../operations/sqlparser.parser.Parser.md#op-b761887fc90688e669ac5e12).

DataFusion mostly follows existing SQL dialects via
`sqlparser`. However, certain statements such as `COPY` and
`CREATE EXTERNAL TABLE` have special syntax in DataFusion. See
[`Statement`](../operations/datafusion_sql.parser.Statement.md#op-c2acea2bc56287d845c8d8f4) for a list of this special syntax

<a id="op-128724a91627dd18ccf7cd73"></a>
## parse_copy

`function` · `datafusion_sql::parser::DFParser::parse_copy` · datafusion-sql 55.1.0

```rust
fn parse_copy(&mut self) -> Result<Statement, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:692`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a SQL `COPY TO` statement

<a id="op-101f41ecbed3e89d61cf35f5"></a>
## parse_create

`function` · `datafusion_sql::parser::DFParser::parse_create` · datafusion-sql 55.1.0

```rust
fn parse_create(&mut self) -> Result<Statement, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:926`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a SQL `CREATE` statement handling `CREATE EXTERNAL TABLE`

<a id="op-ce64ab199f3fe8c5ba25fbab"></a>
## parse_explain

`function` · `datafusion_sql::parser::DFParser::parse_explain` · datafusion-sql 55.1.0

```rust
fn parse_explain(&mut self) -> Result<Statement, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:832`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a SQL `EXPLAIN`

After the `EXPLAIN` keyword, if the dialect supports the Postgres-style
option list and the next non-whitespace token is `(`, we must
disambiguate between an option list (`EXPLAIN (ANALYZE) SELECT ...`)
and a parenthesized query (`EXPLAIN (SELECT ...)` or
`EXPLAIN (q1 EXCEPT q2) UNION ALL ...`).

<a id="op-d9bf639fc300a4017999a060"></a>
## parse_explain_format

`function` · `datafusion_sql::parser::DFParser::parse_explain_format` · datafusion-sql 55.1.0

```rust
fn parse_explain_format(&mut self) -> Result<Option<String>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:910`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4f2ead38999117cb5599dbb"></a>
## parse_expr

`function` · `datafusion_sql::parser::DFParser::parse_expr` · datafusion-sql 55.1.0

```rust
fn parse_expr(&mut self) -> Result<ExprWithAlias, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:651`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0753ea780d93e0c45eec845c"></a>
## parse_into_expr

`function` · `datafusion_sql::parser::DFParser::parse_into_expr` · datafusion-sql 55.1.0

```rust
fn parse_into_expr(&mut self) -> Result<ExprWithAlias, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:668`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parses the entire SQL string into an expression.

In contrast to [`DFParser::parse_expr`](../operations/datafusion_sql.parser.DFParser.md#op-f4f2ead38999117cb5599dbb), this function will report an error if the input
contains any trailing, unparsed tokens.

<a id="op-042d97cfa1c6b97d61399788"></a>
## parse_option_key

`function` · `datafusion_sql::parser::DFParser::parse_option_key` · datafusion-sql 55.1.0

```rust
fn parse_option_key(&mut self) -> Result<String, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:781`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse the next token as a key name for an option list

Note this is different than [`parse_literal_string`]
because it allows keywords as well as other non words

[`parse_literal_string`]: sqlparser::parser::Parser::parse_literal_string

Unresolved upstream links (retained, not inferred): `sqlparser::parser::Parser::parse_literal_string`.

<a id="op-adfe8900a3f6307cbe85ed34"></a>
## parse_option_value

`function` · `datafusion_sql::parser::DFParser::parse_option_value` · datafusion-sql 55.1.0

```rust
fn parse_option_value(&mut self) -> Result<Value, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:812`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse the next token as a value for an option list

Note this is different than [`parse_value`] as it allows any
word or keyword in this location.

[`parse_value`]: sqlparser::parser::Parser::parse_value

Unresolved upstream links (retained, not inferred): `sqlparser::parser::Parser::parse_value`.

<a id="op-9cb1f2699a99c962f2dce80b"></a>
## parse_order_by_expr

`function` · `datafusion_sql::parser::DFParser::parse_order_by_expr` · datafusion-sql 55.1.0

```rust
fn parse_order_by_expr(&mut self) -> Result<OrderByExpr, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:995`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse an ORDER BY sub-expression optionally followed by ASC or DESC.

<a id="op-7e2c7228f6f14024496b6971"></a>
## parse_order_by_exprs

`function` · `datafusion_sql::parser::DFParser::parse_order_by_exprs` · datafusion-sql 55.1.0

```rust
fn parse_order_by_exprs(&mut self) -> Result<Vec<OrderByExpr>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse the ordering clause of a `CREATE EXTERNAL TABLE` SQL statement

<a id="op-f841173aa3721b54451b000e"></a>
## parse_reset

`function` · `datafusion_sql::parser::DFParser::parse_reset` · datafusion-sql 55.1.0

```rust
fn parse_reset(&mut self) -> Result<Statement, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:870`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a SQL `RESET`

<a id="op-a3b0dff9c5d2d0ff9cd41c96"></a>
## parse_sql

`function` · `datafusion_sql::parser::DFParser::parse_sql` · datafusion-sql 55.1.0

```rust
fn parse_sql(sql: &'a str) -> Result<VecDeque<Statement>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a sql string into one or [`Statement`](../operations/datafusion_sql.parser.Statement.md#op-c2acea2bc56287d845c8d8f4)s using the
[`GenericDialect`](../operations/sqlparser.dialect.generic.GenericDialect.md#op-e9e98e2bf43b55c23c54c302).

<a id="op-edffa3c4b4f381e36c9f1a5c"></a>
## parse_sql_into_expr

`function` · `datafusion_sql::parser::DFParser::parse_sql_into_expr` · datafusion-sql 55.1.0

```rust
fn parse_sql_into_expr(sql: &str) -> Result<ExprWithAlias, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:541`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c653203e3c835e312a2eb167"></a>
## parse_sql_into_expr_with_dialect

`function` · `datafusion_sql::parser::DFParser::parse_sql_into_expr_with_dialect` · datafusion-sql 55.1.0

```rust
fn parse_sql_into_expr_with_dialect(sql: &str, dialect: &dyn Dialect) -> Result<ExprWithAlias, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:545`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e4212e0c26bb102d2c867dd"></a>
## parse_sql_with_dialect

`function` · `datafusion_sql::parser::DFParser::parse_sql_with_dialect` · datafusion-sql 55.1.0

```rust
fn parse_sql_with_dialect(sql: &str, dialect: &dyn Dialect) -> Result<VecDeque<Statement>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:533`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a SQL string and produce one or more [`Statement`](../operations/datafusion_sql.parser.Statement.md#op-c2acea2bc56287d845c8d8f4)s with
with the specified dialect.

<a id="op-81d421d0949eeb2e6d92a0ff"></a>
## parse_statement

`function` · `datafusion_sql::parser::DFParser::parse_statement` · datafusion-sql 55.1.0

```rust
fn parse_statement(&mut self) -> Result<Statement, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a new expression

<a id="op-cb2ae0aa0202f86966731d6c"></a>
## parse_statements

`function` · `datafusion_sql::parser::DFParser::parse_statements` · datafusion-sql 55.1.0

```rust
fn parse_statements(&mut self) -> Result<VecDeque<Statement>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParser", "path": "DFParser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [1302, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:556`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Parse a sql string into one or [`Statement`](../operations/datafusion_sql.parser.Statement.md#op-c2acea2bc56287d845c8d8f4)s

<a id="op-8152226c1b232bd9d59a86ff"></a>
## parser

`struct_field` · `datafusion_sql::parser::DFParser::parser` · datafusion-sql 55.1.0

```rust
parser: sqlparser::parser::Parser<'a>
```

Source: `src/parser.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
