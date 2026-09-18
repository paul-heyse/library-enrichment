# `datafusion_sql::parser::ExplainStatement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.ExplainStatement.json).

<a id="op-67bb76670880e06510ad5f91"></a>
## ExplainStatement

`struct` · `datafusion_sql::parser::ExplainStatement` · datafusion-sql 55.1.0

```rust
struct ExplainStatement
```

Source: `src/parser.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DataFusion specific `EXPLAIN`

Supports both the legacy keyword form and, on dialects whose
[`Dialect::supports_explain_with_utility_options`](../operations/sqlparser.dialect.Dialect.md#op-805589ff8e3b6b01fe9b2df2) returns `true`
(PostgreSQL, DuckDB, etc.), the Postgres-style parenthesized option list:

```sql
-- Legacy keyword form (any dialect)
EXPLAIN <ANALYZE> <VERBOSE> [FORMAT format] statement

-- Postgres-style option form (dialect-gated)
EXPLAIN (option [arg] [, ...]) statement
```

See [`ExplainStatementOptions`](../operations/datafusion_common.format.ExplainStatementOptions.md#op-aa41302c6d233871885d8cce) for the list of supported options in the
parenthesized form.

<a id="op-f17ba9f40aac501a4f2211e2"></a>
## clone

`function` · `datafusion_sql::parser::ExplainStatement::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> ExplainStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ExplainStatement", "path": "ExplainStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 17], "end": [74, 22], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9901f9e48690bc4bd758cf20"></a>
## eq

`function` · `datafusion_sql::parser::ExplainStatement::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &ExplainStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ExplainStatement", "path": "ExplainStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 24], "end": [74, 33], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e36b4eb1eba639318fc2fed8"></a>
## fmt

`function` · `datafusion_sql::parser::ExplainStatement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ExplainStatement", "path": "ExplainStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f64d8f69bfb0724c72f49424"></a>
## fmt

`function` · `datafusion_sql::parser::ExplainStatement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ExplainStatement", "path": "ExplainStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [132, 2], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e231f43d764f80059f5db6d4"></a>
## options

`struct_field` · `datafusion_sql::parser::ExplainStatement::options` · datafusion-sql 55.1.0

```rust
options: datafusion_common::format::ExplainStatementOptions
```

Source: `src/parser.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Normalized options parsed from either the legacy keyword form or the
parenthesized option list.

<a id="op-4d999cf8456a493e4a8b6e35"></a>
## statement

`struct_field` · `datafusion_sql::parser::ExplainStatement::statement` · datafusion-sql 55.1.0

```rust
statement: Box<Statement>
```

Source: `src/parser.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The statement to analyze. Note this is a DataFusion [`Statement`](../operations/datafusion_sql.parser.Statement.md#op-c2acea2bc56287d845c8d8f4) (not a
[`sqlparser::ast::Statement`](../operations/sqlparser.ast.Statement.md#op-a6bf150ce3eb740dc90fe9e6) so that we can use `EXPLAIN`, `COPY`, and other
DataFusion specific statements
