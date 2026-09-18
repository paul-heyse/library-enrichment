# `datafusion_physical_plan::execution_plan::ExecutionPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.ExecutionPlan.json).

<a id="op-ac09436cd73f869917923673"></a>
## ExecutionPlan

`trait` · `datafusion_physical_plan::execution_plan::ExecutionPlan` · datafusion-physical-plan 55.1.0

```rust
trait ExecutionPlan: Any + Debug + DisplayAs + Send + Sync
```

Source: `src/execution_plan.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Represent nodes in the DataFusion Physical Plan.

Calling [`execute`] produces an `async` [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6) of
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) that incrementally computes a partition of the
`ExecutionPlan`'s output from its input. See [`Partitioning`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-b631e1d1890e8ea81608b4a4) for more
details on partitioning.

Methods such as [`Self::schema`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-f41258ad6bdf46680c8078d0) and [`Self::properties`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ef2d89ab2da894daea49ae8e) communicate
properties of the output to the DataFusion optimizer, and methods such as
[`required_input_distribution`] and [`required_input_ordering`] express
requirements of the `ExecutionPlan` from its input.

[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) can be displayed in a simplified form using the
return value from [`displayable`](../operations/datafusion_physical_plan.execution_plan.displayable.md#op-727f9d5db0c6bca1a2d77add) in addition to the (normally
quite verbose) `Debug` output.

[`execute`]: ExecutionPlan::execute
[`required_input_distribution`]: ExecutionPlan::required_input_distribution
[`required_input_ordering`]: ExecutionPlan::required_input_ordering

# Examples

See [`datafusion-examples`] for examples, including
[`memory_pool_execution_plan.rs`] which shows how to implement a custom
`ExecutionPlan` with memory tracking and spilling support.

[`datafusion-examples`]: https://github.com/apache/datafusion/tree/main/datafusion-examples
[`memory_pool_execution_plan.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/execution_monitoring/memory_pool_execution_plan.rs

<a id="op-b548b61205d0cb45f3ae084e"></a>
## apply_expressions

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Source: `src/execution_plan.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Apply a closure `f` to each root expression that this node owns and uses
during execution, either by evaluating it or updating it dynamically.

An expression must not be visited solely because it describes an input or
output property, such as cached ordering, partitioning, or equivalence
metadata. However, these may be traversed indirectly. For example,
`RepartitionExec` visits the partitioning expressions it evaluates  and
`SortExec` visits the sort expressions it evaluates to order rows.

This method is shallow: it must not visit expression children or expressions
owned by child execution plans.

Similarly to other [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) APIs, the closure can return
[`TreeNodeRecursion::Stop`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-fbbedd1f0409ac786df535a1) to stop iteration, otherwise iteration
should continue. Note that [`TreeNodeRecursion::Continue`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-128238c101f7ed2798f6d11c) and
[`TreeNodeRecursion::Jump`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-5524dc7ddeb08b422f96ae27) are equivalent because this method is not
recursive.


# Example Usage
```
# use std::sync::Arc;
# use datafusion_physical_plan::ExecutionPlan;
# use datafusion_common::tree_node::TreeNodeRecursion;
# fn example(plan: Arc<dyn ExecutionPlan>) -> datafusion_common::Result<()> {
// Count the number of expressions
let mut count = 0;
plan.apply_expressions(&mut |_expr| {
    count += 1;
    Ok(TreeNodeRecursion::Continue)
})?;
# Ok(())
# }
```

# Implementation Examples

## Node with expressions (e.g., FilterExec, ProjectionExec)

Use [`apply_expression_roots`](../operations/datafusion_physical_plan.execution_plan.apply_expression_roots.md#op-d8f6900037992304af2eeed5) to implement this method. It abstracts away the
[`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) iteration from implementors.
```ignore
fn apply_expressions(
    &self,
    f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
) -> Result<TreeNodeRecursion> {
    apply_expression_roots([&self.predicate], f)
}
```

## Node with no expressions (e.g., EmptyExec, MemoryExec)
```ignore
fn apply_expressions(
    &self,
    _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
) -> Result<TreeNodeRecursion> {
    Ok(TreeNodeRecursion::Continue)
}
```

