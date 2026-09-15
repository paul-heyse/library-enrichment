# `datafusion_physical_plan::topk`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.topk.json`](../model/datafusion_physical_plan.topk.json)

## TopK

`struct` · `datafusion_physical_plan::topk::TopK`

Also reachable as `datafusion::physical_plan::TopK`, `datafusion_physical_plan::TopK`

```rust
struct TopK
```

**Methods** (3)

```rust
fn emit(self) -> Result<SendableRecordBatchStream>
fn insert_batch(&mut self, batch: RecordBatch) -> Result<()>
fn try_new(partition_id: usize, schema: SchemaRef, common_sort_prefix: Vec<PhysicalSortExpr>, expr: LexOrdering, k: usize, batch_size: usize, runtime: Arc<RuntimeEnv>, metrics: &ExecutionPlanMetricsSet, filter: Arc<RwLock<TopKDynamicFilters>>) -> Result<Self>
```

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

---

## TopKDynamicFilters

`struct` · `datafusion_physical_plan::topk::TopKDynamicFilters`

```rust
struct TopKDynamicFilters
```

For more background, please also see the [Dynamic Filters: Passing Information Between Operators During Execution for 25x Faster Queries blog]

[Dynamic Filters: Passing Information Between Operators During Execution for 25x Faster Queries blog]: https://datafusion.apache.org/blog/2025/09/10/dynamic-filters

---
