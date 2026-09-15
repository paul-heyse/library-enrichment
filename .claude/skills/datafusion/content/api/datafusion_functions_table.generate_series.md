# `datafusion_functions_table::generate_series`

Crate `datafusion-functions-table` · 8 public items · structured records in [`model/datafusion_functions_table.generate_series.json`](../model/datafusion_functions_table.generate_series.json)

## GenSeriesArgs

`enum` · `datafusion_functions_table::generate_series::GenSeriesArgs`

```rust
enum GenSeriesArgs
```

**Variants**: `ContainsNull`, `Int64Args`, `TimestampArgs`, `DateArgs`

**Derives**: Clone, Debug

Indicates the arguments used for generating a series.

---

## Empty

`struct` · `datafusion_functions_table::generate_series::Empty`

```rust
struct Empty
```

**Implements**: `core::fmt::Display`, `datafusion_physical_plan::memory::LazyBatchGenerator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn name(&self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::memory::LazyBatchGenerator`**

```rust
fn as_any(&self) -> &dyn Any
fn generate_next_batch(&mut self) -> Result<Option<RecordBatch>>
fn reset_state(&self) -> Arc<RwLock<dyn LazyBatchGenerator>>
```

Empty generator that produces no rows - used when series arguments contain null values

---

## GenerateSeriesFunc

`struct` · `datafusion_functions_table::generate_series::GenerateSeriesFunc`

```rust
struct GenerateSeriesFunc
```

**Implements**: `datafusion_session::table::TableFunctionImpl`

**Derives**: Debug

**via `datafusion_session::table::TableFunctionImpl`**

```rust
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

---

## GenerateSeriesTable

`struct` · `datafusion_functions_table::generate_series::GenerateSeriesTable`

```rust
struct GenerateSeriesTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn as_generator(&self, batch_size: usize) -> Result<Arc<RwLock<dyn LazyBatchGenerator>>>
fn new(schema: SchemaRef, args: GenSeriesArgs) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
```

Table that generates a series of integers/timestamps from `start`(inclusive) to `end`, incrementing by step

---

## GenericSeriesState

`struct` · `datafusion_functions_table::generate_series::GenericSeriesState`

```rust
struct GenericSeriesState<T: SeriesValue>
```

**Implements**: `core::fmt::Display`, `datafusion_physical_plan::memory::LazyBatchGenerator`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn batch_size(&self) -> usize
fn current(&self) -> &T
fn end(&self) -> &T
fn include_end(&self) -> bool
fn name(&self) -> &'static str
fn start(&self) -> &T
fn step(&self) -> &T::StepType
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_physical_plan::memory::LazyBatchGenerator`**

```rust
fn as_any(&self) -> &dyn Any
fn generate_next_batch(&mut self) -> Result<Option<RecordBatch>>
fn reset_state(&self) -> Arc<RwLock<dyn LazyBatchGenerator>>
```

---

## RangeFunc

`struct` · `datafusion_functions_table::generate_series::RangeFunc`

```rust
struct RangeFunc
```

**Implements**: `datafusion_session::table::TableFunctionImpl`

**Derives**: Debug

**via `datafusion_session::table::TableFunctionImpl`**

```rust
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

---

## TimestampValue

`struct` · `datafusion_functions_table::generate_series::TimestampValue`

```rust
struct TimestampValue
```

**Implements**: `datafusion_functions_table::generate_series::SeriesValue`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn tz_str(&self) -> Option<&Arc<str>>
fn value(&self) -> i64
```

**via `datafusion_functions_table::generate_series::SeriesValue`**

```rust
fn advance(&mut self, step: &Self::StepType) -> Result<()>
fn advance_with_end(&mut self, end: &mut Self, step: &Self::StepType) -> Result<()>
fn create_array(&self, values: Vec<Self::ValueType>) -> Result<ArrayRef>
fn display_value(&self) -> String
fn should_stop(&self, end: Self, step: &Self::StepType, include_end: bool) -> bool
fn to_value_type(&self) -> Self::ValueType
```

---

## SeriesValue

`trait` · `datafusion_functions_table::generate_series::SeriesValue`

```rust
trait SeriesValue: fmt::Debug + Clone + Send + Sync + 'static
```

**Implementors** (1)

- `datafusion_functions_table::generate_series::TimestampValue`

**Methods** (6)

```rust
fn advance(&mut self, step: &Self::StepType) -> Result<()>
fn advance_with_end(&mut self, _end: &mut Self, step: &Self::StepType) -> Result<()>
fn create_array(&self, values: Vec<Self::ValueType>) -> Result<ArrayRef>
fn display_value(&self) -> String
fn should_stop(&self, end: Self, step: &Self::StepType, include_end: bool) -> bool
fn to_value_type(&self) -> Self::ValueType
```

Trait for values that can be generated in a series

---
