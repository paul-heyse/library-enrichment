# `datafusion_physical_plan::aggregates`

Crate `datafusion-physical-plan` · 15 public items · structured records in [`model/datafusion_physical_plan.aggregates.json`](../model/datafusion_physical_plan.aggregates.json)

## AggregateInputMode

`enum` · `datafusion_physical_plan::aggregates::AggregateInputMode`

```rust
enum AggregateInputMode
```

**Variants**: `Raw`, `Partial`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.AggregateInputMode.md).


Whether an aggregate stage consumes raw input data or intermediate
accumulator state from a previous aggregation stage.

See the [table on `AggregateMode`](AggregateMode#variants-and-their-inputoutput-modes)
for how this relates to aggregate modes.

---

## AggregateMode

`enum` · `datafusion_physical_plan::aggregates::AggregateMode`

```rust
enum AggregateMode
```

**Variants**: `Partial`, `Final`, `FinalPartitioned`, `Single`, `SinglePartitioned`, `PartialReduce`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn input_mode(&self) -> AggregateInputMode
fn output_mode(&self) -> AggregateOutputMode
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.AggregateMode.md).


Aggregation modes

See [`Accumulator::state`] for background information on multi-phase
aggregation and how these modes are used.

# Variants and their input/output modes

Each variant can be characterized by its [`AggregateInputMode`] and
[`AggregateOutputMode`]:

```text
                      | Input: Raw data           | Input: Partial state
Output: Final values  | Single, SinglePartitioned | Final, FinalPartitioned
Output: Partial state | Partial                   | PartialReduce
```

Use [`AggregateMode::input_mode`] and [`AggregateMode::output_mode`]
to query these properties.

---

## AggregateOutputMode

`enum` · `datafusion_physical_plan::aggregates::AggregateOutputMode`

```rust
enum AggregateOutputMode
```

**Variants**: `Partial`, `Final`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.AggregateOutputMode.md).


Whether an aggregate stage produces intermediate accumulator state
or final output values.

See the [table on `AggregateMode`](AggregateMode#variants-and-their-inputoutput-modes)
for how this relates to aggregate modes.

---

## aggregate_expressions

`function` · `datafusion_physical_plan::aggregates::aggregate_expressions`

```rust
fn aggregate_expressions(aggr_expr: &[std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>], mode: &AggregateMode, col_idx_base: usize) -> datafusion_common::Result<Vec<Vec<std::sync::Arc<dyn PhysicalExpr>>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.aggregate_expressions.md).


Returns physical expressions for arguments to evaluate against a batch.

The expressions are different depending on `mode`:
* Partial: AggregateFunctionExpr::expressions
* Final: columns of `AggregateFunctionExpr::state_fields()`

---

## concat_slices

`function` · `datafusion_physical_plan::aggregates::concat_slices`

```rust
fn concat_slices<T: Clone>(lhs: &[T], rhs: &[T]) -> Vec<T>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.concat_slices.md).


Concatenates the given slices.

---

## create_accumulators

`function` · `datafusion_physical_plan::aggregates::create_accumulators`

```rust
fn create_accumulators(aggr_expr: &[std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>]) -> datafusion_common::Result<Vec<AccumulatorItem>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.create_accumulators.md).


---

## evaluate_group_by

`function` · `datafusion_physical_plan::aggregates::evaluate_group_by`

```rust
fn evaluate_group_by(group_by: &PhysicalGroupBy, batch: &arrow::record_batch::RecordBatch) -> datafusion_common::Result<Vec<Vec<arrow::array::ArrayRef>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.evaluate_group_by.md).


Evaluate a group by expression against a `RecordBatch`

Arguments:
- `group_by`: the expression to evaluate
- `batch`: the `RecordBatch` to evaluate against

Returns: A Vec of Vecs of Array of results
The outer Vec appears to be for grouping sets
The inner Vec contains the results per expression
The inner-inner Array contains the results per row

For example, for `GROUP BY GROUPING SETS ((a, b), (a))` with input:

