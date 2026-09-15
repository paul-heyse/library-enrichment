# `datafusion_functions_aggregate::sum`

Crate `datafusion-functions-aggregate` · 5 public items · structured records in [`model/datafusion_functions_aggregate.sum.json`](../model/datafusion_functions_aggregate.sum.json)

## sum

`function` · `datafusion_functions_aggregate::sum::sum`

Also reachable as `datafusion_functions_aggregate::expr_fn::sum`

```rust
fn sum(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns the sum of a group of values.

---

## sum_distinct

`function` · `datafusion_functions_aggregate::sum::sum_distinct`

Also reachable as `datafusion_functions_aggregate::expr_fn::sum_distinct`

```rust
fn sum_distinct(expr: datafusion_expr::Expr) -> datafusion_expr::Expr
```

---

## sum_udaf

`function` · `datafusion_functions_aggregate::sum::sum_udaf`

```rust
fn sum_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Sum`]

---

## SlidingDistinctSumAccumulator

`struct` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator`

```rust
struct SlidingDistinctSumAccumulator
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

A sliding‐window accumulator for `SUM(DISTINCT)` over Int64 columns.
Maintains a running sum so that `evaluate()` is O(1).

---

## Sum

`struct` · `datafusion_functions_aggregate::sum::Sum`

```rust
struct Sum
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn set_monotonicity(&self, data_type: &DataType) -> SetMonotonicity
fn signature(&self) -> &Signature
fn simplify_expr_op_literal(&self, agg_function: &AggregateFunction, arg: &Expr, op: Operator, lit: &Expr, _arg_is_left: bool) -> Result<Option<Expr>>
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

---
