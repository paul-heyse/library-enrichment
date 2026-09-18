# `datafusion_datasource::source`

Crate `datafusion-datasource` · 3 public items · structured records in [`model/datafusion_datasource.source.json`](../model/datafusion_datasource.source.json)

## DataSourceExec

`struct` · `datafusion_datasource::source::DataSourceExec`

Also reachable as `datafusion::datasource::memory::DataSourceExec`, `datafusion::datasource::source::DataSourceExec`, `datafusion_catalog::memory::DataSourceExec`

```rust
struct DataSourceExec
```

**Implements**: `core::convert::From`, `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn data_source(&self) -> &Arc<dyn DataSource>
fn downcast_to_file_source<T: FileSource>(&self) -> Option<(&FileScanConfig, &T)>
fn from_data_source(data_source: impl DataSource + 'static) -> Arc<Self>
fn new(data_source: Arc<dyn DataSource>) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
fn with_data_source(self, data_source: Arc<dyn DataSource>) -> Self
fn with_partitioning(self, partitioning: Partitioning) -> Self
```

**via `core::convert::From`**

```rust
fn from(source: S) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_state(&self, state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn ExecutionPlan>>
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.source.DataSourceExec.md).


[`ExecutionPlan`] that reads one or more files

`DataSourceExec` implements common functionality such as applying
projections, and caching plan properties.

The [`DataSource`] describes where to find the data for this data source
(for example in files or what in memory partitions).

For file based [`DataSource`]s, format specific behavior is implemented in
the [`FileSource`] trait.

[`FileSource`]: crate::file::FileSource

---

## OpenArgs

`struct` · `datafusion_datasource::source::OpenArgs`

Also reachable as `datafusion::datasource::source::OpenArgs`

```rust
struct OpenArgs
```

**Fields**: `partition`, `context`, `sibling_state`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn new(partition: usize, context: Arc<TaskContext>) -> Self
fn with_shared_state(self, sibling_state: Option<Arc<dyn Any + Send + Sync>>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.source.OpenArgs.md).


Arguments for [`DataSource::open_with_args`]

---

## DataSource

`trait` · `datafusion_datasource::source::DataSource`

Also reachable as `datafusion::datasource::source::DataSource`

```rust
trait DataSource: Any + Send + Sync + Debug
```

**Implementors** (2)

- `datafusion_datasource::file_scan_config::FileScanConfig`
- `datafusion_datasource::memory::MemorySourceConfig`

**Methods** (19)

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn create_sibling_state(&self, _config: &ConfigOptions) -> Option<Arc<dyn Any + Send + Sync>>
fn eq_properties(&self) -> EquivalenceProperties
fn fetch(&self) -> Option<usize>
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
fn metrics(&self) -> ExecutionPlanMetricsSet
fn open(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn open_with_args(&self, args: OpenArgs) -> Result<SendableRecordBatchStream>
fn output_partitioning(&self) -> Partitioning
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
fn repartitioned(&self, _target_partitions: usize, _repartition_file_min_size: usize, _output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>>
fn scheduling_type(&self) -> SchedulingType
fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn DataSource>>>
fn try_pushdown_sort(&self, _order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn DataSource>>>
fn try_swapping_with_projection(&self, _projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>
fn try_to_proto(&self, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, _limit: Option<usize>) -> Option<Arc<dyn DataSource>>
fn with_new_state(&self, _state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn DataSource>>
fn with_preserve_order(&self, _preserve_order: bool) -> Option<Arc<dyn DataSource>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.source.DataSource.md).


A source of data, typically a list of files or memory

This trait provides common behaviors for abstract sources of data. It has
two common implementations:

1. [`FileScanConfig`]: lists of files
2. [`MemorySourceConfig`]: in memory list of `RecordBatch`

File format specific behaviors are defined by [`FileSource`]

# See Also
* [`FileSource`] for file format specific implementations (Parquet, Json, etc)
* [`DataSourceExec`]: The [`ExecutionPlan`] that reads from a `DataSource`

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

---
