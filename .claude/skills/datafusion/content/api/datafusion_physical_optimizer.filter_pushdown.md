# `datafusion_physical_optimizer::filter_pushdown`

Crate `datafusion-physical-optimizer` · 1 public items · structured records in [`model/datafusion_physical_optimizer.filter_pushdown.json`](../model/datafusion_physical_optimizer.filter_pushdown.json)

## FilterPushdown

`struct` · `datafusion_physical_optimizer::filter_pushdown::FilterPushdown`

```rust
struct FilterPushdown
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn new_post_optimization() -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.filter_pushdown.FilterPushdown.md).


Attempts to recursively push given filters from the top of the tree into leaves.

# Default Implementation

The default implementation in [`ExecutionPlan::gather_filters_for_pushdown`]
and [`ExecutionPlan::handle_child_pushdown_result`] assumes that:

* Parent filters can't be passed onto children (determined by [`ExecutionPlan::gather_filters_for_pushdown`])
* This node has no filters to contribute (determined by [`ExecutionPlan::gather_filters_for_pushdown`]).
* Any filters that could not be pushed down to the children are marked as unsupported (determined by [`ExecutionPlan::handle_child_pushdown_result`]).

# Example: Push filter into a `DataSourceExec`

For example, consider the following plan:

```text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│      FilterExec      │
│  filters = [ id=1]   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
└──────────────────────┘
```

Our goal is to move the `id = 1` filter from the [`FilterExec`] node to the `DataSourceExec` node.

If this filter is selective pushing it into the scan can avoid massive
amounts of data being read from the source (the projection is `*` so all
matching columns are read).

The new plan looks like:

```text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
          │
          ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│   filters = [ id=1]  │
└──────────────────────┘
```

# Example: Push filters with `ProjectionExec`

Let's consider a more complex example involving a [`ProjectionExec`]
node in between the [`FilterExec`] and `DataSourceExec` nodes that
creates a new column that the filter depends on.

```text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│      FilterExec      │
│    filters =         │
│     [cost>50,id=1]   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    ProjectionExec    │
│ cost = price * 1.2   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
└──────────────────────┘
```

We want to push down the filters `[id=1]` to the `DataSourceExec` node,
but can't push down `cost>50` because it requires the [`ProjectionExec`]
node to be executed first. A simple thing to do would be to split up the
filter into two separate filters and push down the first one:

```text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│      FilterExec      │
│    filters =         │
│     [cost>50]        │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    ProjectionExec    │
│ cost = price * 1.2   │
└──────────────────────┘
            │
            ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│   filters = [ id=1]  │
└──────────────────────┘
```

We can actually however do better by pushing down `price * 1.2 > 50`
instead of `cost > 50`:

```text
┌──────────────────────┐
│ CoalesceBatchesExec  │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    ProjectionExec    │
│ cost = price * 1.2   │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│   filters = [id=1,   │
│   price * 1.2 > 50]  │
└──────────────────────┘
```

# Example: Push filters within a subtree

There are also cases where we may be able to push down filters within a
subtree but not the entire tree. A good example of this is aggregation
nodes:

```text
┌──────────────────────┐
│ ProjectionExec       │
│ projection = *       │
└──────────────────────┘
          │
          ▼
┌──────────────────────┐
│ FilterExec           │
│ filters = [sum > 10] │
└──────────────────────┘
          │
          ▼
┌───────────────────────┐
│     AggregateExec     │
│    group by = [id]    │
│    aggregate =        │
│      [sum(price)]     │
└───────────────────────┘
          │
          ▼
┌──────────────────────┐
│ FilterExec           │
│ filters = [id=1]     │
└──────────────────────┘
         │
         ▼
┌──────────────────────┐
│ DataSourceExec       │
│ projection = *       │
└──────────────────────┘
```

The transformation here is to push down the `id=1` filter to the
`DataSourceExec` node:

```text
┌──────────────────────┐
│ ProjectionExec       │
│ projection = *       │
└──────────────────────┘
          │
          ▼
┌──────────────────────┐
│ FilterExec           │
│ filters = [sum > 10] │
└──────────────────────┘
          │
          ▼
┌───────────────────────┐
│     AggregateExec     │
│    group by = [id]    │
│    aggregate =        │
│      [sum(price)]     │
└───────────────────────┘
          │
          ▼
