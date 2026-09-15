# `datafusion_physical_plan::explain`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.explain.json`](../model/datafusion_physical_plan.explain.json)

## ExplainExec

`struct` · `datafusion_physical_plan::explain::ExplainExec`

```rust
struct ExplainExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn new(schema: SchemaRef, stringified_plans: Vec<StringifiedPlan>, verbose: bool) -> Self
fn stringified_plans(&self) -> &[StringifiedPlan]
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, _ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
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
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn try_to_proto(&self, _ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Explain execution plan operator. This operator contains the string
values of the various plans it has when it is created, and passes
them to its output.

---
