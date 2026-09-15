# `datafusion_physical_plan::joins::symmetric_hash_join`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.joins.symmetric_hash_join.json`](../model/datafusion_physical_plan.joins.symmetric_hash_join.json)

## SymmetricHashJoinExec

`struct` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec`

Also reachable as `datafusion_physical_plan::joins::SymmetricHashJoinExec`

```rust
struct SymmetricHashJoinExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (12)

```rust
fn check_if_order_information_available(&self) -> Result<bool>
fn filter(&self) -> Option<&JoinFilter>
fn join_type(&self) -> &JoinType
fn left(&self) -> &Arc<dyn ExecutionPlan>
fn left_sort_exprs(&self) -> Option<&LexOrdering>
fn null_equality(&self) -> NullEquality
fn on(&self) -> &[(PhysicalExprRef, PhysicalExprRef)]
fn partition_mode(&self) -> StreamJoinPartitionMode
fn right(&self) -> &Arc<dyn ExecutionPlan>
fn right_sort_exprs(&self) -> Option<&LexOrdering>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: JoinOn, filter: Option<JoinFilter>, join_type: &JoinType, null_equality: NullEquality, left_sort_exprs: Option<LexOrdering>, right_sort_exprs: Option<LexOrdering>, mode: StreamJoinPartitionMode) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

A symmetric hash join with range conditions is when both streams are hashed on the
join key and the resulting hash tables are used to join the streams.
The join is considered symmetric because the hash table is built on the join keys from both
streams, and the matching of rows is based on the values of the join keys in both streams.
This type of join is efficient in streaming context as it allows for fast lookups in the hash
table, rather than having to scan through one or both of the streams to find matching rows, also it
only considers the elements from the stream that fall within a certain sliding window (w/ range conditions),
making it more efficient and less likely to store stale data. This enables operating on unbounded streaming
data without any memory issues.

For each input stream, create a hash table.
  - For each new [RecordBatch] in build side, hash and insert into inputs hash table. Update offsets.
  - Test if input is equal to a predefined set of other inputs.
  - If so record the visited rows. If the matched row results must be produced (INNER, LEFT), output the [RecordBatch].
  - Try to prune other side (probe) with new [RecordBatch].
  - If the join type indicates that the unmatched rows results must be produced (LEFT, FULL etc.),
    output the [RecordBatch] when a pruning happens or at the end of the data.


``` text
                       +-------------------------+
                       |                         |
  left stream ---------|  Left OneSideHashJoiner |---+
                       |                         |   |
                       +-------------------------+   |
                                                     |
                                                     |--------- Joined output
                                                     |
                       +-------------------------+   |
                       |                         |   |
 right stream ---------| Right OneSideHashJoiner |---+
                       |                         |
                       +-------------------------+

Prune build side when the new RecordBatch comes to the probe side. We utilize interval arithmetic
on JoinFilter's sorted PhysicalExprs to calculate the joinable range.


              PROBE SIDE          BUILD SIDE
                BUFFER              BUFFER
            +-------------+     +------------+
            |             |     |            |    Unjoinable
            |             |     |            |    Range
            |             |     |            |
            |             |  |---------------------------------
            |             |  |  |            |
            |             |  |  |            |
            |             | /   |            |
            |             | |   |            |
            |             | |   |            |
            |             | |   |            |
            |             | |   |            |
            |             | |   |            |    Joinable
            |             |/    |            |    Range
            |             ||    |            |
            |+-----------+||    |            |
            || Record    ||     |            |
            || Batch     ||     |            |
            |+-----------+||    |            |
            +-------------+\    +------------+
                            |
                            \
                             |---------------------------------

 This happens when range conditions are provided on sorted columns. E.g.

       SELECT * FROM left_table, right_table
       ON
         left_key = right_key AND
         left_time > right_time - INTERVAL 12 MINUTES AND left_time < right_time + INTERVAL 2 HOUR

or
      SELECT * FROM left_table, right_table
       ON
         left_key = right_key AND
         left_sorted > right_sorted - 3 AND left_sorted < right_sorted + 10

For general purpose, in the second scenario, when the new data comes to probe side, the conditions can be used to
determine a specific threshold for discarding rows from the inner buffer. For example, if the sort order the
two columns ("left_sorted" and "right_sorted") are ascending (it can be different in another scenarios)
and the join condition is "left_sorted > right_sorted - 3" and the latest value on the right input is 1234, meaning
that the left side buffer must only keep rows where "leftTime > rightTime - 3 > 1234 - 3 > 1231" ,
making the smallest value in 'left_sorted' 1231 and any rows below (since ascending)
than that can be dropped from the inner buffer.
```

---
