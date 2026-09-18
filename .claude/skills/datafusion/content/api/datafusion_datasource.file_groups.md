# `datafusion_datasource::file_groups`

Crate `datafusion-datasource` · 2 public items · structured records in [`model/datafusion_datasource.file_groups.json`](../model/datafusion_datasource.file_groups.json)

## FileGroup

`struct` · `datafusion_datasource::file_groups::FileGroup`

Also reachable as `datafusion::datasource::physical_plan::FileGroup`

```rust
struct FileGroup
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::iter::traits::collect::FromIterator`, `core::ops::index::Index`, `core::ops::index::IndexMut`

**Derives**: Clone, Debug, Default

**Methods** (13)

```rust
fn file_statistics(&self, index: Option<usize>) -> Option<&Statistics>
fn files(&self) -> &[PartitionedFile]
fn group_by_partition_values(self, max_target_partitions: usize) -> Vec<FileGroup>
fn into_inner(self) -> Vec<PartitionedFile>
fn is_empty(&self) -> bool
fn iter(&self) -> impl Iterator<Item = &PartitionedFile>
fn len(&self) -> usize
fn new(files: Vec<PartitionedFile>) -> Self
fn pop(&mut self) -> Option<PartitionedFile>
fn push(&mut self, partitioned_file: PartitionedFile)
fn split_files(self, n: usize) -> Vec<FileGroup>
fn statistics_mut(&mut self) -> Option<&mut Statistics>
fn with_statistics(self, statistics: Arc<Statistics>) -> Self
```

**via `core::convert::From`**

```rust
fn from(files: Vec<PartitionedFile>) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(group: &protobuf::FileGroup) -> Result<Self>
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = PartitionedFile>>(iter: I) -> Self
```

**via `core::ops::index::Index`**

```rust
fn index(&self, index: usize) -> &Self::Output
```

**via `core::ops::index::IndexMut`**

```rust
fn index_mut(&mut self, index: usize) -> &mut Self::Output
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_groups.FileGroup.md).


Represents a group of partitioned files that'll be processed by a single thread.
Maintains optional statistics across all files in the group.

# Statistics

The group-level [`FileGroup::file_statistics`] field contains merged statistics from all files
in the group for the **full table schema** (file columns + partition columns).

Partition column statistics are derived from the individual file partition values:
- `min` = minimum partition value across all files in the group
- `max` = maximum partition value across all files in the group
- `null_count` = 0 (partition values are never null)

This allows query optimizers to prune entire file groups based on partition bounds.

---

## FileGroupPartitioner

`struct` · `datafusion_datasource::file_groups::FileGroupPartitioner`

Also reachable as `datafusion::datasource::physical_plan::FileGroupPartitioner`

```rust
struct FileGroupPartitioner
```

**Derives**: Clone, Copy, Debug, Default

**Methods** (5)

```rust
fn new() -> Self
fn repartition_file_groups(&self, file_groups: &[FileGroup]) -> Option<Vec<FileGroup>>
fn with_preserve_order_within_groups(self, preserve_order_within_groups: bool) -> Self
fn with_repartition_file_min_size(self, repartition_file_min_size: usize) -> Self
fn with_target_partitions(self, target_partitions: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.file_groups.FileGroupPartitioner.md).


Repartition input files into `target_partitions` partitions, if total file size exceed
`repartition_file_min_size`

This partitions evenly by file byte range, and does not have any knowledge
of how data is laid out in specific files. The specific `FileOpener` are
responsible for the actual partitioning on specific data source type. (e.g.
the `CsvOpener` will read lines overlap with byte range as well as
handle boundaries to ensure all lines will be read exactly once)

# Example

For example, if there are two files `A` and `B` that we wish to read with 4
partitions (with 4 threads) they will be divided as follows:

```text
                                   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
│  File A (7MB)   │   ────────▶     ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
└─────────────────┘                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│  File B (1MB)   │                  ┌─────────────────┐
│                 │                │ │     File A      │ │
└─────────────────┘                  │  Range: 6-7MB   │
                                   │ └─────────────────┘ │
                                     ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

# Maintaining Order

Within each group files are read sequentially. Thus, if the overall order of
tuples must be preserved, multiple files can not be mixed in the same group.

In this case, the code will split the largest files evenly into any
available empty groups, but the overall distribution may not be as even
as if the order did not need to be preserved.

```text
                                  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
┌─────────────────┐                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│  File A (6MB)   │   ────────▶      │                 │
│    (ordered)    │                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
└─────────────────┘                  │                 │
┌─────────────────┐                │ └─────────────────┘ │
│  File B (1MB)   │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│    (ordered)    │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
└─────────────────┘                  ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

---
