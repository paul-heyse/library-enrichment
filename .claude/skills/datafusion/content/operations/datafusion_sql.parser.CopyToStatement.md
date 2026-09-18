# `datafusion_sql::parser::CopyToStatement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.CopyToStatement.json).

<a id="op-de79c0d3ea66e0b1580efe39"></a>
## CopyToStatement

`struct` · `datafusion_sql::parser::CopyToStatement` · datafusion-sql 55.1.0

```rust
struct CopyToStatement
```

Source: `src/parser.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DataFusion extension DDL for `COPY`

# Syntax:

```text
COPY <table_name | (<query>)>
TO
<destination_url>
(key_value_list)
```

# Examples

```sql
COPY lineitem  TO 'lineitem'
STORED AS PARQUET (
  partitions 16,
  row_group_limit_rows 100000,
  row_group_limit_bytes 200000
)

COPY (SELECT l_orderkey from lineitem) to 'lineitem.parquet';
```

<a id="op-60252ffed0182493a31cb9d0"></a>
## clone

`function` · `datafusion_sql::parser::CopyToStatement::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> CopyToStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToStatement", "path": "CopyToStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 17], "end": [157, 22], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33e8a123adc987f291589d4b"></a>
## eq

`function` · `datafusion_sql::parser::CopyToStatement::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &CopyToStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToStatement", "path": "CopyToStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 24], "end": [157, 33], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9582ad1c108c1b2ffbbe6a1e"></a>
## fmt

`function` · `datafusion_sql::parser::CopyToStatement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToStatement", "path": "CopyToStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [198, 2], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb61da3ae493ae2a54abf7fd"></a>
## fmt

`function` · `datafusion_sql::parser::CopyToStatement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToStatement", "path": "CopyToStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 10], "end": [157, 15], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-286b0fd583c7adf62ef776bc"></a>
## options

`struct_field` · `datafusion_sql::parser::CopyToStatement::options` · datafusion-sql 55.1.0

```rust
options: Vec<(String, sqlparser::ast::Value)>
```

Source: `src/parser.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Target specific options

<a id="op-2f461653f123998393fd3b8d"></a>
## partitioned_by

`struct_field` · `datafusion_sql::parser::CopyToStatement::partitioned_by` · datafusion-sql 55.1.0

```rust
partitioned_by: Vec<String>
```

Source: `src/parser.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Partition keys

<a id="op-08d7550d08818bcfc115797d"></a>
## source

`struct_field` · `datafusion_sql::parser::CopyToStatement::source` · datafusion-sql 55.1.0

```rust
source: CopyToSource
```

Source: `src/parser.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

From where the data comes from

<a id="op-6347c24c63b2432d0c0fd7b2"></a>
## stored_as

`struct_field` · `datafusion_sql::parser::CopyToStatement::stored_as` · datafusion-sql 55.1.0

```rust
stored_as: Option<String>
```

Source: `src/parser.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

File type (Parquet, NDJSON, CSV etc.)

<a id="op-8e828cc6559003f7c9940252"></a>
## target

`struct_field` · `datafusion_sql::parser::CopyToStatement::target` · datafusion-sql 55.1.0

```rust
target: String
```

Source: `src/parser.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The URL to where the data is heading
