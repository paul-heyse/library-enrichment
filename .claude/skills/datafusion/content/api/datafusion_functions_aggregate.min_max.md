# `datafusion_functions_aggregate::min_max`

Crate `datafusion-functions-aggregate` · 8 public items · structured records in [`model/datafusion_functions_aggregate.min_max.json`](../model/datafusion_functions_aggregate.min_max.json)

## max

`function` · `datafusion_functions_aggregate::min_max::max`

Also reachable as `datafusion_functions_aggregate::expr_fn::max`

```rust
fn max(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.max.md).


Returns the maximum of a group of values.

---

## max_udaf

`function` · `datafusion_functions_aggregate::min_max::max_udaf`

```rust
fn max_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.max_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Max`]

---

## min

`function` · `datafusion_functions_aggregate::min_max::min`

Also reachable as `datafusion_functions_aggregate::expr_fn::min`

```rust
fn min(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.min.md).


Returns the minimum of a group of values.

---

## min_udaf

`function` · `datafusion_functions_aggregate::min_max::min_udaf`

```rust
fn min_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.min_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Min`]

---

## Max

`struct` · `datafusion_functions_aggregate::min_max::Max`

```rust
struct Max
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
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn is_descending(&self) -> Option<bool>
fn name(&self) -> &str
fn order_sensitivity(&self) -> datafusion_expr::utils::AggregateOrderSensitivity
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
fn signature(&self) -> &Signature
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.Max.md).


---

## Min

`struct` · `datafusion_functions_aggregate::min_max::Min`

```rust
struct Min
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
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn is_descending(&self) -> Option<bool>
fn name(&self) -> &str
fn order_sensitivity(&self) -> datafusion_expr::utils::AggregateOrderSensitivity
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
fn signature(&self) -> &Signature
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.Min.md).


---

## SlidingMaxAccumulator

`struct` · `datafusion_functions_aggregate::min_max::SlidingMaxAccumulator`

```rust
struct SlidingMaxAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(datatype: &DataType) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.SlidingMaxAccumulator.md).


---

## SlidingMinAccumulator

`struct` · `datafusion_functions_aggregate::min_max::SlidingMinAccumulator`

```rust
struct SlidingMinAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(datatype: &DataType) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.min_max.SlidingMinAccumulator.md).


---
