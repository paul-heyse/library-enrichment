# `datafusion_datasource`

Crate `datafusion-datasource` · 6 public items · structured records in [`model/datafusion_datasource.json`](../model/datafusion_datasource.json)

## generate_test_files

`function` · `datafusion_datasource::generate_test_files`

```rust
fn generate_test_files(num_files: usize, overlap_factor: f64) -> Vec<file_groups::FileGroup>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.generate_test_files.md).


Generates test files with min-max statistics in different overlap patterns.

Used by tests and benchmarks.

# Overlap Factors

The `overlap_factor` parameter controls how much the value ranges in generated test files overlap:
- `0.0`: No overlap between files (completely disjoint ranges)
- `0.2`: Low overlap (20% of the range size overlaps with adjacent files)
- `0.5`: Medium overlap (50% of ranges overlap)
- `0.8`: High overlap (80% of ranges overlap between files)

# Examples

With 5 files and different overlap factors showing `[min, max]` ranges:

overlap_factor = 0.0 (no overlap):

File 0: [0, 20]
File 1: [20, 40]
File 2: [40, 60]
File 3: [60, 80]
File 4: [80, 100]

overlap_factor = 0.5 (50% overlap):

File 0: [0, 40]
File 1: [20, 60]
File 2: [40, 80]
File 3: [60, 100]
File 4: [80, 120]

overlap_factor = 0.8 (80% overlap):

File 0: [0, 100]
File 1: [20, 120]
File 2: [40, 140]
File 3: [60, 160]
File 4: [80, 180]

---

## verify_sort_integrity

`function` · `datafusion_datasource::verify_sort_integrity`

```rust
fn verify_sort_integrity(file_groups: &[file_groups::FileGroup]) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.verify_sort_integrity.md).


Used by tests and benchmarks

---

## FileRange

`struct` · `datafusion_datasource::FileRange`

Also reachable as `datafusion::datasource::listing::FileRange`

```rust
struct FileRange
```

**Fields**: `start`, `end`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn contains(&self, offset: i64) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(range: &protobuf::FileRange) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.FileRange.md).


Only scan a subset of Row Groups from the Parquet file whose data "midpoint"
lies within the [start, end) byte offsets. This option can be used to scan non-overlapping
sections of a Parquet file in parallel.

---

## PartitionedFile

`struct` · `datafusion_datasource::PartitionedFile`

Also reachable as `datafusion::datasource::listing::PartitionedFile`

```rust
struct PartitionedFile
```

**Fields**: `object_meta`, `partition_values`, `range`, `statistics`, `ordering`, `extensions`, `metadata_size_hint`, `table_reference`, `arrow_schema`

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (18)

```rust
fn effective_size(&self) -> u64
fn extension<T: Any + Send + Sync>(&self) -> Option<&T>
fn from_path(path: String) -> Result<Self>
fn has_statistics(&self) -> bool
fn new(path: impl Into<String>, size: u64) -> Self
fn new_from_meta(object_meta: ObjectMeta) -> Self
fn new_with_range(path: String, size: u64, start: i64, end: i64) -> Self
fn path(&self) -> &Path
fn range(&self) -> (u64, u64)
fn with_arrow_schema(self, schema: SchemaRef) -> Self
fn with_extension<T: Any + Send + Sync>(self, value: T) -> Self
fn with_extensions(self, extensions: Arc<dyn Any + Send + Sync>) -> Self
fn with_metadata_size_hint(self, metadata_size_hint: usize) -> Self
fn with_ordering(self, ordering: Option<LexOrdering>) -> Self
fn with_partition_values(self, partition_values: Vec<ScalarValue>) -> Self
fn with_range(self, start: i64, end: i64) -> Self
fn with_statistics(self, file_statistics: Arc<Statistics>) -> Self
fn with_table_reference(self, table_reference: Option<TableReference>) -> Self
```

**via `core::convert::From`**

```rust
fn from(object_meta: ObjectMeta) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(file: &protobuf::PartitionedFile) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.PartitionedFile.md).


A single file or part of a file that should be read, along with its schema, statistics
and partition column values that need to be appended to each row.

# Statistics

The [`Self::statistics`] field contains statistics for the **full table schema**,
which includes both file columns and partition columns. When statistics are set via
[`Self::with_statistics`], exact statistics for partition columns are automatically
computed from [`Self::partition_values`]:

- `min = max = partition_value` (all rows in a file share the same partition value)
- `null_count = 0` (partition values extracted from paths are never null)
- `distinct_count = 1` (single distinct value per file for each partition column)

This enables query optimizers to use partition column bounds for pruning and planning.

---

## FileExtensions

`type_alias` · `datafusion_datasource::FileExtensions`

```rust
type FileExtensions = datafusion_common::extensions::Extensions
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.FileExtensions.md).


User-defined per-file extension data, keyed by concrete Rust type.

Re-exported from [`datafusion_common::extensions::Extensions`]; the same
type backs `SessionConfig::extensions`, `ExtendedStatistics::extensions`,
and other extension fields throughout DataFusion.

---

## PartitionedFileStream

`type_alias` · `datafusion_datasource::PartitionedFileStream`

> **Deprecated** — since 54.0.0: This type is unused and will be removed in a future release

Also reachable as `datafusion::datasource::listing::PartitionedFileStream`

```rust
type PartitionedFileStream = std::pin::Pin<Box<dyn Stream<Item = datafusion_common::Result<PartitionedFile>> + Send + Sync + 'static>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.PartitionedFileStream.md).


Stream of files get listed from object store

---
