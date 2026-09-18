# `datafusion_functions_aggregate::array_agg`

Crate `datafusion-functions-aggregate` · 5 public items · structured records in [`model/datafusion_functions_aggregate.array_agg.json`](../model/datafusion_functions_aggregate.array_agg.json)

## array_agg

`function` · `datafusion_functions_aggregate::array_agg::array_agg`

Also reachable as `datafusion_functions_aggregate::expr_fn::array_agg`

```rust
fn array_agg(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.array_agg.array_agg.md).


input values, including nulls, concatenated into an array

---

## array_agg_udaf

`function` · `datafusion_functions_aggregate::array_agg::array_agg_udaf`

```rust
fn array_agg_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.array_agg.array_agg_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ArrayAgg`]

---

## ArrayAgg

`struct` · `datafusion_functions_aggregate::array_agg::ArrayAgg`

```rust
struct ArrayAgg
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_null_handling_clause(&self) -> bool
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.array_agg.ArrayAgg.md).


ARRAY_AGG aggregate expression

---

## ArrayAggAccumulator

`struct` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator`

```rust
struct ArrayAggAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(datatype: &DataType, ignore_nulls: bool) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.array_agg.ArrayAggAccumulator.md).


---

## DistinctArrayAggAccumulator

`struct` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator`

```rust
struct DistinctArrayAggAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(datatype: &DataType, sort_options: Option<SortOptions>, ignore_nulls: bool) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.array_agg.DistinctArrayAggAccumulator.md).


---
