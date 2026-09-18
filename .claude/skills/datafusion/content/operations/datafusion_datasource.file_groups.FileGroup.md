# `datafusion_datasource::file_groups::FileGroup`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_groups.FileGroup.json).

<a id="op-74a808112c2188d10bc2c30f"></a>
## FileGroup

`struct` · `datafusion_datasource::file_groups::FileGroup` · datafusion-datasource 55.1.0

```rust
struct FileGroup
```

Source: `src/file_groups.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Represents a group of partitioned files that'll be processed by a single thread.
Maintains optional statistics across all files in the group.

# Statistics

The group-level [`FileGroup::file_statistics`](../operations/datafusion_datasource.file_groups.FileGroup.md#op-4aab3b67e4280cb435a47944) field contains merged statistics from all files
in the group for the **full table schema** (file columns + partition columns).

Partition column statistics are derived from the individual file partition values:
- `min` = minimum partition value across all files in the group
- `max` = maximum partition value across all files in the group
- `null_count` = 0 (partition values are never null)

This allows query optimizers to prune entire file groups based on partition bounds.

<a id="op-10208f5486c0bce805d151f7"></a>
## Error

`assoc_type` · `datafusion_datasource::file_groups::FileGroup::Error` · datafusion-datasource 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "crate::file_groups::FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [160, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab78e9e74d75ef27c588fde3"></a>
## Output

`assoc_type` · `datafusion_datasource::file_groups::FileGroup::Output` · datafusion-datasource 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [546, 1], "end": [552, 2], "filename": "src/file_groups.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/file_groups.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cd946aca7ee87e83642db53"></a>
## clone

`function` · `datafusion_datasource::file_groups::FileGroup::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileGroup
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 17], "end": [379, 22], "filename": "src/file_groups.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_groups.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8df702c0c3dfc19800e01d34"></a>
## default

`function` · `datafusion_datasource::file_groups::FileGroup::default` · datafusion-datasource 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [577, 2], "filename": "src/file_groups.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_groups.rs:574`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aab3b67e4280cb435a47944"></a>
## file_statistics

`function` · `datafusion_datasource::file_groups::FileGroup::file_statistics` · datafusion-datasource 55.1.0

```rust
fn file_statistics(&self, index: Option<usize>) -> Option<&Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the specific file statistics for the given index
If the index is None, return the `FileGroup` statistics

<a id="op-e13a17fff9d8182a55edc6d0"></a>
## files

`function` · `datafusion_datasource::file_groups::FileGroup::files` · datafusion-datasource 55.1.0

```rust
fn files(&self) -> &[PartitionedFile]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns a slice of the files in this group

<a id="op-32808317b4b20cbd7a8e63cf"></a>
## fmt

