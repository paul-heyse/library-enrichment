# `deltalake_core::delta_datafusion::table_provider::next::scan::exec`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.delta_datafusion.table_provider.next.scan.exec.json`](../model/deltalake_core.delta_datafusion.table_provider.next.scan.exec.json)

## DeltaScanExec

`struct` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.table_provider.next.scan.exec.DeltaScanExec.md)

Also reachable as `deltalake::delta_datafusion::DeltaScanExec`, `deltalake_core::delta_datafusion::DeltaScanExec`

```rust
struct DeltaScanExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _expr_rewriter: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion, DataFusionError>) -> Result<TreeNodeRecursion, DataFusionError>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn repartitioned(&self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Physical execution plan for scanning Delta tables.

Wraps a Parquet reader execution plan and applies Delta Lake protocol transformations
to produce the logical table data. This includes:

- **Column mapping**: Translates physical column names to logical names
- **Partition values**: Materializes partition column values from file paths
- **Deletion vectors**: Filters out deleted rows using per-file selection vectors
- **Schema evolution**: Handles missing columns and type coercion

# Data Flow

1. Inner [`input`](Self::input) plan reads raw Parquet data
2. Per-file [`transforms`](Self::transforms) convert physical to logical schema
3. The scan applies deletion vectors before it returns rows
4. Result is cast to the projected scan contract's result schema

---
