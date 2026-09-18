# `datafusion_pruning::file_pruner::FilePruner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.file_pruner.FilePruner.json).

<a id="op-668a35f043ad41e2990cb603"></a>
## FilePruner

`struct` · `datafusion_pruning::file_pruner::FilePruner` · datafusion-pruning 55.1.0

```rust
struct FilePruner
```

Source: `src/file_pruner.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Prune based on file-level statistics.

Note: Partition column pruning is handled earlier via `replace_columns_with_literals`
which substitutes partition column references with their literal values before
the predicate reaches this pruner.

<a id="op-5a69714c5551b9ff4d4e8ee6"></a>
## is_watching

`function` · `datafusion_pruning::file_pruner::FilePruner::is_watching` · datafusion-pruning 55.1.0

```rust
fn is_watching(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::file_pruner::FilePruner", "path": "FilePruner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [171, 2], "filename": "src/file_pruner.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_pruner.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Returns `true` if this pruner watches a dynamic filter that can still
change, meaning [`Self::should_prune`](../operations/datafusion_pruning.file_pruner.FilePruner.md#op-364868d6e4e516329a36f870) is worth re-checking as the scan
progresses. When `false`, the predicate is effectively static for the
remainder of the scan and the caller can avoid wrapping the stream in a
per-batch re-pruning adapter.

<a id="op-6a57238d1b96b8152acc5fab"></a>
## new

`function` · `datafusion_pruning::file_pruner::FilePruner::new` · datafusion-pruning 55.1.0

```rust
fn new(predicate: Arc<dyn PhysicalExpr>, logical_file_schema: &SchemaRef, _partition_fields: Vec<FieldRef>, partitioned_file: PartitionedFile, predicate_creation_errors: Count) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::file_pruner::FilePruner", "path": "FilePruner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [171, 2], "filename": "src/file_pruner.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_pruner.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-364868d6e4e516329a36f870"></a>
## should_prune

`function` · `datafusion_pruning::file_pruner::FilePruner::should_prune` · datafusion-pruning 55.1.0

```rust
fn should_prune(&mut self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::file_pruner::FilePruner", "path": "FilePruner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [171, 2], "filename": "src/file_pruner.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_pruner.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6d16bf79b30695420d3ac3a"></a>
## try_new

`function` · `datafusion_pruning::file_pruner::FilePruner::try_new` · datafusion-pruning 55.1.0

```rust
fn try_new(predicate: Arc<dyn PhysicalExpr>, file_schema: &SchemaRef, partitioned_file: &PartitionedFile, predicate_creation_errors: Count) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::file_pruner::FilePruner", "path": "FilePruner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [171, 2], "filename": "src/file_pruner.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_pruner.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Create a file pruner for this file, or `None` when pruning it cannot
help.

Returns `None` when the file has no statistics struct to evaluate a
pruning predicate against, or when the predicate is purely static and the
file has no usable column statistics — in that case planning already did
everything such a pruner could. A predicate carrying a dynamic filter is
always accepted (given a statistics struct), since it may prune via
partition-value folding even without column statistics.
