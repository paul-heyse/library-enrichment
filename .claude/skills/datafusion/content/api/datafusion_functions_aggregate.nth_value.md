# `datafusion_functions_aggregate::nth_value`

Crate `datafusion-functions-aggregate` · 5 public items · structured records in [`model/datafusion_functions_aggregate.nth_value.json`](../model/datafusion_functions_aggregate.nth_value.json)

## nth_value

`function` · `datafusion_functions_aggregate::nth_value::nth_value`

Also reachable as `datafusion_functions_aggregate::expr_fn::nth_value`

```rust
fn nth_value(expr: datafusion_expr::Expr, n: i64, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr
```

Returns the nth value in a group of values.

---

## nth_value_udaf

`function` · `datafusion_functions_aggregate::nth_value::nth_value_udaf`

```rust
fn nth_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`NthValueAgg`]

---

## NthValueAccumulator

`struct` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator`

```rust
struct NthValueAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(n: i64, datatype: &DataType, ordering_dtypes: &[DataType], ordering_req: LexOrdering) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

---

## NthValueAgg

`struct` · `datafusion_functions_aggregate::nth_value::NthValueAgg`

```rust
struct NthValueAgg
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
fn documentation(&self) -> Option<&Documentation>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Expression for a `NTH_VALUE(..., ... ORDER BY ...)` aggregation. In a multi
partition setting, partial aggregations are computed for every partition,
and then their results are merged.

---

## TrivialNthValueAccumulator

`struct` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator`

```rust
struct TrivialNthValueAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(n: i64, datatype: &DataType) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

---
