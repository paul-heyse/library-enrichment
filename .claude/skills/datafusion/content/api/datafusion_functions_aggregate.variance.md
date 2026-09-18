# `datafusion_functions_aggregate::variance`

Crate `datafusion-functions-aggregate` · 9 public items · structured records in [`model/datafusion_functions_aggregate.variance.json`](../model/datafusion_functions_aggregate.variance.json)

## var_pop

`function` · `datafusion_functions_aggregate::variance::var_pop`

Also reachable as `datafusion_functions_aggregate::expr_fn::var_pop`

```rust
fn var_pop(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.var_pop.md).


Computes the population variance.

---

## var_pop_udaf

`function` · `datafusion_functions_aggregate::variance::var_pop_udaf`

```rust
fn var_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.var_pop_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`VariancePopulation`]

---

## var_samp_udaf

`function` · `datafusion_functions_aggregate::variance::var_samp_udaf`

```rust
fn var_samp_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.var_samp_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`VarianceSample`]

---

## var_sample

`function` · `datafusion_functions_aggregate::variance::var_sample`

Also reachable as `datafusion_functions_aggregate::expr_fn::var_sample`

```rust
fn var_sample(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.var_sample.md).


Computes the sample variance.

---

## DistinctVarianceAccumulator

`struct` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator`

```rust
struct DistinctVarianceAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(stat_type: StatsType) -> Self
```

**via `datafusion_expr_common::accumulator::Accumulator`**

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn size(&self) -> usize
fn state(&mut self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.DistinctVarianceAccumulator.md).


---

## VarianceAccumulator

`struct` · `datafusion_functions_aggregate::variance::VarianceAccumulator`

```rust
struct VarianceAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (4)

```rust
fn get_count(&self) -> u64
fn get_m2(&self) -> f64
fn get_mean(&self) -> f64
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.VarianceAccumulator.md).


An accumulator to compute variance
The algorithm used is an online implementation and numerically stable. It is based on this paper:
Welford, B. P. (1962). "Note on a method for calculating corrected sums of squares and products".
Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

The algorithm has been analyzed here:
Ling, Robert F. (1974). "Comparison of Several Algorithms for Computing Sample Means and Variances".
Journal of the American Statistical Association. 69 (348): 859–866. doi:10.2307/2286154. JSTOR 2286154.

---

## VarianceGroupsAccumulator

`struct` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator`

```rust
struct VarianceGroupsAccumulator
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Derives**: Debug

**Methods** (2)

```rust
fn new(s_type: StatsType) -> Self
fn variance(&mut self, emit_to: datafusion_expr::EmitTo) -> (Vec<f64>, NullBuffer)
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.VarianceGroupsAccumulator.md).


---

## VariancePopulation

`struct` · `datafusion_functions_aggregate::variance::VariancePopulation`

```rust
struct VariancePopulation
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.VariancePopulation.md).


---

## VarianceSample

`struct` · `datafusion_functions_aggregate::variance::VarianceSample`

```rust
struct VarianceSample
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.variance.VarianceSample.md).


---
