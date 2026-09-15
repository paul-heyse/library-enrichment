# `datafusion_physical_plan::test`

Crate `datafusion-physical-plan` · 13 public items · structured records in [`model/datafusion_physical_plan.test.json`](../model/datafusion_physical_plan.test.json)

## aggr_test_schema

`function` · `datafusion_physical_plan::test::aggr_test_schema`

```rust
fn aggr_test_schema() -> arrow_schema::SchemaRef
```

Get the schema for the aggregate_test_* csv files

---

## assert_is_pending

`function` · `datafusion_physical_plan::test::assert_is_pending`

```rust
fn assert_is_pending<'a, T>(fut: &mut std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>)
```

Asserts that given future is pending.

---

## build_table_i32

`function` · `datafusion_physical_plan::test::build_table_i32`

```rust
fn build_table_i32(a: (&str, &Vec<i32>), b: (&str, &Vec<i32>), c: (&str, &Vec<i32>)) -> arrow::array::RecordBatch
```

Returns record batch with 3 columns of i32 in memory

---

## build_table_i32_two_cols

`function` · `datafusion_physical_plan::test::build_table_i32_two_cols`

```rust
fn build_table_i32_two_cols(a: (&str, &Vec<i32>), b: (&str, &Vec<i32>)) -> arrow::array::RecordBatch
```

Returns record batch with 2 columns of i32 in memory

---

## build_table_scan_i32

`function` · `datafusion_physical_plan::test::build_table_scan_i32`

```rust
fn build_table_scan_i32(a: (&str, &Vec<i32>), b: (&str, &Vec<i32>), c: (&str, &Vec<i32>)) -> std::sync::Arc<dyn ExecutionPlan>
```

Returns memory table scan wrapped around record batch with 3 columns of i32

---

## make_partition

`function` · `datafusion_physical_plan::test::make_partition`

```rust
fn make_partition(sz: i32) -> arrow::array::RecordBatch
```

Return a RecordBatch with a single Int32 array with values (0..sz) in a field named "i"

---

## make_partition_utf8

`function` · `datafusion_physical_plan::test::make_partition_utf8`

```rust
fn make_partition_utf8(sz: i32) -> arrow::array::RecordBatch
```

---

## mem_exec

`function` · `datafusion_physical_plan::test::mem_exec`

```rust
fn mem_exec(partitions: usize) -> TestMemoryExec
```

Returns a `DataSourceExec` that scans `partitions` of 100 batches each

---

## mem_exec_utf8

`function` · `datafusion_physical_plan::test::mem_exec_utf8`

```rust
fn mem_exec_utf8(partitions: usize) -> TestMemoryExec
```

---

## scan_partitioned

`function` · `datafusion_physical_plan::test::scan_partitioned`

```rust
fn scan_partitioned(partitions: usize) -> std::sync::Arc<dyn ExecutionPlan>
```

Returns a `DataSourceExec` that scans `partitions` of 100 batches each

---

## scan_partitioned_utf8

`function` · `datafusion_physical_plan::test::scan_partitioned_utf8`

```rust
fn scan_partitioned_utf8(partitions: usize) -> std::sync::Arc<dyn ExecutionPlan>
```

---

## TestMemoryExec

`struct` · `datafusion_physical_plan::test::TestMemoryExec`

```rust
struct TestMemoryExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn original_schema(&self) -> SchemaRef
fn partitions(&self) -> &[Vec<RecordBatch>]
fn projection(&self) -> &Option<Vec<usize>>
fn sort_information(&self) -> &[LexOrdering]
fn try_new(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
fn try_new_exec(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Arc<TestMemoryExec>>
fn try_with_sort_information(self, sort_information: Vec<LexOrdering>) -> Result<Self>
fn update_cache(source: &Arc<TestMemoryExec>) -> TestMemoryExec
fn with_limit(self, limit: Option<usize>) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn fetch(&self) -> Option<usize>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn repartitioned(&self, _target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>>
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

`TestMemoryExec` is a mock equivalent to [`MemorySourceConfig`] with [`ExecutionPlan`] implemented for testing.
i.e. It has some but not all the functionality of [`MemorySourceConfig`].
This implements an in-memory DataSource rather than explicitly implementing a trait.
It is implemented in this manner to keep relevant unit tests in place
while avoiding circular dependencies between `datafusion-physical-plan` and `datafusion-datasource`.

[`MemorySourceConfig`]: https://github.com/apache/datafusion/tree/main/datafusion/datasource/src/memory.rs

---

## TestPartitionStream

`struct` · `datafusion_physical_plan::test::TestPartitionStream`

```rust
struct TestPartitionStream
```

**Fields**: `schema`, `batches`

**Implements**: `datafusion_physical_plan::streaming::PartitionStream`

**Derives**: Debug

**Methods** (1)

```rust
fn new_with_batches(batches: Vec<RecordBatch>) -> Self
```

**via `datafusion_physical_plan::streaming::PartitionStream`**

```rust
fn execute(&self, _ctx: Arc<TaskContext>) -> SendableRecordBatchStream
fn schema(&self) -> &SchemaRef
```

---
