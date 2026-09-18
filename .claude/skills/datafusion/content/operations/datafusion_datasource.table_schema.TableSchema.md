# `datafusion_datasource::table_schema::TableSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.table_schema.TableSchema.json).

<a id="op-edcf381c15c6591908c8608e"></a>
## TableSchema

`struct` · `datafusion_datasource::table_schema::TableSchema` · datafusion-datasource 55.1.0

```rust
struct TableSchema
```

Source: `src/table_schema.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The overall schema for potentially partitioned data sources.

When reading partitioned data (such as Hive-style partitioning), a [`TableSchema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-edcf381c15c6591908c8608e)
consists of up to three parts:
1. **File schema**: The schema of the actual data files on disk
2. **Partition columns**: Columns whose values are encoded in the directory structure,
   but not stored in the files themselves
3. **Virtual columns**: Columns produced by the file reader (e.g. Parquet
   `row_number`) that are not stored in the files

The full table schema is composed in that order: file columns, then
partition columns, then virtual columns. Consumers that need a different
output ordering should use a projection on top of
[`TableSchema::table_schema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-d44cdfea14aa48a2665079e1).

# Example: Partitioned Table

Consider a table with the following directory structure:
```text
/data/date=2025-10-10/region=us-west/data.parquet
/data/date=2025-10-11/region=us-east/data.parquet
```

In this case:
- **File schema**: The schema of `data.parquet` files (e.g., `[user_id, amount]`)
- **Partition columns**: `[date, region]` extracted from the directory path
- **Table schema**: The full schema combining both (e.g., `[user_id, amount, date, region]`)

# When to Use

Use `TableSchema` when:
- Reading partitioned data sources (Parquet, CSV, etc. with Hive-style partitioning)
- You need to efficiently access different schema representations without reconstructing them
- You want to avoid repeatedly concatenating file and partition schemas

For non-partitioned data or when working with a single schema representation,
working directly with Arrow's `Schema` or `SchemaRef` is simpler.

# Performance

This struct pre-computes and caches the full table schema, allowing cheap references
to any representation without repeated allocations or reconstructions.

<a id="op-bcae8cd4c0f4d79bfeded60b"></a>
## builder

`function` · `datafusion_datasource::table_schema::TableSchema::builder` · datafusion-datasource 55.1.0

```rust
fn builder(file_schema: SchemaRef) -> TableSchemaBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Start building a [`TableSchema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-edcf381c15c6591908c8608e) from its (required) file schema.

Partition columns are optional and added with
[`TableSchemaBuilder::with_table_partition_cols`](../operations/datafusion_datasource.table_schema.TableSchemaBuilder.md#op-1708a4f87360b4c97fbe1586); the full table schema
is computed once by [`TableSchemaBuilder::build`](../operations/datafusion_datasource.table_schema.TableSchemaBuilder.md#op-74d96a3ee774430aafeed6a3). This is the preferred
way to construct a `TableSchema`.

# Example

```
# use std::sync::Arc;
# use arrow::datatypes::{Schema, Field, DataType};
# use datafusion_datasource::TableSchema;
let file_schema = Arc::new(Schema::new(vec![
    Field::new("user_id", DataType::Int64, false),
    Field::new("amount", DataType::Float64, false),
]));

let table_schema = TableSchema::builder(file_schema)
    .with_table_partition_cols(vec![
        Arc::new(Field::new("date", DataType::Utf8, false)),
        Arc::new(Field::new("region", DataType::Utf8, false)),
    ])
    .build();

// Table schema will have 4 columns: user_id, amount, date, region
assert_eq!(table_schema.table_schema().fields().len(), 4);
```

<a id="op-e9c2d5039f9af61159fad66e"></a>
## clone

`function` · `datafusion_datasource::table_schema::TableSchema::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 17], "end": [65, 22], "filename": "src/table_schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_schema.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4df68a9947942b5f05e16f17"></a>
## file_schema

`function` · `datafusion_datasource::table_schema::TableSchema::file_schema` · datafusion-datasource 55.1.0

```rust
fn file_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the file schema (without partition columns).

This is the schema of the actual data files on disk.

<a id="op-7fa00a39eb99a58c477fe7ba"></a>
## fmt

`function` · `datafusion_datasource::table_schema::TableSchema::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/table_schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_schema.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-605e3dbc16c25075ba8024f3"></a>
## from

`function` · `datafusion_datasource::table_schema::TableSchema::from` · datafusion-datasource 55.1.0

```rust
fn from(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [236, 2], "filename": "src/table_schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_schema.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad1737028877d2daa3cefb95"></a>
## from

`function` · `datafusion_datasource::table_schema::TableSchema::from` · datafusion-datasource 55.1.0

```rust
fn from(schema: &SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [242, 2], "filename": "src/table_schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_schema.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b762a8f755b9f6f16eeebdd"></a>
## from_file_schema

`function` · `datafusion_datasource::table_schema::TableSchema::from_file_schema` · datafusion-datasource 55.1.0

```rust
fn from_file_schema(file_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new TableSchema with no partition columns.

<a id="op-468d0c741f41b7c526f68a4f"></a>
## new

`function` · `datafusion_datasource::table_schema::TableSchema::new` · datafusion-datasource 55.1.0

```rust
fn new(file_schema: SchemaRef, table_partition_cols: Vec<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new TableSchema from a file schema and partition columns.

This is a convenience for
`TableSchema::builder(file_schema).with_table_partition_cols(cols).build()`.

<a id="op-b380e06628d8454e54e6ac3e"></a>
## schema_without_virtual_columns

`function` · `datafusion_datasource::table_schema::TableSchema::schema_without_virtual_columns` · datafusion-datasource 55.1.0

```rust
fn schema_without_virtual_columns(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Schema of columns that can be referenced by predicates pushed into the
file reader: file columns plus partition columns, excluding virtual
columns.

Virtual columns are produced by the reader itself (e.g. Parquet
`row_number`) and cannot be referenced inside the reader's row filter,
so predicates that reference them must stay above the scan. Callers
deciding which filters to push down should check against this schema
rather than [`Self::table_schema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-d44cdfea14aa48a2665079e1).

When there are no virtual columns this returns the same schema as
[`Self::table_schema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-d44cdfea14aa48a2665079e1).

<a id="op-4f933ba9c148699284538ca7"></a>
## table_partition_cols

`function` · `datafusion_datasource::table_schema::TableSchema::table_partition_cols` · datafusion-datasource 55.1.0

```rust
fn table_partition_cols(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the table partition columns.

These are the columns derived from the directory structure that
will be appended to each row during query execution.

<a id="op-d44cdfea14aa48a2665079e1"></a>
## table_schema

`function` · `datafusion_datasource::table_schema::TableSchema::table_schema` · datafusion-datasource 55.1.0

```rust
fn table_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the full table schema (file schema + partition columns + virtual columns).

This is the complete schema that will be seen by queries. Fields appear
in the order: file columns, partition columns, virtual columns.

<a id="op-4a2ec4983bfc1e8a768b2f22"></a>
## virtual_columns

`function` · `datafusion_datasource::table_schema::TableSchema::virtual_columns` · datafusion-datasource 55.1.0

```rust
fn virtual_columns(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the virtual columns.

Virtual columns are produced by the file reader (e.g. Parquet
`row_number`) and are not stored in the data files or derived from
partition paths.

<a id="op-6d31ec85a696858a778874b9"></a>
## with_table_partition_cols

`function` · `datafusion_datasource::table_schema::TableSchema::with_table_partition_cols` · datafusion-datasource 55.1.0

```rust
fn with_table_partition_cols(self, partition_cols: Vec<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchema", "path": "TableSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [230, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a new `TableSchema` with `partition_cols` as its partition columns,
replacing any existing ones. Existing virtual columns are preserved.
