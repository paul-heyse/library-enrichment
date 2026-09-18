# `datafusion_physical_plan::joins::hash_join::exec`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.joins.hash_join.exec.json`](../model/datafusion_physical_plan.joins.hash_join.exec.json)

## HashJoinExec

`struct` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec`

Also reachable as `datafusion_physical_plan::joins::HashJoinExec`

```rust
struct HashJoinExec
```

**Fields**: `left`, `right`, `on`, `filter`, `join_type`, `mode`, `projection`, `null_equality`, `null_aware`

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`, `datafusion_physical_plan::projection::EmbeddedProjection`

**Derives**: Debug

**Methods** (17)

```rust
fn builder(&self) -> HashJoinExecBuilder
fn contains_projection(&self) -> bool
fn dynamic_filter_expr(&self) -> Option<&Arc<DynamicFilterPhysicalExpr>>
fn filter(&self) -> Option<&JoinFilter>
fn join_schema(&self) -> &SchemaRef
fn join_type(&self) -> &JoinType
fn left(&self) -> &Arc<dyn ExecutionPlan>
fn null_equality(&self) -> NullEquality
fn on(&self) -> &[(PhysicalExprRef, PhysicalExprRef)]
fn partition_mode(&self) -> &PartitionMode
fn probe_side() -> JoinSide
fn right(&self) -> &Arc<dyn ExecutionPlan>
fn swap_inputs(&self, partition_mode: PartitionMode) -> Result<Arc<dyn ExecutionPlan>>
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: JoinOn, filter: Option<JoinFilter>, join_type: &JoinType, projection: Option<Vec<usize>>, partition_mode: PartitionMode, null_equality: NullEquality, null_aware: bool) -> Result<Self>
fn with_dynamic_filter_expr(self, filter: Arc<DynamicFilterPhysicalExpr>) -> Result<Self>
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &ConfigOptions) -> Result<FilterDescription>
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn maintains_input_order(&self) -> Vec<bool>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn supports_limit_pushdown(&self) -> bool
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

**via `datafusion_physical_plan::projection::EmbeddedProjection`**

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md).


Join execution plan: Evaluates equijoin predicates in parallel on multiple
partitions using a hash table and an optional filter list to apply post
join.

# Join Expressions

This implementation is optimized for evaluating equijoin predicates  (
`<col1> = <col2>`) expressions, which are represented as a list of `Columns`
in [`Self::on`].

Non-equality predicates, which can not pushed down to a join inputs (e.g.
`<col1> != <col2>`) are known as "filter expressions" and are evaluated
after the equijoin predicates.

# ArrayMap Optimization

For joins with a single integer-based join key, `HashJoinExec` may use an [`ArrayMap`]
(also known as a "perfect hash join") instead of a general-purpose hash map.
This optimization is used when:
1. There is exactly one join key.
2. The join key is an integer type up to 64 bits wide that can be losslessly converted
   to `u64` (128-bit integer types such as `i128` and `u128` are not supported).
3. The range of keys is small enough (controlled by `perfect_hash_join_small_build_threshold`)
   OR the keys are sufficiently dense (controlled by `perfect_hash_join_min_key_density`).
4. build_side.num_rows() < u32::MAX
5. NullEqualsNothing || (NullEqualsNull && build side doesn't contain null)

See [`try_create_array_map`] for more details.

Note that when using [`PartitionMode::Partitioned`], the build side is split into multiple
partitions. This can cause a dense build side to become sparse within each partition,
potentially disabling this optimization.

For example, consider:
```sql
SELECT t1.value, t2.value
FROM range(10000) AS t1
JOIN range(10000) AS t2
  ON t1.value = t2.value;
```
With 24 partitions, each partition will only receive a subset of the 10,000 rows.
The first partition might contain values like `3, 10, 18, 39, 43`, which are sparse
relative to the original range, even though the overall data set is dense.

# "Build Side" vs "Probe Side"

HashJoin takes two inputs, which are referred to as the "build" and the
"probe". The build side is the first child, and the probe side is the second
child.

The two inputs are treated differently and it is VERY important that the
*smaller* input is placed on the build side to minimize the work of creating
the hash table.

```text
         ┌───────────┐
         │ HashJoin  │
         │           │
         └───────────┘
             │   │
       ┌─────┘   └─────┐
       ▼               ▼
┌────────────┐  ┌─────────────┐
│   Input    │  │    Input    │
│    [0]     │  │     [1]     │
└────────────┘  └─────────────┘

 "build side"    "probe side"