┌──────────────────────┐
│ DataSourceExec       │
│ projection = *       │
│ filters = [id=1]     │
└──────────────────────┘
```

The point here is that:
1. We cannot push down `sum > 10` through the [`AggregateExec`] node into the `DataSourceExec` node.
   Any filters above the [`AggregateExec`] node are not pushed down.
   This is determined by calling [`ExecutionPlan::gather_filters_for_pushdown`] on the [`AggregateExec`] node.
2. We need to keep recursing into the tree so that we can discover the other [`FilterExec`] node and push
   down the `id=1` filter.

# Example: Push filters through Joins

It is also possible to push down filters through joins and filters that
originate from joins. For example, a hash join where we build a hash
table of the left side and probe the right side (ignoring why we would
choose this order, typically it depends on the size of each table,
etc.).

```text
             ┌─────────────────────┐
             │     FilterExec      │
             │ filters =           │
             │  [d.size > 100]     │
             └─────────────────────┘
                        │
                        │
             ┌──────────▼──────────┐
             │                     │
             │    HashJoinExec     │
             │ [u.dept@hash(d.id)] │
             │                     │
             └─────────────────────┘
                        │
           ┌────────────┴────────────┐
┌──────────▼──────────┐   ┌──────────▼──────────┐
│   DataSourceExec    │   │   DataSourceExec    │
│  alias [users as u] │   │  alias [dept as d]  │
│                     │   │                     │
└─────────────────────┘   └─────────────────────┘
```

There are two pushdowns we can do here:
1. Push down the `d.size > 100` filter through the `HashJoinExec` node to the `DataSourceExec`
   node for the `departments` table.
2. Push down the hash table state from the `HashJoinExec` node to the `DataSourceExec` node to avoid reading
   rows from the `users` table that will be eliminated by the join.
   This can be done via a bloom filter or similar and is not (yet) supported
   in DataFusion. See <https://github.com/apache/datafusion/issues/7955>.

```text
             ┌─────────────────────┐
             │                     │
             │    HashJoinExec     │
             │ [u.dept@hash(d.id)] │
             │                     │
             └─────────────────────┘
                        │
           ┌────────────┴────────────┐
┌──────────▼──────────┐   ┌──────────▼──────────┐
│   DataSourceExec    │   │   DataSourceExec    │
│  alias [users as u] │   │  alias [dept as d]  │
│ filters =           │   │  filters =          │
│   [depg@hash(d.id)] │   │    [ d.size > 100]  │
└─────────────────────┘   └─────────────────────┘
```

You may notice in this case that the filter is *dynamic*: the hash table
is built _after_ the `departments` table is read and at runtime. We
don't have a concrete `InList` filter or similar to push down at
optimization time. These sorts of dynamic filters are handled by
building a specialized [`PhysicalExpr`] that can be evaluated at runtime
and internally maintains a reference to the hash table or other state.

To make working with these sorts of dynamic filters more tractable we have the method [`PhysicalExpr::snapshot`]
which attempts to simplify a dynamic filter into a "basic" non-dynamic filter.
For a join this could mean converting it to an `InList` filter or a min/max filter for example.
See `datafusion/physical-plan/src/dynamic_filters.rs` for more details.

# Example: Push TopK filters into Scans

Another form of dynamic filter is pushing down the state of a `TopK`
operator for queries like `SELECT * FROM t ORDER BY id LIMIT 10`:

```text
┌──────────────────────┐
│       TopK           │
│     limit = 10       │
│   order by = [id]    │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
└──────────────────────┘
```

We can avoid large amounts of data processing by transforming this into:

```text
┌──────────────────────┐
│       TopK           │
│     limit = 10       │
│   order by = [id]    │
└──────────────────────┘
           │
           ▼
┌──────────────────────┐
│    DataSourceExec    │
│    projection = *    │
│ filters =            │
│    [id < @ TopKHeap] │
└──────────────────────┘
```

Now as we fill our `TopK` heap we can push down the state of the heap to
the `DataSourceExec` node to avoid reading files / row groups / pages /
rows that could not possibly be in the top 10.

This is not yet implemented in DataFusion. See
<https://github.com/apache/datafusion/issues/15037>

[`PhysicalExpr`]: datafusion_physical_plan::PhysicalExpr
[`PhysicalExpr::snapshot`]: datafusion_physical_plan::PhysicalExpr::snapshot
[`FilterExec`]: datafusion_physical_plan::filter::FilterExec
[`ProjectionExec`]: datafusion_physical_plan::projection::ProjectionExec
[`AggregateExec`]: datafusion_physical_plan::aggregates::AggregateExec

---
