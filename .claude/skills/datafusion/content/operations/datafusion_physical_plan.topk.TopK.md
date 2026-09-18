# `datafusion_physical_plan::topk::TopK`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.topk.TopK.json).

<a id="op-2fcd59ff29c9103fa9939e7b"></a>
## TopK

`struct` · `datafusion_physical_plan::topk::TopK` · datafusion-physical-plan 55.1.0

```rust
struct TopK
```

Source: `src/topk/mod.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

TopK

# Background

"Top K" is a common query optimization used for queries such as
"find the top 3 customers by revenue". The (simplified) SQL for
such a query might be:

```sql
SELECT customer_id, revenue FROM 'sales.csv' ORDER BY revenue DESC limit 3;
```

The simple plan would be:

```sql
> explain SELECT customer_id, revenue FROM sales ORDER BY revenue DESC limit 3;
+--------------+----------------------------------------+
| plan_type    | plan                                   |
+--------------+----------------------------------------+
| logical_plan | Limit: 3                               |
|              |   Sort: revenue DESC NULLS FIRST       |
|              |     Projection: customer_id, revenue   |
|              |       TableScan: sales                 |
+--------------+----------------------------------------+
```

While this plan produces the correct answer, it will fully sorts the
input before discarding everything other than the top 3 elements.

The same answer can be produced by simply keeping track of the top
K=3 elements, reducing the total amount of required buffer memory.

# Partial Sort Optimization

This implementation additionally optimizes queries where the input is already
partially sorted by a common prefix of the requested ordering. If subsequent
rows are guaranteed to be strictly greater (in sort order) than a known TopK
boundary on this prefix, the operator safely terminates early.

For a local TopK, that boundary comes from the local heap once it has K rows.
For a partitioned `SortExec`, a shared dynamic-filter threshold can provide
the same prefix boundary before a lagging partition has filled its local heap.

## Example

For input sorted by `(day DESC)`, but not by `timestamp`, a query such as:

```sql
SELECT day, timestamp FROM sensor ORDER BY day DESC, timestamp DESC LIMIT 10;
```

can terminate scanning early once sufficient rows from the latest days have been
collected, skipping older data.

# Structure

This operator tracks the top K items using a `TopKHeap`.

<a id="op-5c26825bbf28ba49f68678b5"></a>
## emit

`function` · `datafusion_physical_plan::topk::TopK::emit` · datafusion-physical-plan 55.1.0

```rust
fn emit(self) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::topk::TopK", "path": "TopK"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [839, 2], "filename": "src/topk/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/topk/mod.rs:789`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the top k results broken into `batch_size` [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es, consuming the heap

<a id="op-be998f60f887f5438e70969d"></a>
## insert_batch

`function` · `datafusion_physical_plan::topk::TopK::insert_batch` · datafusion-physical-plan 55.1.0

```rust
fn insert_batch(&mut self, batch: RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::topk::TopK", "path": "TopK"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [839, 2], "filename": "src/topk/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/topk/mod.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Insert `batch`, remembering if any of its values are among
the top k seen so far.

<a id="op-7cf88481704dc3e54e58d9f8"></a>
## try_new

`function` · `datafusion_physical_plan::topk::TopK::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(partition_id: usize, schema: SchemaRef, common_sort_prefix: Vec<PhysicalSortExpr>, expr: LexOrdering, k: usize, batch_size: usize, runtime: Arc<RuntimeEnv>, metrics: &ExecutionPlanMetricsSet, filter: Arc<RwLock<TopKDynamicFilters>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::topk::TopK", "path": "TopK"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [839, 2], "filename": "src/topk/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/topk/mod.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`TopK`](../operations/datafusion_physical_plan.topk.TopK.md#op-2fcd59ff29c9103fa9939e7b) that stores the top `k` values, as
defined by the sort expressions in `expr`.