```text
a  b
1  1
1  2
2  1
```

The output is:

```text
[
  [
    a:           [1, 1, 2]
    b:           [1, 2, 1]
    grouping_id: [0, 0, 0]
  ],
  [
    a:           [1, 1, 2]
    b:           [NULL, NULL, NULL]
    grouping_id: [1, 1, 1]
  ]
]
```

---

## evaluate_many

`function` · `datafusion_physical_plan::aggregates::evaluate_many`

```rust
fn evaluate_many(expr: &[Vec<std::sync::Arc<dyn PhysicalExpr>>], batch: &arrow::record_batch::RecordBatch) -> datafusion_common::Result<Vec<Vec<arrow::array::ArrayRef>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.evaluate_many.md).


Evaluates groups of expressions against a record batch.

---

## finalize_aggregation

`function` · `datafusion_physical_plan::aggregates::finalize_aggregation`

```rust
fn finalize_aggregation(accumulators: &mut [AccumulatorItem], mode: &AggregateMode) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.finalize_aggregation.md).


returns a vector of ArrayRefs, where each entry corresponds to either the
final value (mode = Final, FinalPartitioned and Single) or states (mode = Partial)

---

## get_finer_aggregate_exprs_requirement

`function` · `datafusion_physical_plan::aggregates::get_finer_aggregate_exprs_requirement`

```rust
fn get_finer_aggregate_exprs_requirement(aggr_exprs: &mut [std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>], group_by: &PhysicalGroupBy, eq_properties: &datafusion_physical_expr::EquivalenceProperties, agg_mode: &AggregateMode) -> datafusion_common::Result<Vec<datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.get_finer_aggregate_exprs_requirement.md).


Gets the common requirement that satisfies all the aggregate expressions.
When possible, chooses the requirement that is already satisfied by the
equivalence properties.

# Parameters

- `aggr_exprs`: A slice of `AggregateFunctionExpr` containing all the
  aggregate expressions.
- `group_by`: A reference to a `PhysicalGroupBy` instance representing the
  physical GROUP BY expression.
- `eq_properties`: A reference to an `EquivalenceProperties` instance
  representing equivalence properties for ordering.
- `agg_mode`: A reference to an `AggregateMode` instance representing the
  mode of aggregation.

# Returns

A `Result<Vec<PhysicalSortRequirement>>` instance, which is the requirement
that satisfies all the aggregate requirements. Returns an error in case of
conflicting requirements.

---

## topk_types_supported

`function` · `datafusion_physical_plan::aggregates::topk_types_supported`

```rust
fn topk_types_supported(key_type: &arrow::datatypes::DataType, value_type: &arrow::datatypes::DataType) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.topk_types_supported.md).


Returns true if TopK aggregation data structures support the provided key and value types.

