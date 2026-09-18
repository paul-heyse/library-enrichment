# `buoyant_kernel::transaction::write_context`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.transaction.write_context.json`](../model/buoyant_kernel.transaction.write_context.json)

## WriteContext

`struct` · `buoyant_kernel::transaction::write_context::WriteContext`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transaction.write_context.WriteContext.md)

Also reachable as `buoyant_kernel::transaction::WriteContext`, `delta_kernel::transaction::write_context::WriteContext`

```rust
struct WriteContext
```

**Derives**: Debug

**Methods** (10)

```rust
fn column_mapping_mode(&self) -> ColumnMappingMode
fn logical_schema(&self) -> &SchemaRef
fn logical_to_physical(&self) -> ExpressionRef
fn new_deletion_vector_path(&self, random_prefix: String) -> DeletionVectorPath
fn physical_partition_values(&self) -> &HashMap<String, Option<String>>
fn physical_schema(&self) -> &SchemaRef
fn resolve_file_path(&self, file_location: &Url) -> DeltaResult<String>
fn stats_columns(&self) -> &[ColumnName]
fn table_root_dir(&self) -> &Url
fn write_dir(&self) -> Url
```

A write context for a specific partition or an unpartitioned table. Created by
[`Transaction::partitioned_write_context`] or [`Transaction::unpartitioned_write_context`].

Note: clustered tables are unpartitioned and use `unpartitioned_write_context`.

Contains both table-wide state (shared cheaply via `Arc`) and per-partition state
(serialized partition values with physical column names as keys). How you use a
`WriteContext` depends on your engine:

- **`DefaultEngine` consumers**: pass this to `DefaultEngine::write_parquet`, which handles
  everything (transform, write, partition metadata).
- **Arrow-based custom engines**: write parquet yourself, then call `build_add_file_metadata`
  with the resulting `DataFileMetadata` and this `WriteContext` to produce the Add action
  `EngineData` for [`Transaction::add_files`].
- **Fully custom (non-Arrow) engines**: use [`physical_partition_values`] to build the
  `partitionValues` map in Add actions directly.

[`Transaction::partitioned_write_context`]: super::Transaction::partitioned_write_context
[`Transaction::unpartitioned_write_context`]: super::Transaction::unpartitioned_write_context
[`Transaction::add_files`]: super::Transaction::add_files
[`physical_partition_values`]: WriteContext::physical_partition_values

---
