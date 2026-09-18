# `buoyant_kernel::scan`

Crate `buoyant_kernel` · 9 public items · structured records in [`model/buoyant_kernel.scan.json`](../model/buoyant_kernel.scan.json)

## StructStats

`enum` · `buoyant_kernel::scan::StructStats`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.StructStats.md)

Also reachable as `delta_kernel::scan::StructStats`

```rust
enum StructStats
```

**Variants**: `None`, `All`, `Columns`

**Derives**: Clone, Debug

Which struct stats columns appear in `stats_parsed` in scan metadata output.

---

## get_transform_for_row

`function` · `buoyant_kernel::scan::get_transform_for_row`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.get_transform_for_row.md)

Also reachable as `delta_kernel::scan::get_transform_for_row`

```rust
fn get_transform_for_row(row: usize, transforms: &[Option<expressions::ExpressionRef>]) -> Option<expressions::ExpressionRef>
```

utility method making it easy to get a transform for a particular row. If the requested row is
outside the range of the passed slice returns `None`, otherwise returns the element at the index
of the specified row

---

## scan_row_schema

`function` · `buoyant_kernel::scan::scan_row_schema`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.scan_row_schema.md)

Also reachable as `delta_kernel::scan::scan_row_schema`

```rust
fn scan_row_schema() -> schema::SchemaRef
```

Get the base schema that scan rows (from [`Scan::scan_metadata`]) will be returned with.

This is the base shape; engines may add trailing `*_parsed` columns by opting in via
[`StatsOptions`] (`stats_parsed`) or [`PartitionValuesOptions`] (`partitionValues_parsed`).

It is:
```ignored
{
   path: string,
   size: long,
   modificationTime: long,
   stats: string,
   deletionVector: {
     storageType: string,
     pathOrInlineDv: string,
     offset: int,
     sizeInBytes: int,
     cardinality: long,
   },
   fileConstantValues: {
     partitionValues: map<string, string>,
     tags: map<string, string>,
     baseRowId: long,
     defaultRowCommitVersion: long,
     clusteringProvider: string,
   }
}
```

---

## selection_vector

`function` · `buoyant_kernel::scan::selection_vector`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.selection_vector.md)

Also reachable as `delta_kernel::scan::selection_vector`

```rust
fn selection_vector(engine: &dyn Engine, descriptor: &actions::deletion_vector::DeletionVectorDescriptor, table_root: &url::Url) -> DeltaResult<Vec<bool>>
```

---

## PartitionValuesOptions

`struct` · `buoyant_kernel::scan::PartitionValuesOptions`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.PartitionValuesOptions.md)

Also reachable as `delta_kernel::scan::PartitionValuesOptions`

```rust
struct PartitionValuesOptions
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn string_map_only() -> Self
fn with_struct() -> Self
```

Engine-facing partition value options. Pass to [`ScanBuilder::with_partition_values`] to
declare whether scan metadata output includes the typed `partitionValues_parsed` struct
alongside the raw string map (`fileConstantValues.partitionValues`), which is always present.

When the typed struct is requested, scan metadata output gains a top-level
`partitionValues_parsed` struct column with one typed nullable field per partition column
(physical names, table partition-column order). On non-partitioned tables the column is
omitted. Values come directly from the checkpoint's native `partitionValues_parsed` column
when present, otherwise from parsing the string map.

---

## Scan

`struct` · `buoyant_kernel::scan::Scan`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.Scan.md)

Also reachable as `delta_kernel::scan::Scan`

```rust
struct Scan
```

**Derives**: Debug

**Methods** (9)

```rust
fn execute(&self, engine: Arc<dyn Engine>) -> DeltaResult<impl Iterator<Item = DeltaResult<Box<dyn EngineData>>>>
fn logical_schema(&self) -> &SchemaRef
fn parallel_scan_metadata(&self, engine: Arc<dyn Engine>) -> DeltaResult<SequentialScanMetadata>
fn physical_predicate(&self) -> Option<PredicateRef>
fn physical_schema(&self) -> &SchemaRef
fn scan_metadata(&self, engine: &dyn Engine) -> DeltaResult<impl Iterator<Item = DeltaResult<ScanMetadata>>>
fn scan_metadata_from(&self, engine: &dyn Engine, existing_version: Version, existing_data: impl IntoIterator<Item = Box<dyn EngineData>> + 'static, _existing_predicate: Option<PredicateRef>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<ScanMetadata>>>>
fn snapshot(&self) -> &SnapshotRef
fn table_root(&self) -> &Url
```

The result of building a scan over a table. This can be used to get the actual data from
scanning the table.

---

## ScanBuilder

`struct` · `buoyant_kernel::scan::ScanBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.ScanBuilder.md)

Also reachable as `delta_kernel::scan::ScanBuilder`

```rust
struct ScanBuilder
```

**Derives**: Debug

**Methods** (9)

```rust
fn build(self) -> DeltaResult<Scan>
fn new(snapshot: impl Into<SnapshotRef>) -> Self
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
fn with_partition_values(self, partition_values: PartitionValuesOptions) -> Self
fn with_predicate(self, predicate: impl Into<Option<PredicateRef>>) -> Self
fn with_schema(self, logical_read_schema: SchemaRef) -> Self
fn with_schema_opt(self, schema_opt: Option<SchemaRef>) -> Self
fn with_stats(self, stats: StatsOptions) -> Self
fn without_row_transforms(self) -> Self
```

Builder to scan a snapshot of a table.

---

## ScanMetadata

`struct` · `buoyant_kernel::scan::ScanMetadata`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.ScanMetadata.md)

Also reachable as `delta_kernel::scan::ScanMetadata`

```rust
struct ScanMetadata
```

**Fields**: `scan_files`, `scan_file_transforms`

**Implements**: `buoyant_kernel::log_replay::HasSelectionVector`

**Methods** (1)

```rust
fn visit_scan_files<T>(&self, context: T, callback: ScanCallback<T>) -> DeltaResult<T>
```

**via `buoyant_kernel::log_replay::HasSelectionVector`**

```rust
fn has_selected_rows(&self) -> bool
```

[`ScanMetadata`] contains (1) a batch of [`FilteredEngineData`] specifying data files to be
scanned and (2) a vector of transforms (one transform per scan file) that must be applied to the
data read from those files.

---

## StatsOptions

`struct` · `buoyant_kernel::scan::StatsOptions`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.StatsOptions.md)

Also reachable as `delta_kernel::scan::StatsOptions`

```rust
struct StatsOptions
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn all() -> Self
fn all_struct() -> Self
fn json_only() -> Self
fn none() -> Self
fn struct_columns(cols: Vec<ColumnName>) -> Self
```

Engine-facing stats options. Pass to [`ScanBuilder::with_stats`] to declare
what stats data the engine wants in scan metadata output. Two orthogonal axes:
JSON stats (`add.stats`) and struct stats (`add.stats_parsed`).

Most consumers should pick one of the named constructors:
- [`Self::json_only`] (default) -- JSON stats only.
- [`Self::all_struct`] -- all struct stats, no JSON. Cheap path when the engine consumes
  `stats_parsed` directly; avoids the per-batch `ToJson` cost.
- [`Self::struct_columns`] -- struct stats projected to a subset of columns, no JSON.
- [`Self::all`] -- both representations.
- [`Self::none`] -- neither, AND disables internal data skipping. Unlike the other four
  constructors, this is the only one that stops kernel from reading stats from parquet at all.

---