```

Execution proceeds in 2 stages:

1. the **build phase** creates a hash table from the tuples of the build side,
   and single concatenated batch containing data from all fetched record batches.
   Resulting hash table stores hashed join-key fields for each row as a key, and
   indices of corresponding rows in concatenated batch.

When using the standard `JoinHashMap`, hash join uses LIFO data structure as a hash table,
and in order to retain original build-side input order while obtaining data during probe phase,
hash table is updated by iterating batch sequence in reverse order -- it allows to
keep rows with smaller indices "on the top" of hash table, and still maintain
correct indexing for concatenated build-side data batch.

Example of build phase for 3 record batches:


```text

 Original build-side data   Inserting build-side values into hashmap    Concatenated build-side batch
                                                                        ┌───────────────────────────┐
                            hashmap.insert(row-hash, row-idx + offset)  │                      idx  │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 1 │        1) update_hash for batch 3 with offset 0    │          │ Row 6 │    0   │
  Batch 1  │       │           - hashmap.insert(Row 7, idx 1)           │ Batch 3  │       │        │
           │ Row 2 │           - hashmap.insert(Row 6, idx 0)           │          │ Row 7 │    1   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 3 │        2) update_hash for batch 2 with offset 2    │          │ Row 3 │    2   │
           │       │           - hashmap.insert(Row 5, idx 4)           │          │       │        │
  Batch 2  │ Row 4 │           - hashmap.insert(Row 4, idx 3)           │ Batch 2  │ Row 4 │    3   │
           │       │           - hashmap.insert(Row 3, idx 2)           │          │       │        │
           │ Row 5 │                                                    │          │ Row 5 │    4   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
           ┌───────┐                                                    │          ┌───────┐        │
           │ Row 6 │        3) update_hash for batch 1 with offset 5    │          │ Row 1 │    5   │
  Batch 3  │       │           - hashmap.insert(Row 2, idx 6)           │ Batch 1  │       │        │
           │ Row 7 │           - hashmap.insert(Row 1, idx 5)           │          │ Row 2 │    6   │
           └───────┘                                                    │          └───────┘        │
                                                                        │                           │
                                                                        └───────────────────────────┘
```

2. the **probe phase** where the tuples of the probe side are streamed
   through, checking for matches of the join keys in the hash table.

```text
                ┌────────────────┐          ┌────────────────┐
                │ ┌─────────┐    │          │ ┌─────────┐    │
                │ │  Hash   │    │          │ │  Hash   │    │
                │ │  Table  │    │          │ │  Table  │    │
                │ │(keys are│    │          │ │(keys are│    │
                │ │equi join│    │          │ │equi join│    │  Stage 2: batches from
 Stage 1: the   │ │columns) │    │          │ │columns) │    │    the probe side are
*entire* build  │ │         │    │          │ │         │    │  streamed through, and
 side is read   │ └─────────┘    │          │ └─────────┘    │   checked against the
into the hash   │      ▲         │          │          ▲     │   contents of the hash
    table       │       HashJoin │          │  HashJoin      │          table
                └──────┼─────────┘          └──────────┼─────┘
            ─ ─ ─ ─ ─ ─                                 ─ ─ ─ ─ ─ ─ ─
           │                                                         │

           │                                                         │
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘
          ...                                                       ...
    ┌────────────┐                                            ┌────────────┐
    │RecordBatch │                                            │RecordBatch │
    └────────────┘                                            └────────────┘

       build side                                                probe side
```

# Example "Optimal" Plans

The differences in the inputs means that for classic "Star Schema Query",
the optimal plan will be a **"Right Deep Tree"** . A Star Schema Query is
one where there is one large table and several smaller "dimension" tables,
joined on `Foreign Key = Primary Key` predicates.

A "Right Deep Tree" looks like this large table as the probe side on the
lowest join:

```text
            ┌───────────┐
            │ HashJoin  │
            │           │
            └───────────┘
                │   │
        ┌───────┘   └──────────┐
        ▼                      ▼
┌───────────────┐        ┌───────────┐
│ small table 1 │        │ HashJoin  │
│  "dimension"  │        │           │
└───────────────┘        └───┬───┬───┘
                  ┌──────────┘   └───────┐
                  │                      │
                  ▼                      ▼
          ┌───────────────┐        ┌───────────┐
          │ small table 2 │        │ HashJoin  │
          │  "dimension"  │        │           │
          └───────────────┘        └───┬───┬───┘
                              ┌────────┘   └────────┐
                              │                     │
                              ▼                     ▼
                      ┌───────────────┐     ┌───────────────┐
                      │ small table 3 │     │  large table  │
                      │  "dimension"  │     │    "fact"     │
                      └───────────────┘     └───────────────┘
```

# Clone / Shared State

Note this structure includes a [`OnceAsync`] that is used to coordinate the
loading of the left side with the processing in each output stream.
Therefore it can not be [`Clone`]

---

## HashJoinExecBuilder

`struct` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder`

Also reachable as `datafusion_physical_plan::joins::HashJoinExecBuilder`

```rust
struct HashJoinExecBuilder
```

**Implements**: `core::convert::From`

**Methods** (15)

```rust
fn build(self) -> Result<HashJoinExec>
fn build_exec(self) -> Result<Arc<dyn ExecutionPlan>>
fn new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: Vec<(PhysicalExprRef, PhysicalExprRef)>, join_type: JoinType) -> Self
fn recompute_properties(self) -> Self
fn reset_state(self) -> Self
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_filter(self, filter: Option<JoinFilter>) -> Self
fn with_new_children(self, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Self>
fn with_null_aware(self, null_aware: bool) -> Self
fn with_null_equality(self, null_equality: NullEquality) -> Self
fn with_on(self, on: Vec<(PhysicalExprRef, PhysicalExprRef)>) -> Self
fn with_partition_mode(self, mode: PartitionMode) -> Self
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
fn with_projection_ref(self, projection: Option<ProjectionRef>) -> Self
fn with_type(self, join_type: JoinType) -> Self
```

**via `core::convert::From`**

```rust
fn from(exec: &HashJoinExec) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExecBuilder.md).


Helps to build [`HashJoinExec`].

Builder can be created from an existing [`HashJoinExec`] using [`From::from`].
In this case, all its fields are inherited. If a field that affects the node's
properties is modified, they will be automatically recomputed during the build.

# Adding setters

When adding a new setter, it is necessary to ensure that the `preserve_properties`
flag is set to false if modifying the field requires a recomputation of the plan's
properties.

---
