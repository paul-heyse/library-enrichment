# `datafusion_functions_aggregate::stddev`

Crate `datafusion-functions-aggregate` · 8 public items · structured records in [`model/datafusion_functions_aggregate.stddev.json`](../model/datafusion_functions_aggregate.stddev.json)

## stddev

`function` · `datafusion_functions_aggregate::stddev::stddev`

Also reachable as `datafusion_functions_aggregate::expr_fn::stddev`

```rust
fn stddev(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.stddev.md).


Compute the standard deviation of a set of numbers

---

## stddev_pop

`function` · `datafusion_functions_aggregate::stddev::stddev_pop`

Also reachable as `datafusion_functions_aggregate::expr_fn::stddev_pop`

```rust
fn stddev_pop(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.stddev_pop.md).


Compute the population standard deviation of a set of numbers

---

## stddev_pop_udaf

`function` · `datafusion_functions_aggregate::stddev::stddev_pop_udaf`

```rust
fn stddev_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.stddev_pop_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`StddevPop`]

---

## stddev_udaf

`function` · `datafusion_functions_aggregate::stddev::stddev_udaf`

```rust
fn stddev_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.stddev_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Stddev`]

---

## Stddev

`struct` · `datafusion_functions_aggregate::stddev::Stddev`

```rust
struct Stddev
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
fn aliases(&self) -> &[String]
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, acc_args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.Stddev.md).


STDDEV and STDDEV_SAMP (standard deviation) aggregate expression

---

## StddevAccumulator

`struct` · `datafusion_functions_aggregate::stddev::StddevAccumulator`

```rust
struct StddevAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (2)

```rust
fn get_m2(&self) -> f64
fn try_new(s_type: StatsType) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.StddevAccumulator.md).


An accumulator to compute the average

---

## StddevGroupsAccumulator

`struct` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator`

```rust
struct StddevGroupsAccumulator
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(s_type: StatsType) -> Self
```

**via `datafusion_expr_common::groups_accumulator::GroupsAccumulator`**

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
fn evaluate(&mut self, emit_to: datafusion_expr::EmitTo) -> Result<ArrayRef>
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
fn size(&self) -> usize
fn state(&mut self, emit_to: datafusion_expr::EmitTo) -> Result<Vec<ArrayRef>>
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.StddevGroupsAccumulator.md).


---

## StddevPop

`struct` · `datafusion_functions_aggregate::stddev::StddevPop`

```rust
struct StddevPop
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
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, acc_args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.stddev.StddevPop.md).


STDDEV_POP population aggregate expression

---
