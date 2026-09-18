# `datafusion_physical_plan::repartition::BatchPartitioner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.repartition.BatchPartitioner.json).

<a id="op-86d978b7240ddd22811cc71d"></a>
## BatchPartitioner

`struct` · `datafusion_physical_plan::repartition::BatchPartitioner` · datafusion-physical-plan 55.1.0

```rust
struct BatchPartitioner
```

Source: `src/repartition/mod.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A utility that can be used to partition batches based on [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4)

<a id="op-24e6c506bbbee37a6b629599"></a>
## new_hash_partitioner

`function` · `datafusion_physical_plan::repartition::BatchPartitioner::new_hash_partitioner` · datafusion-physical-plan 55.1.0

```rust
fn new_hash_partitioner(exprs: Vec<Arc<dyn PhysicalExpr>>, num_partitions: usize, timer: metrics::Time) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::BatchPartitioner", "path": "BatchPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [1275, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:950`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`BatchPartitioner`](../operations/datafusion_physical_plan.repartition.BatchPartitioner.md#op-86d978b7240ddd22811cc71d) for hash-based repartitioning.

# Parameters
- `exprs`: Expressions used to compute the hash for each input row.
- `num_partitions`: Total number of output partitions.
- `timer`: Metric used to record time spent during repartitioning.

The partition count is fixed for the lifetime of the partitioner, so this
precomputes a strength-reduced reducer for `hash % num_partitions`.

# Errors
Returns an error if `num_partitions` is zero.

<a id="op-8f2f75eb6e4c26fee3dc6917"></a>
## new_range_partitioner

`function` · `datafusion_physical_plan::repartition::BatchPartitioner::new_range_partitioner` · datafusion-physical-plan 55.1.0

```rust
fn new_range_partitioner(range_partitioning: &RangePartitioning, timer: metrics::Time) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::BatchPartitioner", "path": "BatchPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [1275, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1001`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`BatchPartitioner`](../operations/datafusion_physical_plan.repartition.BatchPartitioner.md#op-86d978b7240ddd22811cc71d) for range-based repartitioning.

# Parameters
- `range_partitioning`: `RangePartitioning` struct used for ordering, split points, and number of partitions
- `timer`: Metric used to record time spent during repartitioning.

<a id="op-7fe800082b57117299f21f06"></a>
## new_round_robin_partitioner

`function` · `datafusion_physical_plan::repartition::BatchPartitioner::new_round_robin_partitioner` · datafusion-physical-plan 55.1.0

```rust
fn new_round_robin_partitioner(num_partitions: usize, timer: metrics::Time, input_partition: usize, num_input_partitions: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::BatchPartitioner", "path": "BatchPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [1275, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:981`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`BatchPartitioner`](../operations/datafusion_physical_plan.repartition.BatchPartitioner.md#op-86d978b7240ddd22811cc71d) for round-robin repartitioning.

# Parameters
- `num_partitions`: Total number of output partitions.
- `timer`: Metric used to record time spent during repartitioning.
- `input_partition`: Index of the current input partition.
- `num_input_partitions`: Total number of input partitions.

# Notes
The starting output partition is derived from the input partition
to avoid skew when multiple input partitions are used.

<a id="op-fd126d7fb7f6d5a5ae21ade2"></a>
## partition

`function` · `datafusion_physical_plan::repartition::BatchPartitioner::partition` · datafusion-physical-plan 55.1.0

```rust
fn partition<F>(&mut self, batch: RecordBatch, f: F) -> Result<()> where F: FnMut(usize, RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::BatchPartitioner", "path": "BatchPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [1275, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1072`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Partition the provided [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) into one or more partitioned [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)
based on the [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) specified on construction

`f` will be called for each partitioned [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) with the corresponding
partition index. Any error returned by `f` will be immediately returned by this
function without attempting to publish further [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

The time spent repartitioning, not including time spent in `f` will be recorded
to the [`metrics::Time`](../operations/datafusion_physical_expr_common.metrics.value.Time.md#op-0552155cccfca2434f2fc732) provided on construction

<a id="op-979ad06d8b6c6389dec5a759"></a>
## partition_iter

`function` · `datafusion_physical_plan::repartition::BatchPartitioner::partition_iter` · datafusion-physical-plan 55.1.0

```rust
fn partition_iter(&mut self, batch: RecordBatch) -> Result<impl Iterator<Item = Result<(usize, RecordBatch)>> + Send + '_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::BatchPartitioner", "path": "BatchPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [1275, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1096`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns an iterator of `(partition_index, RecordBatch)` pairs for the given batch.

This is useful for async consumers that want to separate CPU-bound partitioning
from I/O. For example, you can iterate results on the async side and send them
through a channel, while performing file I/O on a blocking task:

```ignore
for result in partitioner.partition_iter(batch)? {
    let (partition, batch) = result?;
    tx.send((partition, batch)).await?;
}
```

The sync [`partition`](Self::partition) method is implemented on top of this.

<a id="op-1c7d57edf1e9701c0d3d02e1"></a>
## try_new

`function` · `datafusion_physical_plan::repartition::BatchPartitioner::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(partitioning: Partitioning, timer: metrics::Time, input_partition: usize, num_input_partitions: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::repartition::BatchPartitioner", "path": "BatchPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [937, 1], "end": [1275, 2], "filename": "src/repartition/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/repartition/mod.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`BatchPartitioner`](../operations/datafusion_physical_plan.repartition.BatchPartitioner.md#op-86d978b7240ddd22811cc71d) based on the provided [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) scheme.

This is a convenience constructor that delegates to the specialized
hash, round-robin, or range constructors depending on the partitioning variant.

# Parameters
- `partitioning`: Partitioning scheme to apply (hash, round-robin, or range).
- `timer`: Metric used to record time spent during repartitioning.
- `input_partition`: Index of the current input partition.
- `num_input_partitions`: Total number of input partitions.

# Errors
Returns an error if the provided partitioning scheme is not supported,
or if hash partitioning is requested with zero output partitions.
