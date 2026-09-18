# `datafusion_pruning::file_pruner`

Crate `datafusion-pruning` · 1 public items · structured records in [`model/datafusion_pruning.file_pruner.json`](../model/datafusion_pruning.file_pruner.json)

## FilePruner

`struct` · `datafusion_pruning::file_pruner::FilePruner`

Also reachable as `datafusion_physical_optimizer::pruning::FilePruner`, `datafusion_pruning::FilePruner`

```rust
struct FilePruner
```

**Methods** (4)

```rust
fn is_watching(&self) -> bool
fn new(predicate: Arc<dyn PhysicalExpr>, logical_file_schema: &SchemaRef, _partition_fields: Vec<FieldRef>, partitioned_file: PartitionedFile, predicate_creation_errors: Count) -> Result<Self>
fn should_prune(&mut self) -> Result<bool>
fn try_new(predicate: Arc<dyn PhysicalExpr>, file_schema: &SchemaRef, partitioned_file: &PartitionedFile, predicate_creation_errors: Count) -> Option<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_pruning.file_pruner.FilePruner.md).


Prune based on file-level statistics.

Note: Partition column pruning is handled earlier via `replace_columns_with_literals`
which substitutes partition column references with their literal values before
the predicate reaches this pruner.

---
