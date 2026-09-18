# `datafusion_common::pruning::PrunableStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.pruning.PrunableStatistics.json).

<a id="op-f2228c72463176247a3d9a97"></a>
## PrunableStatistics

`struct` · `datafusion_common::pruning::PrunableStatistics` · datafusion-common 55.1.0

```rust
struct PrunableStatistics
```

Source: `src/pruning.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Prune a set of containers represented by their statistics.

Each [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) represents a "container" -- some collection of data
that has statistics of its columns.

It is up to the caller to decide what each container represents. For
example, they can come from a file (e.g. [`PartitionedFile`]) or a set of
files (e.g. [`FileGroup`])

[`PartitionedFile`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.PartitionedFile.html
[`FileGroup`]: https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.FileGroup.html

<a id="op-7be53b95b27c3eb3ad3b43e6"></a>
## clone

`function` · `datafusion_common::pruning::PrunableStatistics::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> PrunableStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 10], "end": [313, 15], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/pruning.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6698aa94c1c53a4614c42f3f"></a>
## contained

`function` · `datafusion_common::pruning::PrunableStatistics::contained` · datafusion-common 55.1.0

```rust
fn contained(&self, _column: &Column, _values: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [433, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:426`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ce739320b3260dc0a59d5d"></a>
## max_values

`function` · `datafusion_common::pruning::PrunableStatistics::max_values` · datafusion-common 55.1.0

```rust
fn max_values(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [433, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:369`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3dfea22da7c85c3d5406b2f"></a>
## min_values

`function` · `datafusion_common::pruning::PrunableStatistics::min_values` · datafusion-common 55.1.0

```rust
fn min_values(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [433, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96502b92a8bbb78f97a30bd8"></a>
## new

`function` · `datafusion_common::pruning::PrunableStatistics::new` · datafusion-common 55.1.0

```rust
fn new(statistics: Vec<Arc<Statistics>>, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 1], "end": [362, 2], "filename": "src/pruning.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new instance of [`PrunableStatistics`](../operations/datafusion_common.pruning.PrunableStatistics.md#op-f2228c72463176247a3d9a97).
Each [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) represents a container (e.g. a file or a partition of files).
The `schema` is the schema of the data in the containers and should apply to all files.

<a id="op-2fdba818b8749b6c09cf3595"></a>
## null_counts

`function` · `datafusion_common::pruning::PrunableStatistics::null_counts` · datafusion-common 55.1.0

```rust
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [433, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ed611b3838988c01e5c66e6"></a>
## num_containers

`function` · `datafusion_common::pruning::PrunableStatistics::num_containers` · datafusion-common 55.1.0

```rust
fn num_containers(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [433, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-318f27500329e5855ba64084"></a>
## row_counts

`function` · `datafusion_common::pruning::PrunableStatistics::row_counts` · datafusion-common 55.1.0

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PrunableStatistics", "path": "PrunableStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [433, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