This function checks whether both the key type (used for grouping) and value type
(used in min/max aggregation) can be handled by the TopK aggregation heap and hash table.
Supported types include Arrow primitives (integers, floats, decimals, intervals) and
UTF-8 strings (`Utf8`, `LargeUtf8`, `Utf8View`).
```text

---

## AggregateExec

`struct` · `datafusion_physical_plan::aggregates::AggregateExec`

```rust
struct AggregateExec
```

**Fields**: `input`, `input_schema`

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (20)

```rust
fn aggr_expr(&self) -> &[Arc<AggregateFunctionExpr>]
fn cache(&self) -> &PlanProperties
fn compute_properties(input: &Arc<dyn ExecutionPlan>, schema: SchemaRef, group_expr_mapping: &ProjectionMapping, is_true_no_grouping: bool, mode: &AggregateMode, input_order_mode: &InputOrderMode, aggr_exprs: &[Arc<AggregateFunctionExpr>]) -> Result<PlanProperties>
fn dynamic_filter_expr(&self) -> Option<&Arc<DynamicFilterPhysicalExpr>>
fn filter_expr(&self) -> &[Option<Arc<dyn PhysicalExpr>>]
fn get_minmax_desc(&self) -> Option<(FieldRef, bool)>
fn group_expr(&self) -> &PhysicalGroupBy
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn input_order_mode(&self) -> &InputOrderMode
fn input_schema(&self) -> SchemaRef
fn is_unordered_unfiltered_group_by_distinct(&self) -> bool
fn limit_options(&self) -> Option<LimitOptions>
fn mode(&self) -> &AggregateMode
fn output_group_expr(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(mode: AggregateMode, group_by: impl Into<Arc<PhysicalGroupBy>>, aggr_expr: Vec<Arc<AggregateFunctionExpr>>, filter_expr: Vec<Option<Arc<dyn PhysicalExpr>>>, input: Arc<dyn ExecutionPlan>, input_schema: SchemaRef) -> Result<Self>
fn with_dynamic_filter_expr(self, filter: Arc<DynamicFilterPhysicalExpr>) -> Result<Self>
fn with_limit_options(self, limit_options: Option<LimitOptions>) -> Self
fn with_new_aggr_exprs(&self, aggr_expr: impl Into<Arc<[Arc<AggregateFunctionExpr>]>>) -> Self
fn with_new_limit_options(&self, limit_options: Option<LimitOptions>) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn cardinality_effect(&self) -> CardinalityEffect
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.AggregateExec.md).


Hash aggregate execution plan

---

## LimitOptions

`struct` · `datafusion_physical_plan::aggregates::LimitOptions`

```rust
struct LimitOptions
```

**Fields**: `limit`, `descending`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn descending(&self) -> Option<bool>
fn limit(&self) -> usize
fn new(limit: usize) -> Self
fn new_with_order(limit: usize, descending: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.LimitOptions.md).


Configuration for limit-based optimizations in aggregation

---

## PhysicalGroupBy

`struct` · `datafusion_physical_plan::aggregates::PhysicalGroupBy`

```rust
struct PhysicalGroupBy
```

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (15)

```rust
fn as_final(&self) -> PhysicalGroupBy
fn expr(&self) -> &[(Arc<dyn PhysicalExpr>, String)]
fn exprs_nullable(&self) -> Vec<bool>
fn group_schema(&self, schema: &Schema) -> Result<SchemaRef>
fn groups(&self) -> &[Vec<bool>]
fn has_grouping_set(&self) -> bool
fn input_exprs(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn is_empty(&self) -> bool
fn is_single(&self) -> bool
fn is_true_no_grouping(&self) -> bool
fn new(expr: Vec<(Arc<dyn PhysicalExpr>, String)>, null_expr: Vec<(Arc<dyn PhysicalExpr>, String)>, groups: Vec<Vec<bool>>, has_grouping_set: bool) -> Self
fn new_single(expr: Vec<(Arc<dyn PhysicalExpr>, String)>) -> Self
fn null_expr(&self) -> &[(Arc<dyn PhysicalExpr>, String)]
fn num_group_exprs(&self) -> usize
fn output_exprs(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.PhysicalGroupBy.md).


Represents `GROUP BY` clause in the plan (including the more general GROUPING SET)
In the case of a simple `GROUP BY a, b` clause, this will contain the expression [a, b]
and a single group [false, false].
In the case of `GROUP BY GROUPING SETS/CUBE/ROLLUP` the planner will expand the expression
into multiple groups, using null expressions to align each group.
For example, with a group by clause `GROUP BY GROUPING SETS ((a,b),(a),(b))` the planner should
create a `PhysicalGroupBy` like
```text
PhysicalGroupBy {
    expr: [(col(a), a), (col(b), b)],
    null_expr: [(NULL, a), (NULL, b)],
    groups: [
        [false, false], // (a,b)
        [false, true],  // (a) <=> (a, NULL)
        [true, false]   // (b) <=> (NULL, b)
    ]
}
```

---

## AccumulatorItem

`type_alias` · `datafusion_physical_plan::aggregates::AccumulatorItem`

```rust
type AccumulatorItem = Box<dyn Accumulator>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.AccumulatorItem.md).


---
