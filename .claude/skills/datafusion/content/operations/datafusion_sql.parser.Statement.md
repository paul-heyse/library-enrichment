# `datafusion_sql::parser::Statement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.Statement.json).

<a id="op-c2acea2bc56287d845c8d8f4"></a>
## Statement

`enum` · `datafusion_sql::parser::Statement` · datafusion-sql 55.1.0

```rust
enum Statement
```

Source: `src/parser.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DataFusion SQL Statement.

This can either be a [`Statement`] from [`sqlparser`](../modules/sqlparser.md#op-209408c3e0722e98219d2290) from a
standard SQL dialect, or a DataFusion extension such as `CREATE
EXTERNAL TABLE`. See [`DFParser`](../operations/datafusion_sql.parser.DFParser.md#op-c4bfa9a3c20364cfab8a7ca0) for more information.

[`Statement`]: sqlparser::ast::Statement

<a id="op-fd18a8482a1eac5b1e7eda7a"></a>
## CopyTo

`variant` · `datafusion_sql::parser::Statement::CopyTo` · datafusion-sql 55.1.0

```rust
CopyTo
```

Source: `src/parser.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Extension: `COPY TO`

<a id="op-5296ffc259da9dc1bb0d1d10"></a>
## CreateExternalTable

`variant` · `datafusion_sql::parser::Statement::CreateExternalTable` · datafusion-sql 55.1.0

```rust
CreateExternalTable
```

Source: `src/parser.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Extension: `CREATE EXTERNAL TABLE`

<a id="op-36f47b226f7cf0c4aaf1ea38"></a>
## Explain

`variant` · `datafusion_sql::parser::Statement::Explain` · datafusion-sql 55.1.0

```rust
Explain
```

Source: `src/parser.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

EXPLAIN for extensions

<a id="op-66ff3d806edae27d456adc1c"></a>
## Reset

`variant` · `datafusion_sql::parser::Statement::Reset` · datafusion-sql 55.1.0

```rust
Reset
```

Source: `src/parser.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Extension: `RESET`

<a id="op-690224118cdb022cc8253a95"></a>
## Statement

`variant` · `datafusion_sql::parser::Statement::Statement` · datafusion-sql 55.1.0

```rust
Statement
```

Source: `src/parser.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

ANSI SQL AST node (from sqlparser-rs)

<a id="op-505adea35b3a963310a6d78a"></a>
## clone

`function` · `datafusion_sql::parser::Statement::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> Statement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [334, 17], "end": [334, 22], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b7cad3cc3ce3cf8a807eabe"></a>
## eq

`function` · `datafusion_sql::parser::Statement::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &Statement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [334, 24], "end": [334, 33], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4c568ba223569b486eb7bc2"></a>
## fmt

`function` · `datafusion_sql::parser::Statement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [334, 10], "end": [334, 15], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcd2a0f2d14d69f871847564"></a>
## fmt

`function` · `datafusion_sql::parser::Statement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [358, 2], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
