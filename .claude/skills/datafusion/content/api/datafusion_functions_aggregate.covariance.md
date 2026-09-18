# `datafusion_functions_aggregate::covariance`

Crate `datafusion-functions-aggregate` · 7 public items · structured records in [`model/datafusion_functions_aggregate.covariance.json`](../model/datafusion_functions_aggregate.covariance.json)

## covar_pop

`function` · `datafusion_functions_aggregate::covariance::covar_pop`

Also reachable as `datafusion_functions_aggregate::expr_fn::covar_pop`

```rust
fn covar_pop(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.covar_pop.md).


Computes the population covariance.

---

## covar_pop_udaf

`function` · `datafusion_functions_aggregate::covariance::covar_pop_udaf`

```rust
fn covar_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.covar_pop_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`CovariancePopulation`]

---

## covar_samp

`function` · `datafusion_functions_aggregate::covariance::covar_samp`

Also reachable as `datafusion_functions_aggregate::expr_fn::covar_samp`

```rust
fn covar_samp(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.covar_samp.md).


Computes the sample covariance.

---

## covar_samp_udaf

`function` · `datafusion_functions_aggregate::covariance::covar_samp_udaf`

```rust
fn covar_samp_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.covar_samp_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`CovarianceSample`]

---

## CovarianceAccumulator

`struct` · `datafusion_functions_aggregate::covariance::CovarianceAccumulator`

```rust
struct CovarianceAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (5)

```rust
fn get_algo_const(&self) -> f64
fn get_count(&self) -> u64
fn get_mean1(&self) -> f64
fn get_mean2(&self) -> f64
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.CovarianceAccumulator.md).


An accumulator to compute covariance
The algorithm used is an online implementation and numerically stable. It is derived from the following paper
for calculating variance:
Welford, B. P. (1962). "Note on a method for calculating corrected sums of squares and products".
Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

The algorithm has been analyzed here:
Ling, Robert F. (1974). "Comparison of Several Algorithms for Computing Sample Means and Variances".
Journal of the American Statistical Association. 69 (348): 859–866. doi:10.2307/2286154. JSTOR 2286154.

Though it is not covered in the original paper but is based on the same idea, as a result the algorithm is online,
parallelize and numerically stable.

---

## CovariancePopulation

`struct` · `datafusion_functions_aggregate::covariance::CovariancePopulation`

```rust
struct CovariancePopulation
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
fn documentation(&self) -> Option<&Documentation>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.CovariancePopulation.md).


---

## CovarianceSample

`struct` · `datafusion_functions_aggregate::covariance::CovarianceSample`

```rust
struct CovarianceSample
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
fn aliases(&self) -> &[String]
fn documentation(&self) -> Option<&Documentation>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.covariance.CovarianceSample.md).


---
