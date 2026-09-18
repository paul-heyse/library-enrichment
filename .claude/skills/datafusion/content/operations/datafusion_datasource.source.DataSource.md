# `datafusion_datasource::source::DataSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.source.DataSource.json).

<a id="op-9da841d166501739a52bf7ac"></a>
## DataSource

`trait` · `datafusion_datasource::source::DataSource` · datafusion-datasource 55.1.0

```rust
trait DataSource: Any + Send + Sync + Debug
```

Source: `src/source.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A source of data, typically a list of files or memory

This trait provides common behaviors for abstract sources of data. It has
two common implementations:

1. [`FileScanConfig`]: lists of files
2. [`MemorySourceConfig`]: in memory list of `RecordBatch`

File format specific behaviors are defined by [`FileSource`]

# See Also
* [`FileSource`] for file format specific implementations (Parquet, Json, etc)
* [`DataSourceExec`](../operations/datafusion_datasource.source.DataSourceExec.md#op-96b0a6eef9f3c044c580d9b6): The [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) that reads from a `DataSource`

# Notes

Requires `Debug` to assist debugging

[`FileScanConfig`]: https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.FileScanConfig.html
[`MemorySourceConfig`]: https://docs.rs/datafusion/latest/datafusion/datasource/memory/struct.MemorySourceConfig.html
[`FileSource`]: crate::file::FileSource
[`FileFormat``]: https://docs.rs/datafusion/latest/datafusion/datasource/file_format/index.html
[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

The following diagram shows how DataSource, FileSource, and DataSourceExec are related
```text
                      ┌─────────────────────┐                              -----► execute path
                      │                     │                              ┄┄┄┄┄► init path
                      │   DataSourceExec    │
                      │                     │
                      └───────▲─────────────┘
                              ┊  │
                              ┊  │
                      ┌──────────▼──────────┐                            ┌──────────-──────────┐
                      │                     │                            |                     |
                      │  DataSource(trait)  │                            | TableProvider(trait)|
                      │                     │                            |                     |
                      └───────▲─────────────┘                            └─────────────────────┘
                              ┊  │                                                  ┊
              ┌───────────────┿──┴────────────────┐                                 ┊
              |   ┌┄┄┄┄┄┄┄┄┄┄┄┘                   |                                 ┊
              |   ┊                               |                                 ┊
   ┌──────────▼──────────┐             ┌──────────▼──────────┐                      ┊
   │                     │             │                     │           ┌──────────▼──────────┐
   │   FileScanConfig    │             │ MemorySourceConfig  │           |                     |
   │                     │             │                     │           |  FileFormat(trait)  |
   └──────────────▲──────┘             └─────────────────────┘           |                     |
              │   ┊                                                      └─────────────────────┘
              │   ┊                                                                 ┊
              │   ┊                                                                 ┊
   ┌──────────▼──────────┐                                               ┌──────────▼──────────┐
   │                     │                                               │     ArrowSource     │
   │ FileSource(trait)   ◄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄│          ...        │
   │                     │                                               │    ParquetSource    │
   └─────────────────────┘                                               └─────────────────────┘
              │
              │
              │
              │
   ┌──────────▼──────────┐
   │     ArrowSource     │
   │          ...        │
   │    ParquetSource    │
   └─────────────────────┘
              |
FileOpener (called by FileStream)
              │
   ┌──────────▼──────────┐
   │                     │
   │     RecordBatch     │
   │                     │
   └─────────────────────┘
```

<a id="op-a9386dc63a798752a3982bf7"></a>
## apply_expressions

`function` · `datafusion_datasource::source::DataSource::apply_expressions` · datafusion-datasource 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Source: `src/source.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Apply a closure to each expression used by this data source.

This includes filter predicates (which may contain dynamic filters) and any
other expressions used during data scanning.

The function `f` should be called once per expression unless the function returns
[`TreeNodeRecursion::Stop`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-fbbedd1f0409ac786df535a1) to stop iteration.

See [`ExecutionPlan::apply_expressions`] for more details and implementation examples.

[`ExecutionPlan::apply_expressions`]: datafusion_physical_plan::ExecutionPlan::apply_expressions

<a id="op-b6f45e11fb485500af46863a"></a>
## create_sibling_state

`function` · `datafusion_datasource::source::DataSource::create_sibling_state` · datafusion-datasource 55.1.0

```rust
fn create_sibling_state(&self, _config: &ConfigOptions) -> Option<Arc<dyn Any + Send + Sync>>
```

Source: `src/source.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create per execution state to share across sibling instances of this
data source during one execution.

`config` is the session configuration, so implementations can honor
options that disable sibling sharing (returning `None`) for consumers
that cannot poll all partitions in one process.

Returns `None` (the default) if this data source has
no sibling-shared execution state.

<a id="op-2b554dbb169d271e73401185"></a>
## eq_properties

`function` · `datafusion_datasource::source::DataSource::eq_properties` · datafusion-datasource 55.1.0

```rust
fn eq_properties(&self) -> EquivalenceProperties
```

Source: `src/source.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47486394fb3eb1ea2747ba2c"></a>
## fetch

`function` · `datafusion_datasource::source::DataSource::fetch` · datafusion-datasource 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Source: `src/source.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08b8104c011d85ba665d4c90"></a>
## fmt_as

`function` · `datafusion_datasource::source::DataSource::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

Source: `src/source.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Format this source for display in explain plans

<a id="op-09896726288c96fe1a329aa3"></a>
## metrics

`function` · `datafusion_datasource::source::DataSource::metrics` · datafusion-datasource 55.1.0

```rust
fn metrics(&self) -> ExecutionPlanMetricsSet
```

Source: `src/source.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d627dadd6b2728f6428958e"></a>
## open

`function` · `datafusion_datasource::source::DataSource::open` · datafusion-datasource 55.1.0

```rust
fn open(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Source: `src/source.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Open the specified output partition and return its stream of
[`RecordBatch`]es.

This should be used by data sources that do not need any sibling
coordination. Data sources that want to use per-execution shared state
(for example, to reorder work across partitions at runtime) should
implement [`Self::open_with_args`](../operations/datafusion_datasource.source.DataSource.md#op-f4650a7c687d1d6b4d5c6906) instead.

[`RecordBatch`]: arrow::record_batch::RecordBatch

<a id="op-f4650a7c687d1d6b4d5c6906"></a>
## open_with_args

`function` · `datafusion_datasource::source::DataSource::open_with_args` · datafusion-datasource 55.1.0

```rust
fn open_with_args(&self, args: OpenArgs) -> Result<SendableRecordBatchStream>
```

Source: `src/source.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Open a partition using optional sibling-shared execution state.

The default implementation ignores the additional state and delegates to
[`Self::open`](../operations/datafusion_datasource.source.DataSource.md#op-8d627dadd6b2728f6428958e).

<a id="op-732f35305d718530d0568cdf"></a>
## output_partitioning

`function` · `datafusion_datasource::source::DataSource::output_partitioning` · datafusion-datasource 55.1.0

```rust
fn output_partitioning(&self) -> Partitioning
```

Source: `src/source.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ceeebb95758d8c2494c1f1ff"></a>
## partition_statistics

`function` · `datafusion_datasource::source::DataSource::partition_statistics` · datafusion-datasource 55.1.0

```rust
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
```

Source: `src/source.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns statistics for a specific partition, or aggregate statistics
across all partitions if `partition` is `None`.

<a id="op-2c6fe409f79db273197b6979"></a>
## repartitioned

`function` · `datafusion_datasource::source::DataSource::repartitioned` · datafusion-datasource 55.1.0

```rust
fn repartitioned(&self, _target_partitions: usize, _repartition_file_min_size: usize, _output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>>
```

Source: `src/source.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a copy of this DataSource with a new partitioning scheme.

Returns `Ok(None)` (the default) if the partitioning cannot be changed.
Refer to [`ExecutionPlan::repartitioned`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-584645e19b0573a60b990c36) for details on when None should be returned.

Repartitioning should not change the output ordering, if this ordering exists.
Refer to [`MemorySourceConfig::repartition_preserving_order`](crate::memory::MemorySourceConfig)
and the FileSource's
[`FileGroupPartitioner::repartition_file_groups`](crate::file_groups::FileGroupPartitioner::repartition_file_groups)
for examples.

<a id="op-052d0dfe7cfb2511ed748a83"></a>
## scheduling_type

`function` · `datafusion_datasource::source::DataSource::scheduling_type` · datafusion-datasource 55.1.0

```rust
fn scheduling_type(&self) -> SchedulingType
```

Source: `src/source.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769946bfa5adbdf2ffdec38a"></a>
## try_pushdown_filters

`function` · `datafusion_datasource::source::DataSource::try_pushdown_filters` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn DataSource>>>
```

Source: `src/source.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Try to push down filters into this DataSource.

These filters are in terms of the output schema of this DataSource (e.g.
[`Self::eq_properties`](../operations/datafusion_datasource.source.DataSource.md#op-2b554dbb169d271e73401185) and output of any projections pushed into the
source), not the original table schema.

See [`ExecutionPlan::handle_child_pushdown_result`] for more details.

[`ExecutionPlan::handle_child_pushdown_result`]: datafusion_physical_plan::ExecutionPlan::handle_child_pushdown_result

<a id="op-69063c56b8149bff4bcee7e3"></a>
## try_pushdown_sort

`function` · `datafusion_datasource::source::DataSource::try_pushdown_sort` · datafusion-datasource 55.1.0

```rust
fn try_pushdown_sort(&self, _order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn DataSource>>>
```

Source: `src/source.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Try to create a new DataSource that produces data in the specified sort order.

# Arguments
* `order` - The desired output ordering

# Returns
* `Ok(SortOrderPushdownResult::Exact { .. })` - Created a source that guarantees exact ordering
* `Ok(SortOrderPushdownResult::Inexact { .. })` - Created a source optimized for the ordering
* `Ok(SortOrderPushdownResult::Unsupported)` - Cannot optimize for this ordering
* `Err(e)` - Error occurred

Default implementation returns `Unsupported`.

<a id="op-f531f93b53bfc42c1d65d85c"></a>
## try_swapping_with_projection

`function` · `datafusion_datasource::source::DataSource::try_swapping_with_projection` · datafusion-datasource 55.1.0

```rust
fn try_swapping_with_projection(&self, _projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>
```

Source: `src/source.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8bcd9fab3832ccbdeea97ca"></a>
## try_to_proto

`function` · `datafusion_datasource::source::DataSource::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Source: `src/source.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Serialize this data source to a full [`PhysicalPlanNode`] (a
`DataSourceExec` wrapping this source), if it knows how.

This is the `DataSource` analog of
[`ExecutionPlan::try_to_proto`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-759ee1c728536c82a93b9a30).
[`DataSourceExec::try_to_proto`](crate::source::DataSourceExec) delegates
to this hook, which for file scans forwards to
[`FileSource::try_to_proto`](../operations/datafusion_datasource.file.FileSource.md#op-83152ed75bb6a4e441648c13)
through the shared [`FileScanConfig`](../operations/datafusion_datasource.file_scan_config.FileScanConfig.md#op-d7aa66cd63851cc944f55f57)
spine.

* `Ok(None)` (the default) — "I don't serialize myself"; the caller falls
  back to the central downcast chain in `datafusion-proto`.
* `Ok(Some(node))` — fully serialized; the caller must not fall back.

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode

<a id="op-e8cf31edd117b34143385fdb"></a>
## with_fetch

`function` · `datafusion_datasource::source::DataSource::with_fetch` · datafusion-datasource 55.1.0

```rust
fn with_fetch(&self, _limit: Option<usize>) -> Option<Arc<dyn DataSource>>
```

Source: `src/source.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a copy of this DataSource with a new fetch limit

<a id="op-46514af1f2e6865771fb652e"></a>
## with_new_state

`function` · `datafusion_datasource::source::DataSource::with_new_state` · datafusion-datasource 55.1.0

```rust
fn with_new_state(&self, _state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn DataSource>>
```

Source: `src/source.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Injects arbitrary run-time state into this DataSource, returning a new instance
that incorporates that state *if* it is relevant to the concrete DataSource implementation.

This is a generic entry point: the `state` can be any type wrapped in
`Arc<dyn Any + Send + Sync>`.  A data source that cares about the state should
down-cast it to the concrete type it expects and, if successful, return a
modified copy of itself that captures the provided value.  If the state is
not applicable, the default behaviour is to return `None` so that parent
nodes can continue propagating the attempt further down the plan tree.

<a id="op-e87a241e81ad10f19bd0ac48"></a>
## with_preserve_order

`function` · `datafusion_datasource::source::DataSource::with_preserve_order` · datafusion-datasource 55.1.0

```rust
fn with_preserve_order(&self, _preserve_order: bool) -> Option<Arc<dyn DataSource>>
```

Source: `src/source.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns a variant of this `DataSource` that is aware of order-sensitivity.