<a id="op-782da3d20642405ff2f5efb1"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Source: `src/execution_plan.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specifies whether the `ExecutionPlan` benefits from increased
parallelization at its input for each child.

If returns `true`, the `ExecutionPlan` would benefit from partitioning
its corresponding child (and thus from more parallelism). For
`ExecutionPlan` that do very little work the overhead of extra
parallelism may outweigh any benefits

The default implementation returns `true` unless this `ExecutionPlan`
has signalled it requires a single child input partition.

<a id="op-defddba55890b31ee9934a1f"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Source: `src/execution_plan.rs:804`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Gets the effect on cardinality, if known

<a id="op-a9c1bef0e1633369ece4daa7"></a>
## check_invariants

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::check_invariants` · datafusion-physical-plan 55.1.0

```rust
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
```

Source: `src/execution_plan.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns an error if this individual node does not conform to its invariants.
These invariants are typically only checked in debug mode.

A default set of invariants is provided in the [check_default_invariants](../operations/datafusion_physical_plan.execution_plan.check_default_invariants.md#op-55643d6e49309cb47ba00c0a) function.
The default implementation of `check_invariants` calls this function.
Extension nodes can provide their own invariants.

<a id="op-9b5f78775acc13c2573e8ec1"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
```

Source: `src/execution_plan.rs:774`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns, per child, which statistics the [`StatisticsContext`] should resolve
before calling [`Self::statistics_from_inputs`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-7dad80ba3b610abc3c532d8a).