`function` · `datafusion_datasource::file_groups::FileGroup::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 10], "end": [379, 15], "filename": "src/file_groups.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_groups.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0af8c138fee79efcd2a6d623"></a>
## from

`function` · `datafusion_datasource::file_groups::FileGroup::from` · datafusion-datasource 55.1.0

```rust
fn from(files: Vec<PartitionedFile>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [571, 2], "filename": "src/file_groups.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file_groups.rs:568`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdbd041cb9cec1f8bbe47f55"></a>
## from_iter

`function` · `datafusion_datasource::file_groups::FileGroup::from_iter` · datafusion-datasource 55.1.0

```rust
fn from_iter<I: IntoIterator<Item = PartitionedFile>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 1], "end": [565, 2], "filename": "src/file_groups.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_datasource::PartitionedFile", "path": "PartitionedFile"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/file_groups.rs:561`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a39b12bee84067e0f3ec1013"></a>
## group_by_partition_values

`function` · `datafusion_datasource::file_groups::FileGroup::group_by_partition_values` · datafusion-datasource 55.1.0

```rust
fn group_by_partition_values(self, max_target_partitions: usize) -> Vec<FileGroup>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Groups files by their partition values, ensuring all files with same
partition values are in the same group.

Note: May return fewer groups than `max_target_partitions` when the
number of unique partition values is less than the target.

<a id="op-6eb4df883528feddbc6bd5a9"></a>
## index

`function` · `datafusion_datasource::file_groups::FileGroup::index` · datafusion-datasource 55.1.0

```rust
fn index(&self, index: usize) -> &Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [546, 1], "end": [552, 2], "filename": "src/file_groups.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/file_groups.rs:549`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c0e550aaba7ac2a7c71a2a9"></a>
## index_mut

`function` · `datafusion_datasource::file_groups::FileGroup::index_mut` · datafusion-datasource 55.1.0

```rust
fn index_mut(&mut self, index: usize) -> &mut Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [554, 1], "end": [558, 2], "filename": "src/file_groups.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::IndexMut", "path": "IndexMut"}, "trait_path": "core::ops::index::IndexMut"}`

Source: `src/file_groups.rs:555`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffa870ad55fc9a7da0db2b56"></a>
## into_inner

`function` · `datafusion_datasource::file_groups::FileGroup::into_inner` · datafusion-datasource 55.1.0

```rust
fn into_inner(self) -> Vec<PartitionedFile>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e3762d4f8252bd0772065fc"></a>
## is_empty

`function` · `datafusion_datasource::file_groups::FileGroup::is_empty` · datafusion-datasource 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8034a9875b82c4703ca0fd92"></a>
## iter

`function` · `datafusion_datasource::file_groups::FileGroup::iter` · datafusion-datasource 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = &PartitionedFile>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cd6291cbacf71d168bebf80"></a>
## len

`function` · `datafusion_datasource::file_groups::FileGroup::len` · datafusion-datasource 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the number of files in this group

<a id="op-ac53599fceaa41cb77f00e61"></a>
## new

`function` · `datafusion_datasource::file_groups::FileGroup::new` · datafusion-datasource 55.1.0

```rust
fn new(files: Vec<PartitionedFile>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new FileGroup from a vector of PartitionedFile objects

<a id="op-8c2593e18dc52a6e7fb59d41"></a>
## pop

`function` · `datafusion_datasource::file_groups::FileGroup::pop` · datafusion-datasource 55.1.0

```rust
fn pop(&mut self) -> Option<PartitionedFile>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Removes the last element from the files vector and returns it, or None if empty

<a id="op-3f269e4308e05dc8b687c653"></a>
## push

`function` · `datafusion_datasource::file_groups::FileGroup::push` · datafusion-datasource 55.1.0

```rust
fn push(&mut self, partitioned_file: PartitionedFile)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Adds a file to the group

<a id="op-54e40dbaee420f98c25f22fb"></a>
## split_files

`function` · `datafusion_datasource::file_groups::FileGroup::split_files` · datafusion-datasource 55.1.0

```rust
fn split_files(self, n: usize) -> Vec<FileGroup>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Partition the list of files into `n` groups

<a id="op-97e99ad257b18f7de5226246"></a>
## statistics_mut

`function` · `datafusion_datasource::file_groups::FileGroup::statistics_mut` · datafusion-datasource 55.1.0

```rust
fn statistics_mut(&mut self) -> Option<&mut Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Get the mutable reference to the statistics for this group

<a id="op-e8c82316490d3e837cf14957"></a>
## try_from

`function` · `datafusion_datasource::file_groups::FileGroup::try_from` · datafusion-datasource 55.1.0

```rust
fn try_from(group: &protobuf::FileGroup) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "crate::file_groups::FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [160, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::FileGroup", "path": "FileGroup"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62e33716e0c7641545a2c8dc"></a>
## with_statistics

`function` · `datafusion_datasource::file_groups::FileGroup::with_statistics` · datafusion-datasource 55.1.0

```rust
fn with_statistics(self, statistics: Arc<Statistics>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroup", "path": "FileGroup"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [391, 1], "end": [544, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the statistics for this group
