# `datafusion_physical_plan::analyze`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.analyze.json`](../model/datafusion_physical_plan.analyze.json)

## AnalyzeExec

`struct` · `datafusion_physical_plan::analyze::AnalyzeExec`

```rust
struct AnalyzeExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn builder(verbose: bool, show_statistics: bool, input: Arc<dyn ExecutionPlan>, schema: SchemaRef) -> AnalyzeExecBuilder
fn format(&self) -> &ExplainFormat
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn metric_categories(&self) -> Option<&[MetricCategory]>
fn show_statistics(&self) -> bool
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn verbose(&self) -> bool
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

`EXPLAIN ANALYZE` execution plan operator. This operator runs its input,
discards the results, and then prints out an annotated plan with metrics

---

## AnalyzeExecBuilder

`struct` · `datafusion_physical_plan::analyze::AnalyzeExecBuilder`

```rust
struct AnalyzeExecBuilder
```

**Methods** (5)

```rust
fn build(self) -> AnalyzeExec
fn new(verbose: bool, show_statistics: bool, input: Arc<dyn ExecutionPlan>, schema: SchemaRef) -> Self
fn with_format(self, format: ExplainFormat) -> Self
fn with_metric_categories(self, metric_categories: Option<Vec<MetricCategory>>) -> Self
fn with_metric_types(self, metric_types: Vec<MetricType>) -> Self
```

Builder for [`AnalyzeExec`].

Builder for [AnalyzeExec].

---
