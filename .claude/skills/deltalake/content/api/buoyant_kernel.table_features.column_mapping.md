# `buoyant_kernel::table_features::column_mapping`

Crate `buoyant_kernel` · 5 public items · structured records in [`model/buoyant_kernel.table_features.column_mapping.json`](../model/buoyant_kernel.table_features.column_mapping.json)

## ColumnMappingMode

`enum` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode`

Also reachable as `buoyant_kernel::table_features::ColumnMappingMode`, `delta_kernel::table_features::column_mapping::ColumnMappingMode`

```rust
enum ColumnMappingMode
```

**Variants**: `None`, `Id`, `Name`

**Implements**: `core::convert::TryFrom`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &str) -> ::core::result::Result<ColumnMappingMode, <Self as ::core::convert::TryFrom>::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<ColumnMappingMode, <Self as ::core::str::FromStr>::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Modes of column mapping a table can be in

---

## assign_column_mapping_metadata

`function` · `buoyant_kernel::table_features::column_mapping::assign_column_mapping_metadata`

Also reachable as `buoyant_kernel::table_features::assign_column_mapping_metadata`, `delta_kernel::table_features::column_mapping::assign_column_mapping_metadata`

```rust
fn assign_column_mapping_metadata(schema: &schema::StructType, max_id: &mut i64, assign_nested_field_ids: bool) -> DeltaResult<schema::StructType>
```

Assigns column mapping metadata (id and physicalName) to all fields in a schema,
matching delta-spark's `DeltaColumnMapping.assignColumnIdAndPhysicalName` semantics.

This function recursively processes all fields in the schema, including nested structs,
arrays, and maps. For each field:
- If both `delta.columnMapping.id` and `delta.columnMapping.physicalName` are present, they are
  preserved verbatim and `max_id` is advanced past the preserved id.
- If only `id` is present, `physicalName` is filled in as `col-<uuid>` and the id is preserved.
- If only `physicalName` is present, a new `id` is allocated as `*max_id + 1`.
- If neither is present, both are assigned.

Wrong-typed annotations (`id` not a number, `physicalName` not a string), an empty
`physicalName`, and a negative `id` error out. The latter two are stricter than
delta-spark: an empty physical name fails PROTOCOL.md's "globally unique identifier"
requirement and would break Parquet column resolution, and a negative id is rejected so
`(*max_id).max(n)` cannot silently no-op below the allocator's seed.

Pre-populated `delta.columnMapping.nested.ids` and `parquet.field.nested.ids` annotations
are always rejected: those payloads are kernel-managed and only emitted by the nested-ids
assignment pass below.

When `assign_nested_field_ids` is `true`, after the per-field flat assignment this function
also allocates fresh parquet field ids for the synthetic `Array.element`,
`Map.key`, and `Map.value` slots and stores them under
`delta.columnMapping.nested.ids` on the nearest ancestor `StructField`. Allocating these
nested ids is a hard requirement for IcebergCompatV2/3.

Callers should seed `max_id` from [`find_max_column_id_in_schema`] so newly assigned
IDs do not collide with preserved ones. Duplicate preserved IDs are detected by
[`crate::schema::StructType::make_physical`], which runs from
`TableConfiguration::try_new[_with_schema]` after assignment.

# Arguments

* `schema` - The schema to transform.
* `max_id` - Tracks the highest column ID. Read and updated in place. Should be seeded from
  [`find_max_column_id_in_schema`] (or `0` for a brand-new table with no preserved IDs anywhere)
  so newly assigned IDs cannot collide with preserved ones.
* `assign_nested_field_ids` - When `true`, also allocates IDs for synthetic `Array.element` and
  `Map.key`/`Map.value` fields, stores them in `delta.columnMapping.nested.ids`, and includes
  them in `max_id`. Assigning IDs to these nested fields is a hard requirement for
  IcebergCompatV2/3.

# Returns

A new schema with column mapping metadata present on every field.

# Example

Given a top-level field `m: map<list<int>, int>` with no pre-existing annotations, after
calling with `assign_nested_field_ids = true` the field gets (`<pname>` is the assigned
UUID-based physical name):

```json
{
  "delta.columnMapping.id": 1,
  "delta.columnMapping.physicalName": "<pname>",
  "delta.columnMapping.nested.ids": {
    "<pname>.key":         2,
    "<pname>.key.element": 3,
    "<pname>.value":       4
  }
}
```

`max_id` ends at `4`. With `assign_nested_field_ids = false` the same field gets only the
flat CM metadata and `max_id` ends at `1`:

```json
{
  "delta.columnMapping.id": 1,
  "delta.columnMapping.physicalName": "<pname>"
}
```

---

## find_max_column_id_in_schema

`function` · `buoyant_kernel::table_features::column_mapping::find_max_column_id_in_schema`

Also reachable as `buoyant_kernel::table_features::find_max_column_id_in_schema`, `delta_kernel::table_features::column_mapping::find_max_column_id_in_schema`

```rust
fn find_max_column_id_in_schema(schema: &schema::StructType) -> Option<i64>
```

Returns the largest column mapping id found anywhere in `schema`. This includes both
per-field `delta.columnMapping.id` annotations and the nested ids in
`delta.columnMapping.nested.ids` metadata.

---

## get_any_level_column_physical_name

`function` · `buoyant_kernel::table_features::column_mapping::get_any_level_column_physical_name`

Also reachable as `buoyant_kernel::table_features::get_any_level_column_physical_name`, `delta_kernel::table_features::column_mapping::get_any_level_column_physical_name`

```rust
fn get_any_level_column_physical_name(schema: &schema::StructType, col_name: &expressions::ColumnName, column_mapping_mode: ColumnMappingMode) -> DeltaResult<expressions::ColumnName>
```

Translates a logical [`ColumnName`] to physical. It can be top level or nested.

Uses `StructType::walk_column_fields` to walk the column path through nested structs,
then maps each field to its physical name based on the column mapping mode.

Returns an error if the column name cannot be resolved in the schema, or if column mapping is
enabled but any field in the path lacks the required
[`ColumnMetadataKey::ColumnMappingPhysicalName`] or [`ColumnMetadataKey::ColumnMappingId`]
annotations.

---

## validate_schema_column_mapping

`function` · `buoyant_kernel::table_features::column_mapping::validate_schema_column_mapping`

Also reachable as `buoyant_kernel::table_features::validate_schema_column_mapping`, `delta_kernel::table_features::column_mapping::validate_schema_column_mapping`

```rust
fn validate_schema_column_mapping(schema: &schema::Schema, mode: ColumnMappingMode) -> DeltaResult<()>
```

Validates `delta.columnMapping.id` and `delta.columnMapping.physicalName` annotations across
every field in `schema`. Aligns with delta-spark's validation logic.

When `mode` is [`ColumnMappingMode::Id`] or [`ColumnMappingMode::Name`]: each field must
carry both annotations, no two fields may share a `delta.columnMapping.id`, and no two
fields may share a *full physical column path*. Two fields may share the same leaf
`physicalName` if they live at different physical column paths.

When `mode` is [`ColumnMappingMode::None`]: verifies no field carries either annotation.

Examples for physical name validation:
Rejected (two siblings share `delta.columnMapping.physicalName="x"`):
```json
{"type":"struct","fields":[
  {"name":"a","type":"long","nullable":true,
   "metadata":{"delta.columnMapping.id":1,"delta.columnMapping.physicalName":"x"}},
  {"name":"b","type":"long","nullable":true,
   "metadata":{"delta.columnMapping.id":2,"delta.columnMapping.physicalName":"x"}}
]}
```

Accepted (same `delta.columnMapping.physicalName="x"` at different physical column paths):
```json
{"type":"struct","fields":[
  {"name":"a","type":"long","nullable":true,
   "metadata":{"delta.columnMapping.id":1,"delta.columnMapping.physicalName":"x"}},
  {"name":"nested","nullable":true,
   "metadata":{"delta.columnMapping.id":2,"delta.columnMapping.physicalName":"nested"},
   "type":{"type":"struct","fields":[
     {"name":"a","type":"long","nullable":true,
      "metadata":{"delta.columnMapping.id":3,"delta.columnMapping.physicalName":"x"}}
  ]}}
]}
```

---
