# `datafusion_functions_aggregate::count`

Crate `datafusion-functions-aggregate` · 7 public items · structured records in [`model/datafusion_functions_aggregate.count.json`](../model/datafusion_functions_aggregate.count.json)

## count

`function` · `datafusion_functions_aggregate::count::count`

Also reachable as `datafusion_functions_aggregate::expr_fn::count`

```rust
fn count(expr: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Count the number of non-null values in the column

---

## count_all

`function` · `datafusion_functions_aggregate::count::count_all`

```rust
fn count_all() -> datafusion_expr::Expr
```

Creates aggregation to count all rows.

In SQL this is `SELECT COUNT(*) ... `

The expression is equivalent to `COUNT(*)`, `COUNT()`, `COUNT(1)`, and is
aliased to a column named `"count(*)"` for backward compatibility.

Example
```
# use datafusion_functions_aggregate::count::count_all;
# use datafusion_expr::col;
// create `count(*)` expression
let expr = count_all();
assert_eq!(expr.schema_name().to_string(), "count(*)");
// if you need to refer to this column, use the `schema_name` function
let expr = col(expr.schema_name().to_string());
```

---

## count_all_window

`function` · `datafusion_functions_aggregate::count::count_all_window`

```rust
fn count_all_window() -> datafusion_expr::Expr
```

Creates window aggregation to count all rows.

In SQL this is `SELECT COUNT(*) OVER (..) ... `

The expression is equivalent to `COUNT(*)`, `COUNT()`, `COUNT(1)`

Example
```
# use datafusion_functions_aggregate::count::count_all_window;
# use datafusion_expr::col;
// create `count(*)` OVER ... window function expression
let expr = count_all_window();
assert_eq!(
    expr.schema_name().to_string(),
    "count(Int64(1)) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
);
// if you need to refer to this column, use the `schema_name` function
let expr = col(expr.schema_name().to_string());
```

---

## count_distinct

`function` · `datafusion_functions_aggregate::count::count_distinct`

Also reachable as `datafusion_functions_aggregate::expr_fn::count_distinct`

```rust
fn count_distinct(expr: datafusion_expr::Expr) -> datafusion_expr::Expr
```

---

## count_udaf

`function` · `datafusion_functions_aggregate::count::count_udaf`

```rust
fn count_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Count`]

---

## Count

`struct` · `datafusion_functions_aggregate::count::Count`

```rust
struct Count
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

---

## SlidingDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate::count::SlidingDistinctCountAccumulator`

```rust
struct SlidingDistinctCountAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn supports_retract_batch(&self) -> bool
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

---
