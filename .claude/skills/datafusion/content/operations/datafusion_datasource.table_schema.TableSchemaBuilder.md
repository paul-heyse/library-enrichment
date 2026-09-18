# `datafusion_datasource::table_schema::TableSchemaBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.table_schema.TableSchemaBuilder.json).

<a id="op-f2ea99ce0456086ba9c2a350"></a>
## TableSchemaBuilder

`struct` · `datafusion_datasource::table_schema::TableSchemaBuilder` · datafusion-datasource 55.1.0

```rust
struct TableSchemaBuilder
```

Source: `src/table_schema.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Builder for [`TableSchema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-edcf381c15c6591908c8608e).

The file schema is the only required input; partition columns and virtual
columns are optional. Unlike calling [`TableSchema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-edcf381c15c6591908c8608e)'s setters repeatedly,
the builder computes the concatenated table schema exactly once, in
[`TableSchemaBuilder::build`](../operations/datafusion_datasource.table_schema.TableSchemaBuilder.md#op-74d96a3ee774430aafeed6a3).

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

<a id="op-74d96a3ee774430aafeed6a3"></a>
## build

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::build` · datafusion-datasource 55.1.0

```rust
fn build(self) -> TableSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [337, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Build the [`TableSchema`](../operations/datafusion_datasource.table_schema.TableSchema.md#op-edcf381c15c6591908c8608e), computing the full
`file + partition + virtual` schema once.

<a id="op-83690dd029443c4b69bfd98d"></a>
## clone

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> TableSchemaBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 17], "end": [261, 22], "filename": "src/table_schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_schema.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-085960ea91cc13bb0c0af5f7"></a>
## fmt

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 10], "end": [261, 15], "filename": "src/table_schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_schema.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b76b4455e2efeb436b28f38"></a>
## from

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::from` · datafusion-datasource 55.1.0

```rust
fn from(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 1], "end": [343, 2], "filename": "src/table_schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_schema.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e24c25f3d03944a01cc1de06"></a>
## from

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::from` · datafusion-datasource 55.1.0

```rust
fn from(schema: &SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [349, 2], "filename": "src/table_schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_schema.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da1b1d6dfb1176c3408ff9fd"></a>
## new

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::new` · datafusion-datasource 55.1.0

```rust
fn new(file_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [337, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a builder for a `TableSchema` over the given file schema, with no
partition or virtual columns yet.

<a id="op-1708a4f87360b4c97fbe1586"></a>
## with_table_partition_cols

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::with_table_partition_cols` · datafusion-datasource 55.1.0

```rust
fn with_table_partition_cols(self, table_partition_cols: impl Into<Fields>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [337, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the partition columns, replacing any previously set.

Accepts anything convertible into [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) (e.g. `Vec<FieldRef>` or an
existing schema's `Fields`, which is shared zero-copy).

<a id="op-fed8001a390209e1cc290643"></a>
## with_virtual_columns

`function` · `datafusion_datasource::table_schema::TableSchemaBuilder::with_virtual_columns` · datafusion-datasource 55.1.0

```rust
fn with_virtual_columns(self, virtual_columns: impl Into<Fields>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::table_schema::TableSchemaBuilder", "path": "TableSchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [337, 2], "filename": "src/table_schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_schema.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the virtual columns, replacing any previously set.

Virtual columns are produced by the file reader (e.g. Parquet
`row_number`) and appended at the end of the table schema. Each field
must carry an arrow virtual extension type so the reader can recognize
it.

Accepts anything convertible into [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) (e.g. `Vec<FieldRef>`).
