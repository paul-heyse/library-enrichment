# `datafusion_datasource::table_schema`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.table_schema.json`](../model/datafusion_datasource.table_schema.json)

## TableSchema

`struct` · `datafusion_datasource::table_schema::TableSchema`

Also reachable as `datafusion::datasource::table_schema::TableSchema`, `datafusion_datasource::TableSchema`

```rust
struct TableSchema
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn builder(file_schema: SchemaRef) -> TableSchemaBuilder
fn file_schema(&self) -> &SchemaRef
fn from_file_schema(file_schema: SchemaRef) -> Self
fn new(file_schema: SchemaRef, table_partition_cols: Vec<FieldRef>) -> Self
fn schema_without_virtual_columns(&self) -> &SchemaRef
fn table_partition_cols(&self) -> &Fields
fn table_schema(&self) -> &SchemaRef
fn virtual_columns(&self) -> &Fields
fn with_table_partition_cols(self, partition_cols: Vec<FieldRef>) -> Self
```

**via `core::convert::From`**

```rust
fn from(schema: SchemaRef) -> Self
fn from(schema: &SchemaRef) -> Self
```

The overall schema for potentially partitioned data sources.

When reading partitioned data (such as Hive-style partitioning), a [`TableSchema`]
consists of up to three parts:
1. **File schema**: The schema of the actual data files on disk
2. **Partition columns**: Columns whose values are encoded in the directory structure,
   but not stored in the files themselves
3. **Virtual columns**: Columns produced by the file reader (e.g. Parquet
   `row_number`) that are not stored in the files

The full table schema is composed in that order: file columns, then
partition columns, then virtual columns. Consumers that need a different
output ordering should use a projection on top of
[`TableSchema::table_schema`].

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

---

## TableSchemaBuilder

`struct` · `datafusion_datasource::table_schema::TableSchemaBuilder`

Also reachable as `datafusion::datasource::table_schema::TableSchemaBuilder`, `datafusion_datasource::TableSchemaBuilder`

```rust
struct TableSchemaBuilder
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn build(self) -> TableSchema
fn new(file_schema: SchemaRef) -> Self
fn with_table_partition_cols(self, table_partition_cols: impl Into<Fields>) -> Self
fn with_virtual_columns(self, virtual_columns: impl Into<Fields>) -> Self
```

**via `core::convert::From`**

```rust
fn from(schema: &SchemaRef) -> Self
fn from(schema: SchemaRef) -> Self
```

Builder for [`TableSchema`].

The file schema is the only required input; partition columns and virtual
columns are optional. Unlike calling [`TableSchema`]'s setters repeatedly,
the builder computes the concatenated table schema exactly once, in
[`TableSchemaBuilder::build`].

```
# use std::sync::Arc;
# use arrow::datatypes::{Schema, Field, DataType};
# use datafusion_datasource::TableSchemaBuilder;
# let file_schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
let table_schema = TableSchemaBuilder::new(file_schema)
    .with_table_partition_cols(vec![Arc::new(Field::new("date", DataType::Utf8, false))])
    .build();
assert_eq!(table_schema.table_partition_cols().len(), 1);
```

---
