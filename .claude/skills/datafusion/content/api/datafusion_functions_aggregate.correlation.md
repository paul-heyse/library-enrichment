# `datafusion_functions_aggregate::correlation`

Crate `datafusion-functions-aggregate` · 5 public items · structured records in [`model/datafusion_functions_aggregate.correlation.json`](../model/datafusion_functions_aggregate.correlation.json)

## corr

`function` · `datafusion_functions_aggregate::correlation::corr`

Also reachable as `datafusion_functions_aggregate::expr_fn::corr`

```rust
fn corr(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Correlation between two numeric values.

---

## corr_udaf

`function` · `datafusion_functions_aggregate::correlation::corr_udaf`

```rust
fn corr_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Correlation`]

---

## Correlation

`struct` · `datafusion_functions_aggregate::correlation::Correlation`

```rust
struct Correlation
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

---

## CorrelationAccumulator

`struct` · `datafusion_functions_aggregate::correlation::CorrelationAccumulator`

```rust
struct CorrelationAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new() -> Result<Self>
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

An accumulator to compute correlation

---

## CorrelationGroupsAccumulator

`struct` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator`

```rust
struct CorrelationGroupsAccumulator
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::groups_accumulator::GroupsAccumulator`**

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
fn size(&self) -> usize
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

---
