# `datafusion_physical_plan::sorts::partial_sort`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.sorts.partial_sort.json`](../model/datafusion_physical_plan.sorts.partial_sort.json)

## PartialSortExec

`struct` · `datafusion_physical_plan::sorts::partial_sort::PartialSortExec`

```rust
struct PartialSortExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn common_prefix_length(&self) -> usize
fn expr(&self) -> &LexOrdering
fn fetch(&self) -> Option<usize>
fn input(&self) -> &Arc<dyn ExecutionPlan>
fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>, common_prefix_length: usize) -> Self
fn preserve_partitioning(&self) -> bool
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_preserve_partitioning(self, preserve_partitioning: bool) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn benefits_from_input_partitioning(&self) -> Vec<bool>
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn input_distribution_requirements(&self) -> InputDistributionRequirements
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn required_input_distribution(&self) -> Vec<Distribution>
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Sort execution plan for inputs that are already partially sorted.

This operator takes input ordered by a prefix of the required ordering, and
produces output ordered by the required ordering, emitting rows sooner
(streaming) and using less peak memory than [`SortExec`] which must buffer
all rows before producing any output.

[`PartialSortExec`] relies on the property that rows with the same sort
prefix are contiguous, so it can sort one prefix group at a time, emitting
completed groups without reading (and buffering) the entire input.

For example, if the required output is `(a, b, c)`, but the input is only
ordered by `(a, b)`, `PartialSortExec` sorts only within each `(a, b)`
group to produce output ordered by `(a, b, c)`.

```text
input ordered by a, b              output ordered by a, b, c

+---+---+---+                      +---+---+---+
| a | b | c |                      | a | b | c |
+---+---+---+                      +---+---+---+
| 0 | 0 | 3 |  --  new group  -->  | 0 | 0 | 1 |
| 0 | 0 | 2 |                      | 0 | 0 | 2 |
| 0 | 0 | 1 |                      | 0 | 0 | 3 |
| 0 | 1 | 1 |  --  new group  -->  | 0 | 1 | 1 |
| 0 | 2 | 4 |  --  new group  -->  | 0 | 2 | 0 |
| 0 | 2 | 0 |                      | 0 | 2 | 4 |
| 1 | 0 | 5 |  --  new group  -->  | 1 | 0 | 5 |
+---+---+---+                      +---+---+---+
```

# Buffering and Emitting Rows

[`PartialSortExec`] buffers rows only until it can *prove* a prefix group
will never be seen again, then sorts and emits buffered rows. A group is
guaranteed to never be seen again once a row with a *different* prefix
value arrives. This relies on the input's existing ordering guarantees.

Using the example from above, rows accumulate in the in-memory buffer in
batches. As long as the `(a, b)` prefix keeps repeating, more rows are
buffered.

```text
           Buffer
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 0 | 0 | 3 |
       | 0 | 0 | 2 |
       | 0 | 0 | 1 |
       +---+---+---+
```

Once a batch arrives that contains a new `(a, b)` prefix, e.g. `(0, 2)`:
every buffered row for previous prefixes may be emitted:

```text
           Buffer
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 0 | 0 | 3 |
       | 0 | 0 | 2 |
       | 0 | 0 | 1 |
       | 0 | 1 | 1 |  <-- first row of new batch, new prefix
       | 0 | 2 | 4 |  <-- new prefix
       | 0 | 2 | 0 |
       | 1 | 0 | 5 |  <-- last row of new batch, new prefix
       +---+---+---+
```

Once known complete, the buffered rows are sorted by the full `(a, b, c)`
ordering and emitted as a [`RecordBatch`]; Any rows from the most recently
seen prefix remain buffered (as more rows with the same prefix may arrive in
future batches.

```text
         Emitted      <-- fully sorted on (a, b, c)
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 0 | 0 | 1 |   <-- completed group
       | 0 | 0 | 2 |
       | 0 | 0 | 3 |
       | 0 | 2 | 0 |   <-- completed group
       | 0 | 2 | 4 |
       | 0 | 1 | 1 |   <-- completed group
       +---+---+---+

           Buffer
       +---+---+---+
       | a | b | c |
       +---+---+---+
       | 1 | 0 | 5 |   <-- (possibly) in progress group
       +---+---+---+
```

[`SortExec`]: crate::sorts::sort::SortExec

---
