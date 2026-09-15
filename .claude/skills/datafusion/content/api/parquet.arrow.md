# `parquet::arrow`

Crate `parquet` · 4 public items · structured records in [`model/parquet.arrow.json`](../model/parquet.arrow.json)

## ARROW_SCHEMA_META_KEY

`constant` · `parquet::arrow::ARROW_SCHEMA_META_KEY`

```rust
const ARROW_SCHEMA_META_KEY: &str = "ARROW:schema"
```

Schema metadata key used to store serialized Arrow schema

The Arrow schema is encoded using the Arrow IPC format, and then base64
encoded. This is the same format used by arrow-cpp systems, such as pyarrow.

---

## PARQUET_FIELD_ID_META_KEY

`constant` · `parquet::arrow::PARQUET_FIELD_ID_META_KEY`

```rust
const PARQUET_FIELD_ID_META_KEY: &str = "PARQUET:field_id"
```

The value of this metadata key, if present on [`Field::metadata`], will be used
to populate [`BasicTypeInfo::id`]

[`Field::metadata`]: arrow_schema::Field::metadata
[`BasicTypeInfo::id`]: crate::schema::types::BasicTypeInfo::id

---

## parquet_column

`function` · `parquet::arrow::parquet_column`

```rust
fn parquet_column<'a>(parquet_schema: &schema::types::SchemaDescriptor, arrow_schema: &'a arrow_schema::Schema, name: &str) -> Option<(usize, &'a arrow_schema::FieldRef)>
```

Lookups up the parquet column by name

Returns the parquet column index and the corresponding arrow field

---

## ProjectionMask

`struct` · `parquet::arrow::ProjectionMask`

```rust
struct ProjectionMask
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn all() -> Self
fn columns<'a>(schema: &SchemaDescriptor, names: impl IntoIterator<Item = &'a str>) -> Self
fn intersect(&mut self, other: &Self)
fn leaf_included(&self, leaf_idx: usize) -> bool
fn leaves(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self
fn none(len: usize) -> Self
fn roots(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self
fn union(&mut self, other: &Self)
```

A [`ProjectionMask`] identifies a set of columns within a potentially nested schema to project

In particular, a [`ProjectionMask`] can be constructed from a list of leaf column indices
or root column indices where:

* Root columns are the direct children of the root schema, enumerated in order
* Leaf columns are the child-less leaves of the schema as enumerated by a depth-first search

For example, the schema

```ignore
message schema {
  REQUIRED boolean         leaf_1;
  REQUIRED GROUP group {
    OPTIONAL int32 leaf_2;
    OPTIONAL int64 leaf_3;
  }
}
```

Has roots `["leaf_1", "group"]` and leaves `["leaf_1", "leaf_2", "leaf_3"]`

For non-nested schemas, i.e. those containing only primitive columns, the root
and leaves are the same

---
