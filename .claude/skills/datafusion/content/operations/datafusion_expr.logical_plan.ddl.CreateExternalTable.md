# `datafusion_expr::logical_plan::ddl::CreateExternalTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateExternalTable.json).

<a id="op-4041a48d736315b98cb3dab6"></a>
## CreateExternalTable

`struct` · `datafusion_expr::logical_plan::ddl::CreateExternalTable` · datafusion-expr 55.1.0

```rust
struct CreateExternalTable
```

Source: `src/logical_plan/ddl.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an external table.

<a id="op-7b032ed55223befa88abf7bc"></a>
## builder

`function` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::builder` · datafusion-expr 55.1.0

```rust
fn builder(name: impl Into<TableReference>, location: impl Into<String>, file_type: impl Into<String>, schema: DFSchemaRef) -> CreateExternalTableBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [288, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/ddl.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a builder for [`CreateExternalTable`](../operations/datafusion_expr.logical_plan.ddl.CreateExternalTable.md#op-4041a48d736315b98cb3dab6) with required fields.

# Arguments
* `name` - The table name
* `location` - The physical location of the table files
* `file_type` - The file type (e.g., "parquet", "csv", "json")
* `schema` - The table schema

# Example
```
# use datafusion_expr::CreateExternalTable;
# use datafusion_common::{DFSchema, TableReference};
# use std::sync::Arc;
let table = CreateExternalTable::builder(
    TableReference::bare("my_table"),
    "/path/to/data",
    "parquet",
    Arc::new(DFSchema::empty())
).build();
```

<a id="op-f9cf72b970993b1e5c073dcb"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateExternalTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 17], "end": [208, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-065e4dbeff136ac245ab09d4"></a>
## column_defaults

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::column_defaults` · datafusion-expr 55.1.0

```rust
column_defaults: std::collections::HashMap<String, Expr>
```

Source: `src/logical_plan/ddl.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Default values for columns

<a id="op-8adf897727caaa64464e8eb7"></a>
## constraints

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::constraints` · datafusion-expr 55.1.0

```rust
constraints: datafusion_common::Constraints
```

Source: `src/logical_plan/ddl.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The list of constraints in the schema, such as primary key, unique, etc.

<a id="op-a40f23e5ff854d4a5347476e"></a>
## definition

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::definition` · datafusion-expr 55.1.0

```rust
definition: Option<String>
```

Source: `src/logical_plan/ddl.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

SQL used to create the table, if available

<a id="op-ffeefaff9b1a4e6967724007"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateExternalTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 24], "end": [208, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-549da08c7910ba4ffbae72bf"></a>
## file_type

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::file_type` · datafusion-expr 55.1.0

```rust
file_type: String
```

Source: `src/logical_plan/ddl.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The file type of physical file

<a id="op-baf817d87272dccffccd30ca"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 10], "end": [208, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96925e63825b77fd861919f8"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [407, 1], "end": [420, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cf5de11364c47cca8d23a19"></a>
## if_not_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::if_not_exists` · datafusion-expr 55.1.0

```rust
if_not_exists: bool
```

Source: `src/logical_plan/ddl.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Option to not error if table already exists

<a id="op-c932a1d261c86fa14b15aca4"></a>
## locations

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::locations` · datafusion-expr 55.1.0

```rust
locations: Vec<String>
```

Source: `src/logical_plan/ddl.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The physical locations of the table files.

More than one location may be supplied (for example
`CREATE EXTERNAL TABLE ... LOCATION ('a.parquet', 'b.parquet')`), in which
case the files are read together as a single table.

<a id="op-53a6bee9387e5d9c76a934aa"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::name` · datafusion-expr 55.1.0

```rust
name: datafusion_common::TableReference
```

Source: `src/logical_plan/ddl.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table name

<a id="op-48807eb2f4b736440e2444ca"></a>
## options

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::options` · datafusion-expr 55.1.0

```rust
options: std::collections::HashMap<String, String>
```

Source: `src/logical_plan/ddl.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Table(provider) specific options

<a id="op-5e4ed0693042bb58e6944cc2"></a>
## or_replace

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::or_replace` · datafusion-expr 55.1.0

```rust
or_replace: bool
```

Source: `src/logical_plan/ddl.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Option to replace table content if table already exists

<a id="op-f70f372233dc19d968a94ea8"></a>
## order_exprs

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::order_exprs` · datafusion-expr 55.1.0

```rust
order_exprs: Vec<Vec<expr::Sort>>
```

Source: `src/logical_plan/ddl.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Order expressions supplied by user

<a id="op-a6f3039d3d45c7a2b628796d"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateExternalTable", "path": "CreateExternalTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [424, 1], "end": [474, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f522c1937c8aec58b56504db"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table schema

<a id="op-9b10040ef6effd59bdc253aa"></a>
## table_partition_cols

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::table_partition_cols` · datafusion-expr 55.1.0

```rust
table_partition_cols: Vec<String>
```

Source: `src/logical_plan/ddl.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Partition Columns

<a id="op-bd3125ebab7d4622662fe306"></a>
## temporary

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::temporary` · datafusion-expr 55.1.0

```rust
temporary: bool
```

Source: `src/logical_plan/ddl.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the table is a temporary table

<a id="op-d0fa6f88437f009318a4480f"></a>
## unbounded

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateExternalTable::unbounded` · datafusion-expr 55.1.0

```rust
unbounded: bool
```

Source: `src/logical_plan/ddl.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the table is an infinite streams
