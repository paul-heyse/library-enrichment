# `datafusion_functions_aggregate::first_last`

Crate `datafusion-functions-aggregate` · 9 public items · structured records in [`model/datafusion_functions_aggregate.first_last.json`](../model/datafusion_functions_aggregate.first_last.json)

## first_value

`function` · `datafusion_functions_aggregate::first_last::first_value`

Also reachable as `datafusion_functions_aggregate::expr_fn::first_value`

```rust
fn first_value(expression: datafusion_expr::Expr, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr
```

Returns the first value in a group of values.

---

## first_value_udaf

`function` · `datafusion_functions_aggregate::first_last::first_value_udaf`

```rust
fn first_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`FirstValue`]

---

## last_value

`function` · `datafusion_functions_aggregate::first_last::last_value`

Also reachable as `datafusion_functions_aggregate::expr_fn::last_value`

```rust
fn last_value(expression: datafusion_expr::Expr, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr
```

Returns the last value in a group of values.

---

## last_value_udaf

`function` · `datafusion_functions_aggregate::first_last::last_value_udaf`

```rust
fn last_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`LastValue`]

---

## FirstValue

`struct` · `datafusion_functions_aggregate::first_last::FirstValue`

```rust
struct FirstValue
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
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_null_handling_clause(&self) -> bool
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

---

## FirstValueAccumulator

`struct` · `datafusion_functions_aggregate::first_last::FirstValueAccumulator`

```rust
struct FirstValueAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, ordering_dtypes: &[DataType], ordering_req: LexOrdering, is_input_pre_ordered: bool, ignore_nulls: bool) -> Result<Self>
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

## LastValue

`struct` · `datafusion_functions_aggregate::first_last::LastValue`

```rust
struct LastValue
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
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_null_handling_clause(&self) -> bool
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

---

## TrivialFirstValueAccumulator

`struct` · `datafusion_functions_aggregate::first_last::TrivialFirstValueAccumulator`

```rust
struct TrivialFirstValueAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, ignore_nulls: bool) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

This accumulator is used when there is no ordering specified for the
`FIRST_VALUE` aggregation. It simply returns the first value it sees
according to the pre-existing ordering of the input data, and provides
a fast path for this case without needing to maintain any ordering state.

---

## TrivialLastValueAccumulator

`struct` · `datafusion_functions_aggregate::first_last::TrivialLastValueAccumulator`

```rust
struct TrivialLastValueAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(data_type: &DataType, ignore_nulls: bool) -> Result<Self>
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

This accumulator is used when there is no ordering specified for the
`LAST_VALUE` aggregation. It simply updates the last value it sees
according to the pre-existing ordering of the input data, and provides
a fast path for this case without needing to maintain any ordering state.

---
