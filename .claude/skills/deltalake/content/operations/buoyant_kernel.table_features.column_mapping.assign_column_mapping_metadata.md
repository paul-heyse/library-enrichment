# `buoyant_kernel::table_features::column_mapping::assign_column_mapping_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.column_mapping.assign_column_mapping_metadata.json).

<a id="op-e7fc30c054b226a0cd502886"></a>
## assign_column_mapping_metadata

`function` · `buoyant_kernel::table_features::column_mapping::assign_column_mapping_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn assign_column_mapping_metadata(schema: &schema::StructType, max_id: &mut i64, assign_nested_field_ids: bool) -> DeltaResult<schema::StructType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L383).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:383`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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

Callers should seed `max_id` from [`find_max_column_id_in_schema`](../operations/buoyant_kernel.table_features.column_mapping.find_max_column_id_in_schema.md#op-ca539fe0c0bd760ec501c0bf) so newly assigned
IDs do not collide with preserved ones. Duplicate preserved IDs are detected by
[`crate::schema::StructType::make_physical`](../operations/buoyant_kernel.schema.StructType.md#op-170d49e537dca68f8591d655), which runs from
`TableConfiguration::try_new[_with_schema]` after assignment.

# Arguments

* `schema` - The schema to transform.
* `max_id` - Tracks the highest column ID. Read and updated in place. Should be seeded from
  [`find_max_column_id_in_schema`](../operations/buoyant_kernel.table_features.column_mapping.find_max_column_id_in_schema.md#op-ca539fe0c0bd760ec501c0bf) (or `0` for a brand-new table with no preserved IDs anywhere)
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
