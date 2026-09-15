# `datafusion_datasource::projection`

Crate `datafusion-datasource` · 3 public items · structured records in [`model/datafusion_datasource.projection.json`](../model/datafusion_datasource.projection.json)

## PartitionColumnIndex

`struct` · `datafusion_datasource::projection::PartitionColumnIndex`

Also reachable as `datafusion::datasource::projection::PartitionColumnIndex`

```rust
struct PartitionColumnIndex
```

**Fields**: `in_remainder_projection`, `in_partition_values`

**Derives**: Clone, Copy, Debug

---

## ProjectionOpener

`struct` · `datafusion_datasource::projection::ProjectionOpener`

Also reachable as `datafusion::datasource::projection::ProjectionOpener`

```rust
struct ProjectionOpener
```

**Implements**: `datafusion_datasource::file_stream::FileOpener`

**Methods** (1)

```rust
fn try_new(projection: SplitProjection, inner: Arc<dyn FileOpener>, file_schema: &Schema) -> Result<Arc<dyn FileOpener>>
```

**via `datafusion_datasource::file_stream::FileOpener`**

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

A file opener that handles applying a projection on top of an inner opener.

This includes handling partition columns.

Any projection pushed down will be split up into:
- Simple column indices / column selection
- A remainder projection that this opener applies on top of it

This is meant to simplify projection pushdown for sources like CSV
that can only handle "simple" column selection.

---

## SplitProjection

`struct` · `datafusion_datasource::projection::SplitProjection`

Also reachable as `datafusion::datasource::projection::SplitProjection`

```rust
struct SplitProjection
```

**Fields**: `source`, `file_indices`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(logical_file_schema: &Schema, projection: &ProjectionExprs) -> Self
fn unprojected(table_schema: &TableSchema) -> Self
```

At a high level the goal of SplitProjection is to take a ProjectionExprs meant to be applied to the table schema
and split that into:
- The projection indices into the file schema (file_indices)
- The projection indices into the partition values (partition_value_indices), which pre-compute both the index into the table schema
  and the index into the partition values array
- A remapped projection that can be applied after the file projection is applied
  This remapped projection has the following properties:
    - Column indices referring to file columns are remapped to [0..file_indices.len())
    - Column indices referring to partition columns are remapped to [file_indices.len()..)

  This allows the ProjectionOpener to easily identify which columns in the remapped projection
  refer to partition columns and substitute them with literals from the partition values.

---
