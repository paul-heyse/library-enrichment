# `datafusion_physical_plan::memory`

Crate `datafusion-physical-plan` · 4 public items · structured records in [`model/datafusion_physical_plan.memory.json`](../model/datafusion_physical_plan.memory.json)

## LazyMemoryExec

`struct` · `datafusion_physical_plan::memory::LazyMemoryExec`

```rust
struct LazyMemoryExec
```

**Implements**: `datafusion_physical_plan::display::DisplayAs`, `datafusion_physical_plan::execution_plan::ExecutionPlan`

**Derives**: Debug

**Methods** (5)

```rust
fn add_ordering(&mut self, ordering: impl IntoIterator<Item = PhysicalSortExpr>)
fn generators(&self) -> &Vec<Arc<RwLock<dyn LazyBatchGenerator>>>
fn try_new(schema: SchemaRef, generators: Vec<Arc<RwLock<dyn LazyBatchGenerator>>>) -> Result<Self>
fn try_set_partitioning(&mut self, partitioning: Partitioning) -> Result<()>
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
```

**via `datafusion_physical_plan::display::DisplayAs`**

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::execution_plan::ExecutionPlan`**

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
fn metrics(&self) -> Option<MetricsSet>
fn name(&self) -> &'static str
fn properties(&self) -> &Arc<PlanProperties>
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.memory.LazyMemoryExec.md).


Execution plan for lazy in-memory batches of data

This plan generates output batches lazily, it doesn't have to buffer all batches
in memory up front (compared to `MemorySourceConfig`), thus consuming constant memory.

---

## LazyMemoryStream

`struct` · `datafusion_physical_plan::memory::LazyMemoryStream`

```rust
struct LazyMemoryStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(std::pin::Pin<&mut self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.memory.LazyMemoryStream.md).


Stream that generates record batches on demand

---

## MemoryStream

`struct` · `datafusion_physical_plan::memory::MemoryStream`

```rust
struct MemoryStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Methods** (3)

```rust
fn try_new(data: Vec<RecordBatch>, schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_reservation(self, reservation: MemoryReservation) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(std::pin::Pin<&mut self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.memory.MemoryStream.md).


Iterator over batches

---

## LazyBatchGenerator

`trait` · `datafusion_physical_plan::memory::LazyBatchGenerator`

```rust
trait LazyBatchGenerator: Send + Sync + fmt::Debug + fmt::Display
```

**Implementors** (2)

- `datafusion_functions_table::generate_series::Empty`
- `datafusion_functions_table::generate_series::GenericSeriesState`

**Methods** (4)

```rust
fn as_any(&self) -> &dyn Any
fn boundedness(&self) -> Boundedness
fn generate_next_batch(&mut self) -> Result<Option<RecordBatch>>
fn reset_state(&self) -> Arc<RwLock<dyn LazyBatchGenerator>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.memory.LazyBatchGenerator.md).


---
