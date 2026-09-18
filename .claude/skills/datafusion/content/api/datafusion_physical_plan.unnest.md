# `datafusion_physical_plan::unnest`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.unnest.json`](../model/datafusion_physical_plan.unnest.json)

## ListUnnest

`struct` · `datafusion_physical_plan::unnest::ListUnnest`

```rust
struct ListUnnest
```

**Fields**: `index_in_input_schema`, `depth`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.unnest.ListUnnest.md).


---

## UnnestExec

`struct` · `datafusion_physical_plan::unnest::UnnestExec`

```rust
struct UnnestExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn list_column_indices(&self) -> &[ListUnnest]
fn new(input: Arc<dyn ExecutionPlan>, list_column_indices: Vec<ListUnnest>, struct_column_indices: Vec<usize>, schema: SchemaRef, options: UnnestOptions) -> Result<Self>
fn options(&self) -> &UnnestOptions
fn struct_column_indices(&self) -> &[usize]
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
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
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.unnest.UnnestExec.md).


Unnest the given columns (either with type struct or list)
For list unnesting, each row is vertically transformed into multiple rows
For struct unnesting, each column is horizontally transformed into multiple columns,
Thus the original RecordBatch with dimension (n x m) may have new dimension (n' x m')

See [`UnnestOptions`] for more details and an example.

---
