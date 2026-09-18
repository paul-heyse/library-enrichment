# `datafusion_sql::parser::CreateExternalTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.CreateExternalTable.json).

<a id="op-4858c5118fd28c08f69db6bf"></a>
## CreateExternalTable

`struct` · `datafusion_sql::parser::CreateExternalTable` · datafusion-sql 55.1.0

```rust
struct CreateExternalTable
```

Source: `src/parser.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DataFusion extension DDL for `CREATE EXTERNAL TABLE`

Syntax:

```text
CREATE
[ OR REPLACE ]
EXTERNAL TABLE
[ IF NOT EXISTS ]
<TABLE_NAME>[ (<column_definition>) ]
STORED AS <file_type>
[ PARTITIONED BY (<column_definition list> | <column list>) ]
[ WITH ORDER (<ordered column list>)
[ OPTIONS (<key_value_list>) ]
LOCATION <literal> | LOCATION (<literal>[, ...])

<column_definition> := (<column_name> <data_type>, ...)

<column_list> := (<column_name>, ...)

<ordered_column_list> := (<column_name> <sort_clause>, ...)

<key_value_list> := (<literal> <literal, <literal> <literal>, ...)
```

<a id="op-6782b881652bbc0007e2fced"></a>
## clone

`function` · `datafusion_sql::parser::CreateExternalTable::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> CreateExternalTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 17], "end": [244, 22], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a02f244248f6f06acbae23c"></a>
## columns

`struct_field` · `datafusion_sql::parser::CreateExternalTable::columns` · datafusion-sql 55.1.0

```rust
columns: Vec<sqlparser::ast::ColumnDef>
```

Source: `src/parser.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Optional schema

<a id="op-8828ad1049d0a3f698d5b4f2"></a>
## constraints

`struct_field` · `datafusion_sql::parser::CreateExternalTable::constraints` · datafusion-sql 55.1.0

```rust
constraints: Vec<sqlparser::ast::TableConstraint>
```

Source: `src/parser.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

A table-level constraint

<a id="op-91c8969961eba1b51cfe6046"></a>
## eq

`function` · `datafusion_sql::parser::CreateExternalTable::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &CreateExternalTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 24], "end": [244, 33], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b90a52cf28c2f857ba12bd1d"></a>
## file_type

`struct_field` · `datafusion_sql::parser::CreateExternalTable::file_type` · datafusion-sql 55.1.0

```rust
file_type: String
```

Source: `src/parser.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

File type (Parquet, NDJSON, CSV, etc)

<a id="op-204c4ea62c42878cc1b7ee2b"></a>
## fmt

`function` · `datafusion_sql::parser::CreateExternalTable::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 1], "end": [310, 2], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e98551fab155461fd978c05c"></a>
## fmt

`function` · `datafusion_sql::parser::CreateExternalTable::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 10], "end": [244, 15], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4e6c2d9be44d810f13d295b"></a>
## if_not_exists

`struct_field` · `datafusion_sql::parser::CreateExternalTable::if_not_exists` · datafusion-sql 55.1.0

```rust
if_not_exists: bool
```

Source: `src/parser.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Option to not error if table already exists

<a id="op-b19911e93134531eada1d8dd"></a>
## locations

`struct_field` · `datafusion_sql::parser::CreateExternalTable::locations` · datafusion-sql 55.1.0

```rust
locations: Vec<String>
```

Source: `src/parser.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Paths to files

<a id="op-94ad8b50799532d166a79144"></a>
## name

`struct_field` · `datafusion_sql::parser::CreateExternalTable::name` · datafusion-sql 55.1.0

```rust
name: sqlparser::ast::ObjectName
```

Source: `src/parser.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Table name

<a id="op-85e055f2b9b57f1d30999f53"></a>
## options

`struct_field` · `datafusion_sql::parser::CreateExternalTable::options` · datafusion-sql 55.1.0

```rust
options: Vec<(String, sqlparser::ast::Value)>
```

Source: `src/parser.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Table(provider) specific options

<a id="op-97f51d9c40a5d857da0b153f"></a>
## or_replace

`struct_field` · `datafusion_sql::parser::CreateExternalTable::or_replace` · datafusion-sql 55.1.0

```rust
or_replace: bool
```

Source: `src/parser.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Option to replace table content if table already exists

<a id="op-da9e514491fd61042157c424"></a>
## order_exprs

`struct_field` · `datafusion_sql::parser::CreateExternalTable::order_exprs` · datafusion-sql 55.1.0

```rust
order_exprs: Vec<Vec<sqlparser::ast::OrderByExpr>>
```

Source: `src/parser.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Ordered expressions

<a id="op-26183aada9f06b2cef74ec74"></a>
## table_partition_cols

`struct_field` · `datafusion_sql::parser::CreateExternalTable::table_partition_cols` · datafusion-sql 55.1.0

```rust
table_partition_cols: Vec<String>
```

Source: `src/parser.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Partition Columns

<a id="op-808dd3ccc089d0bf06d15bf6"></a>
## temporary

`struct_field` · `datafusion_sql::parser::CreateExternalTable::temporary` · datafusion-sql 55.1.0

```rust
temporary: bool
```

Source: `src/parser.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether the table is a temporary table

<a id="op-8befbe13a0c0bab150b64fd3"></a>
## unbounded

`struct_field` · `datafusion_sql::parser::CreateExternalTable::unbounded` · datafusion-sql 55.1.0

```rust
unbounded: bool
```

Source: `src/parser.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Infinite streams?