One entry per child (same order as [`Self::children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-74306cb44d2bd0f7989cbdb3)): [`ChildStats::At`](../operations/datafusion_physical_plan.statistics.ChildStats.md#op-d2332e33afc920d370c1f6cc)
requests the child's statistics at a partition (`None` = overall);
[`ChildStats::Skip`](../operations/datafusion_physical_plan.statistics.ChildStats.md#op-609e8a7a86a094c171eeea8c) omits a child whose statistics this node does not need
(a `Statistics::new_unknown` placeholder fills its `input_stats` slot).

The default skips every child, so a node that derives nothing from its
children (for example one that only overrides the deprecated
[`Self::partition_statistics`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-6923c0e0a9951b41d7be05cd)) triggers no child traversal. A node that reads
`input_stats` in [`Self::statistics_from_inputs`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-7dad80ba3b610abc3c532d8a) must override this to declare
the children it uses.

[`StatisticsContext`]: crate::statistics::StatisticsContext

<a id="op-74306cb44d2bd0f7989cbdb3"></a>
## children

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get a list of children `ExecutionPlan`s that act as inputs to this plan.
The returned list will be empty for leaf nodes such as scans, will contain
a single value for unary nodes, or two values for binary nodes (such as
joins).

<a id="op-0ee92934a2bcd160d1c6f59a"></a>
## downcast_delegate

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::downcast_delegate` · datafusion-physical-plan 55.1.0

```rust
fn downcast_delegate(&self) -> Option<&dyn ExecutionPlan>
```

Source: `src/execution_plan.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the plan that provides this plan's public
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) downcast identity.

This hook is for wrapper nodes that delegate their public downcast
identity to another plan while adding cross-cutting behavior such as
instrumentation. The default implementation returns `None`, meaning this
plan's concrete type is used for type introspection.

Most `ExecutionPlan` implementations should use the default `None`;
override this only for wrapper plans that intentionally delegate their
public downcast identity to another plan.

The `is` and `downcast_ref` helpers follow the returned delegate instead
of checking the current concrete type, making intermediate delegating
wrappers invisible to normal downcast-based inspection.

Implementations that opt in should return the delegate plan, not `self`.

This is independent from [`Self::children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-74306cb44d2bd0f7989cbdb3) and should not be used for
plan traversal or optimizer rewrites.

<a id="op-1eb60867be978512e14274ab"></a>
## dynamic_expressions_produced

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::dynamic_expressions_produced` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Source: `src/execution_plan.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the dynamic expressions produced by this plan node.

A dynamic expression is produced when this node updates or completes its
runtime state during execution. Expressions that this node only consumes
must not be returned. This method is shallow and does not include dynamic
expressions produced by child plans.

Each returned expression must have a [`PhysicalExpr::expression_id`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-40e9c4db3e31f20fd22444d4)
since all dynamic expressions such as [`DynamicFilterPhysicalExpr`]
have an expression id.

[`DynamicFilterPhysicalExpr`]: datafusion_physical_expr::expressions::DynamicFilterPhysicalExpr

<a id="op-73dceb67a4da50c62af132d9"></a>
## execute

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Source: `src/execution_plan.rs:696`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Begin execution of `partition`, returning a [`Stream`] of
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es.

# Notes

The `execute` method itself is not `async` but it returns an `async`
[`futures::stream::Stream`]. This `Stream` should incrementally compute
the output, `RecordBatch` by `RecordBatch` (in a streaming fashion).
Most `ExecutionPlan`s should not do any work before the first
`RecordBatch` is requested from the stream.

[`RecordBatchStreamAdapter`] can be used to convert an `async`
[`Stream`] into a [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6).

Using `async` `Streams` allows for network I/O during execution and
takes advantage of Rust's built in support for `async` continuations and
crate ecosystem.

[`Stream`]: futures::stream::Stream
[`StreamExt`]: futures::stream::StreamExt
[`TryStreamExt`]: futures::stream::TryStreamExt
[`RecordBatchStreamAdapter`]: crate::stream::RecordBatchStreamAdapter

# Error handling

Any error that occurs during execution is sent as an `Err` in the output
stream.

`ExecutionPlan` implementations in DataFusion cancel additional work
immediately once an error occurs. The rationale is that if the overall
query will return an error,  any additional work such as continued
polling of inputs will be wasted as it will be thrown away.

# Cancellation / Aborting Execution

The [`Stream`] that is returned must ensure that any allocated resources
are freed when the stream itself is dropped. This is particularly
important for [`spawn`]ed tasks or threads. Unless care is taken to
"abort" such tasks, they may continue to consume resources even after
the plan is dropped, generating intermediate results that are never
used.
Thus, [`spawn`] is disallowed, and instead use [`SpawnedTask`].

To enable timely cancellation, the [`Stream`] that is returned must not
block the CPU indefinitely and must yield back to the tokio runtime regularly.
In a typical [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673), this automatically happens unless there are
special circumstances; e.g. when the computational complexity of processing a
batch is superlinear. See this [general guideline][async-guideline] for more context
on this point, which explains why one should avoid spending a long time without
reaching an `await`/yield point in asynchronous runtimes.
This can be achieved by using the utilities from the [`coop`](crate::coop) module, by
manually returning [`Poll::Pending`] and setting up wakers appropriately, or by calling
[`tokio::task::yield_now()`] when appropriate.
In special cases that warrant manual yielding, determination for "regularly" may be
made using the [Tokio task budget](https://docs.rs/tokio/latest/tokio/task/coop/index.html),
a timer (being careful with the overhead-heavy system call needed to take the time), or by
counting rows or batches.

The [cancellation benchmark] tracks some cases of how quickly queries can
be cancelled.

For more details see [`SpawnedTask`], [`JoinSet`] and [`RecordBatchReceiverStreamBuilder`]
for structures to help ensure all background tasks are cancelled.

[`spawn`]: tokio::task::spawn
[cancellation benchmark]: https://github.com/apache/datafusion/blob/main/benchmarks/README.md#cancellation
[`JoinSet`]: datafusion_common_runtime::JoinSet
[`SpawnedTask`]: datafusion_common_runtime::SpawnedTask
[`RecordBatchReceiverStreamBuilder`]: crate::stream::RecordBatchReceiverStreamBuilder
[`Poll::Pending`]: std::task::Poll::Pending
[async-guideline]: https://ryhl.io/blog/async-what-is-blocking/

# Implementation Examples

While `async` `Stream`s have a non trivial learning curve, the
[`futures`] crate provides [`StreamExt`] and [`TryStreamExt`]
which help simplify many common operations.

Here are some common patterns:

## Return Precomputed `RecordBatch`

We can return a precomputed `RecordBatch` as a `Stream`:

```
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use arrow::datatypes::SchemaRef;
# use datafusion_common::Result;
# use datafusion_execution::{SendableRecordBatchStream, TaskContext};
# use datafusion_physical_plan::memory::MemoryStream;
# use datafusion_physical_plan::stream::RecordBatchStreamAdapter;
struct MyPlan {
    batch: RecordBatch,
}

impl MyPlan {
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        // use functions from futures crate to convert the batch into a stream
        let fut = futures::future::ready(Ok(self.batch.clone()));
        let stream = futures::stream::once(fut);
        Ok(Box::pin(RecordBatchStreamAdapter::new(
            self.batch.schema(),
            stream,
        )))
    }
}
```

## Lazily (async) Compute `RecordBatch`

We can also lazily compute a `RecordBatch` when the returned `Stream` is polled

```
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use arrow::datatypes::SchemaRef;
# use datafusion_common::Result;
# use datafusion_execution::{SendableRecordBatchStream, TaskContext};
# use datafusion_physical_plan::memory::MemoryStream;
# use datafusion_physical_plan::stream::RecordBatchStreamAdapter;
struct MyPlan {
    schema: SchemaRef,
}

/// Returns a single batch when the returned stream is polled
async fn get_batch() -> Result<RecordBatch> {
    todo!()
}

impl MyPlan {
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        let fut = get_batch();
        let stream = futures::stream::once(fut);
        Ok(Box::pin(RecordBatchStreamAdapter::new(
            self.schema.clone(),
            stream,
        )))
    }
}
```

## Lazily (async) create a Stream

If you need to create the return `Stream` using an `async` function,
you can do so by flattening the result:

```
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use arrow::datatypes::SchemaRef;
# use futures::TryStreamExt;
# use datafusion_common::Result;
# use datafusion_execution::{SendableRecordBatchStream, TaskContext};
# use datafusion_physical_plan::memory::MemoryStream;
# use datafusion_physical_plan::stream::RecordBatchStreamAdapter;
struct MyPlan {
    schema: SchemaRef,
}

/// async function that returns a stream
async fn get_batch_stream() -> Result<SendableRecordBatchStream> {
    todo!()
}

impl MyPlan {
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        // A future that yields a stream
        let fut = get_batch_stream();
        // Use TryStreamExt::try_flatten to flatten the stream of streams
        let stream = futures::stream::once(fut).try_flatten();
        Ok(Box::pin(RecordBatchStreamAdapter::new(
            self.schema.clone(),
            stream,
        )))
    }
}
```

Unresolved upstream links (retained, not inferred): `futures::stream::Stream`, ``futures``, ``futures::stream::Stream``, `futures::stream::StreamExt`, ``tokio::task::yield_now()``, `futures::stream::TryStreamExt`, `std::task::Poll::Pending`, `tokio::task::spawn`.

<a id="op-4c7cd952aeddca32f61394b9"></a>
## fetch

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Source: `src/execution_plan.rs:799`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Gets the fetch count for the operator, `None` means there is no fetch.

<a id="op-4d78429029b27c298789f1b5"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Source: `src/execution_plan.rs:851`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Collect filters that this node can push down to its children.
Filters that are being pushed down from parents are passed in,
and the node may generate additional filters to push down.
For example, given the plan FilterExec -> HashJoinExec -> DataSourceExec,
what will happen is that we recurse down the plan calling `ExecutionPlan::gather_filters_for_pushdown`:
1. `FilterExec::gather_filters_for_pushdown` is called with no parent
   filters so it only returns that `FilterExec` wants to push down its own predicate.
2. `HashJoinExec::gather_filters_for_pushdown` is called with the filter from
   `FilterExec`, which it only allows to push down to one side of the join (unless it's on the join key)
   but it also adds its own filters (e.g. pushing down a bloom filter of the hash table to the scan side of the join).
3. `DataSourceExec::gather_filters_for_pushdown` is called with both filters from `HashJoinExec`
   and `FilterExec`, however `DataSourceExec::gather_filters_for_pushdown` doesn't actually do anything
   since it has no children and no additional filters to push down.
   It's only once [`ExecutionPlan::handle_child_pushdown_result`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-be5acc8664105339204cc270) is called on `DataSourceExec` as we recurse
   up the plan that `DataSourceExec` can actually bind the filters.

The default implementation bars all parent filters from being pushed down and adds no new filters.
This is the safest option, making filter pushdown opt-in on a per-node basis.

There are two different phases in filter pushdown, which some operators may handle the same and some differently.
Depending on the phase the operator may or may not be allowed to modify the plan.
See [`FilterPushdownPhase`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPhase.md#op-a860d98a971302895a804c6d) for more details.

Implementations must preserve the order of `parent_filters` in the
returned child [`FilterDescription`](../operations/datafusion_physical_plan.filter_pushdown.FilterDescription.md#op-0116d642527aa63591adbd0f): each child parent-filter result is
matched back to the corresponding input parent filter by position.
Unsupported filters should therefore be marked unsupported in place,
rather than removed or appended after supported filters.

<a id="op-be5acc8664105339204cc270"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Source: `src/execution_plan.rs:943`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Handle the result of a child pushdown.

This method is called as we recurse back up the plan tree after pushing
filters down to child nodes via [`ExecutionPlan::gather_filters_for_pushdown`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4d78429029b27c298789f1b5).
It allows the current node to process the results of filter pushdown from
its children, deciding whether to absorb filters, modify the plan, or pass
filters back up to its parent.

**Purpose and Context:**
Filter pushdown is a critical optimization in DataFusion that aims to
reduce the amount of data processed by applying filters as early as
possible in the query plan. This method is part of the second phase of
filter pushdown, where results are propagated back up the tree after
being pushed down. Each node can inspect the pushdown results from its
children and decide how to handle any unapplied filters, potentially
optimizing the plan structure or filter application.

**Behavior in Different Nodes:**
- For a `DataSourceExec`, this often means absorbing the filters to apply
  them during the scan phase (late materialization), reducing the data
  read from the source.
- A `FilterExec` may absorb any filters its children could not handle,
  combining them with its own predicate. If no filters remain (i.e., the
  predicate becomes trivially true), it may remove itself from the plan
  altogether. It typically marks parent filters as supported, indicating
  they have been handled.
- A `HashJoinExec` might ignore the pushdown result if filters need to
  be applied during the join operation. It passes the parent filters back
  up wrapped in [`FilterPushdownPropagation::if_any`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-39bdb57d228b4f831aeb4e1c), discarding
  any self-filters from children.

**Example Walkthrough:**
Consider a query plan: `FilterExec (f1) -> HashJoinExec -> DataSourceExec`.
1. **Downward Phase (`gather_filters_for_pushdown`):** Starting at
   `FilterExec`, the filter `f1` is gathered and pushed down to
   `HashJoinExec`. `HashJoinExec` may allow `f1` to pass to one side of
   the join or add its own filters (e.g., a min-max filter from the build side),
   then pushes filters to `DataSourceExec`. `DataSourceExec`, being a leaf node,
   has no children to push to, so it prepares to handle filters in the
   upward phase.
2. **Upward Phase (`handle_child_pushdown_result`):** Starting at
   `DataSourceExec`, it absorbs applicable filters from `HashJoinExec`
   for late materialization during scanning, marking them as supported.
   `HashJoinExec` receives the result, decides whether to apply any
   remaining filters during the join, and passes unhandled filters back
   up to `FilterExec`. `FilterExec` absorbs any unhandled filters,
   updates its predicate if necessary, or removes itself if the predicate
   becomes trivial (e.g., `lit(true)`), and marks filters as supported
   for its parent.

The default implementation is a no-op that passes the result of pushdown
from the children to its parent transparently, ensuring no filters are
lost if a node does not override this behavior.

**Notes for Implementation:**
When returning filters via [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286), the order of
filters need not match the order they were passed in via
`child_pushdown_result`. However, preserving the order is recommended for
debugging and ease of reasoning about the resulting plans.

**Helper Methods for Customization:**
There are various helper methods to simplify implementing this method:
- [`FilterPushdownPropagation::if_any`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-39bdb57d228b4f831aeb4e1c): Marks all parent filters as
  supported as long as at least one child supports them.
- [`FilterPushdownPropagation::if_all`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-d13c63c6e3905c916eeef408): Marks all parent filters as
  supported as long as all children support them.
- [`FilterPushdownPropagation::with_parent_pushdown_result`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-2d7cf1531ddd78bc20b6ecea): Allows adding filters
  to the propagation result, indicating which filters are supported by
  the current node.
- [`FilterPushdownPropagation::with_updated_node`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-9d37fce664ffb94f2f7f9fac): Allows updating the
  current node in the propagation result, used if the node
  has modified its plan based on the pushdown results.

**Filter Pushdown Phases:**
There are two different phases in filter pushdown (`Pre` and others),
which some operators may handle differently. Depending on the phase, the
operator may or may not be allowed to modify the plan. See
[`FilterPushdownPhase`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPhase.md#op-a860d98a971302895a804c6d) for more details on phase-specific behavior.

[`PushedDownPredicate::supported`]: crate::filter_pushdown::PushedDownPredicate::supported

<a id="op-8ef86f98da95d0ca77fa067e"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Source: `src/execution_plan.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specifies the input distribution requirements for this plan.

The default implementation wraps [`Self::required_input_distribution`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4675c0f905c4d7c995a86312).
Override this method for richer requirements, such as allowing alternate
satisfaction policies or requiring multiple children to be co-partitioned.
See [`InputDistributionRequirements`](../operations/datafusion_physical_plan.distribution_requirements.InputDistributionRequirements.md#op-e1bc2ab71ce298f90cee462e) for details.

<a id="op-fb678887a0ccc94b18374b08"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Source: `src/execution_plan.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns `false` if this `ExecutionPlan`'s implementation may reorder
rows within or between partitions.

For example, Projection, Filter, and Limit maintain the order
of inputs -- they may transform values (Projection) or not
produce the same number of rows that went in (Filter and
Limit), but the rows that are produced go in the same way.

DataFusion uses this metadata to apply certain optimizations
such as automatically repartitioning correctly.

The default implementation returns `false`

WARNING: if you override this default, you *MUST* ensure that
the `ExecutionPlan`'s maintains the ordering invariant or else
DataFusion may produce incorrect results.

<a id="op-f9e5333c240a14987dead676"></a>
## metrics

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Source: `src/execution_plan.rs:713`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a snapshot of the set of [`Metric`](../operations/datafusion_physical_expr_common.metrics.Metric.md#op-f3d3e8659b0526690e35cebe)s for this
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673). If no `Metric`s are available, return None.

While the values of the metrics in the returned
[`MetricsSet`](../operations/datafusion_physical_expr_common.metrics.MetricsSet.md#op-c077c70588e76ca4a87a0b1b)s may change as execution progresses, the
specific metrics will not.

Once `self.execute()` has returned (technically the future is
resolved) for all available partitions, the set of metrics
should be complete. If this function is called prior to
`execute()` new metrics may appear in subsequent calls.

<a id="op-77cc56ba66fc710c45bf88b5"></a>
## name

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/execution_plan.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Short name for the ExecutionPlan, such as 'DataSourceExec'.

Implementation note: this method can just proxy to
[`static_name`](ExecutionPlan::static_name) if no special action is
needed. It doesn't provide a default implementation like that because
this method doesn't require the `Sized` constrain to allow a wilder
range of use cases.

<a id="op-6923c0e0a9951b41d7be05cd"></a>
## partition_statistics

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::partition_statistics` · datafusion-physical-plan 55.1.0

```rust
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
```

Source: `src/execution_plan.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns statistics for a specific partition of this `ExecutionPlan` node.

Deprecated: use [`StatisticsContext::compute`] instead.

[`StatisticsContext::compute`]: crate::statistics::StatisticsContext::compute

<a id="op-ef2d89ab2da894daea49ae8e"></a>
## properties

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Source: `src/execution_plan.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return properties of the output of the `ExecutionPlan`, such as output
ordering(s), partitioning information etc.

This information is available via methods on [`ExecutionPlanProperties`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlanProperties.md#op-ec582c74a9817b539270a5c9)
trait, which is implemented for all `ExecutionPlan`s.

<a id="op-584645e19b0573a60b990c36"></a>
## repartitioned

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::repartitioned` · datafusion-physical-plan 55.1.0

```rust
fn repartitioned(&self, _target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Source: `src/execution_plan.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If supported, attempt to increase the partitioning of this `ExecutionPlan` to
produce `target_partitions` partitions.

If the `ExecutionPlan` does not support changing its partitioning,
returns `Ok(None)` (the default).

If the `ExecutionPlan` can increase its partitioning, but not to
`target_partitions`, it may return an ExecutionPlan with fewer
partitions. This might happen, for example, if each new partition would
be too small to be efficiently processed individually.

The DataFusion optimizer attempts to use as many threads as possible by
repartitioning its inputs to match the target number of threads
available (`target_partitions`). Some data sources, such as the built in
CSV and Parquet readers, implement this method as they are able to read
from their input files in parallel, regardless of how the source data is
split amongst files.

<a id="op-4e162e6df45a86327a682f22"></a>
## replace_children

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a clone of the existing plan with the children replaced,
skipping recomputation of plan properties when the options indicate
the new children's properties are unchanged.

Callers should typically call [`replace_children_if_necessary`](../operations/datafusion_physical_plan.execution_plan.replace_children_if_necessary.md#op-39321a5fcf4c72a03a82abd3) and
not invoke this method directly.

<a id="op-4675c0f905c4d7c995a86312"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Source: `src/execution_plan.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specifies simple per-child input distribution requirements.

Deprecated: override [`Self::input_distribution_requirements`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-8ef86f98da95d0ca77fa067e) instead.

By default, each child has [`Distribution::UnspecifiedDistribution`](../operations/datafusion_physical_expr.partitioning.Distribution.md#op-28e6f98afaf45f86586579b0).

<a id="op-7533f5db7cc088769c12d3cd"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Source: `src/execution_plan.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specifies the ordering required for all of the children of this
`ExecutionPlan`.

For each child, it's the local ordering requirement within
each partition rather than the global ordering

NOTE that checking `!is_empty()` does **not** check for a
required input ordering. Instead, the correct check is that at
least one entry must be `Some`

<a id="op-5c58755b3547082b3ec956bd"></a>
## reset_state

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reset any internal state within this [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673).

This method is called when an [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) needs to be re-executed,
such as in recursive queries. Unlike [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22), this method
ensures that any stateful components (e.g., [`DynamicFilterPhysicalExpr`])
are reset to their initial state.

The default implementation simply calls [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) with the existing children,
effectively creating a new instance of the [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) with the same children but without
necessarily resetting any internal state. Implementations that require resetting of some
internal state should override this method to provide the necessary logic.

This method should *not* reset state recursively for children, as it is expected that
it will be called from within a walk of the execution plan tree so that it will be called on each child later
or was already called on each child.

Note to implementers: unlike [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) this method does not accept new children as an argument,
thus it is expected that any cached plan properties will remain valid after the reset.

[`DynamicFilterPhysicalExpr`]: datafusion_physical_expr::expressions::DynamicFilterPhysicalExpr

<a id="op-f41258ad6bdf46680c8078d0"></a>
## schema

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Source: `src/execution_plan.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the schema for this execution plan

<a id="op-611293cb3547fb2990c052b9"></a>
## static_name

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::static_name` · datafusion-physical-plan 55.1.0

```rust
fn static_name() -> &'static str where Self: Sized
```

Source: `src/execution_plan.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Short name for the ExecutionPlan, such as 'DataSourceExec'.
Like [`name`](ExecutionPlan::name) but can be called without an instance.

<a id="op-7dad80ba3b610abc3c532d8a"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Source: `src/execution_plan.rs:750`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns statistics for a specific partition of this `ExecutionPlan` node,
given pre-computed child statistics.

If statistics are not available, should return [`Statistics::new_unknown`]
(the default), not an error.
If `args.partition()` is `None`, it returns statistics for all partitions.

Implementations should not call [`StatisticsContext::compute`] from within
this method; child statistics are provided via `input_stats`.

Use [`StatisticsContext::compute`] to initiate a full plan-tree walk.

[`StatisticsContext::compute`]: crate::statistics::StatisticsContext::compute

Unresolved upstream links (retained, not inferred): ``Statistics::new_unknown``.

<a id="op-ea707a92b537a6778bd9806f"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Source: `src/execution_plan.rs:784`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns `true` if a limit can be safely pushed down through this
`ExecutionPlan` node.

If this method returns `true`, and the query plan contains a limit at
the output of this node, DataFusion will push the limit to the input
of this node.

<a id="op-21a92548f5838420c106c4d2"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, _order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Source: `src/execution_plan.rs:991`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Try to push down sort ordering requirements to this node.

This method is called during sort pushdown optimization to determine if this
node can optimize for a requested sort ordering. Implementations should:

- Return [`SortOrderPushdownResult::Exact`](../operations/datafusion_physical_plan.sort_pushdown.SortOrderPushdownResult.md#op-ed428f58d9deea77196e0270) if the node can guarantee the exact
  ordering (allowing the Sort operator to be removed)
- Return [`SortOrderPushdownResult::Inexact`](../operations/datafusion_physical_plan.sort_pushdown.SortOrderPushdownResult.md#op-d76a90b56fe6654ae567f842) if the node can optimize for the
  ordering but cannot guarantee perfect sorting (Sort operator is kept)
- Return [`SortOrderPushdownResult::Unsupported`](../operations/datafusion_physical_plan.sort_pushdown.SortOrderPushdownResult.md#op-2741d2a02b139c9a94076044) if the node cannot optimize
  for the ordering

For transparent nodes (that preserve ordering), implement this to delegate to
children and wrap the result with a new instance of this node.

Default implementation returns `Unsupported`.

<a id="op-0255472dd64adf4edcc20768"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, _projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Source: `src/execution_plan.rs:816`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Attempts to push down the given projection into the input of this `ExecutionPlan`.

If the operator supports this optimization, the resulting plan will be:
`self_new <- projection <- source`, starting from `projection <- self <- source`.
Otherwise, it returns the current `ExecutionPlan` as-is.

Returns `Ok(Some(...))` if pushdown is applied, `Ok(None)` if it is not supported
or not possible, or `Err` on failure.

<a id="op-759ee1c728536c82a93b9a30"></a>
## try_to_proto

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, _ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Source: `src/execution_plan.rs:1025`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize this plan to its protobuf representation, if it knows how.

This is the `ExecutionPlan` analog of
[`PhysicalExpr::try_to_proto`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-3afb516a11e28f2442ad2f38).

* `Ok(None)` (the default) — "I don't serialize myself"; the caller
  (`datafusion-proto`) falls back to the central downcast chain. Every
  un-migrated plan keeps its existing behavior.
* `Ok(Some(node))` — fully serialized; the caller must not fall back.
* `Err(_)` — a real failure (e.g. a child failed to serialize).

Only *self-contained* plans should override this — see [`crate::proto`](../modules/datafusion_physical_plan.proto.md#op-003f7212d73dcf1424b8f88e)
for the session-dependency boundary.

<a id="op-9cea7d2e63946ce72cec4604"></a>
## with_fetch

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, _limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:794`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a fetching variant of this `ExecutionPlan` node, if it supports
fetch limits. Returns `None` otherwise.

See physical optimizer rule [`limit_pushdown`] for details.

[`limit_pushdown`]: https://docs.rs/datafusion/latest/datafusion/physical_optimizer/limit_pushdown/index.html

<a id="op-c94174b2f94c0eb5bf9111f2"></a>
## with_new_children

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deprecated.

DataFusion will remove this method in the future in favor of
[`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22).

Note that this method is still required by the trait; implementations
should delegate to [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) with
[`ChildrenPropertiesMode::Recompute`](../operations/datafusion_physical_plan.execution_plan.ChildrenPropertiesMode.md#op-449a09e8483cda87bd8b69c9).

# Example Implementation
```
# #![allow(deprecated)]
# use std::fmt;
# use std::sync::Arc;
# use datafusion_common::Result;
# use datafusion_common::tree_node::TreeNodeRecursion;
# use datafusion_execution::{SendableRecordBatchStream, TaskContext};
# use datafusion_physical_expr::PhysicalExpr;
# use datafusion_physical_plan::{
#     ChildrenPropertiesMode, DisplayAs, DisplayFormatType, ExecutionPlan,
#     PlanProperties, ReplaceChildrenOptions,
# };
# #[derive(Debug)]
# struct MyExec {
#     input: Arc<dyn ExecutionPlan>,
# }
# impl DisplayAs for MyExec {
#     fn fmt_as(&self, _t: DisplayFormatType, f: &mut fmt::Formatter) -> fmt::Result {
#         write!(f, "MyExec")
#     }
# }
impl ExecutionPlan for MyExec {
    // ...
#    fn name(&self) -> &'static str {
#        "MyExec"
#    }
#    fn properties(&self) -> &Arc<PlanProperties> {
#        self.input.properties()
#    }
#    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
#        vec![&self.input]
#    }
#    fn apply_expressions(
#        &self,
#        _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
#    ) -> Result<TreeNodeRecursion> {
#        Ok(TreeNodeRecursion::Continue)
#    }
#    fn execute(
#        &self,
#        _partition: usize,
#        _context: Arc<TaskContext>,
#    ) -> Result<SendableRecordBatchStream> {
#        unimplemented!()
#    }
    fn replace_children(
        self: Arc<Self>,
        mut children: Vec<Arc<dyn ExecutionPlan>>,
        _options: ReplaceChildrenOptions,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Ok(Arc::new(MyExec {
            input: children.swap_remove(0),
        }))
    }

    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        // call into `replace_children` with `ReplaceChildrenOptions`
        self.replace_children(
            children,
            ReplaceChildrenOptions::new(ChildrenPropertiesMode::Recompute),
        )
    }
}
```

<a id="op-9eee7e585a6a23c3142e8e1c"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:446`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deprecated. Implement [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22) instead.

<a id="op-773842958d794e1c6e26820a"></a>
## with_new_state

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::with_new_state` · datafusion-physical-plan 55.1.0

```rust
fn with_new_state(&self, _state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:968`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Injects arbitrary run-time state into this execution plan, returning a new plan
instance that incorporates that state *if* it is relevant to the concrete
node implementation.

This is a generic entry point: the `state` can be any type wrapped in
`Arc<dyn Any + Send + Sync>`.  A node that cares about the state should
down-cast it to the concrete type it expects and, if successful, return a
modified copy of itself that captures the provided value.  If the state is
not applicable, the default behaviour is to return `None` so that parent
nodes can continue propagating the attempt further down the plan tree.

For example, [`WorkTableExec`](crate::work_table::WorkTableExec)
down-casts the supplied state to an `Arc<WorkTable>`
in order to wire up the working table used during recursive-CTE execution.
Similar patterns can be followed by custom nodes that need late-bound
dependencies or shared state.

<a id="op-b16480f4379ff077223f79f9"></a>
## with_preserve_order

`function` · `datafusion_physical_plan::execution_plan::ExecutionPlan::with_preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_order(&self, _preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

Source: `src/execution_plan.rs:1004`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a variant of this `ExecutionPlan` that is aware of order-sensitivity.

This is used to signal to data sources that the output ordering must be
preserved, even if it might be more efficient to ignore it (e.g. by
skipping some row groups in Parquet).

