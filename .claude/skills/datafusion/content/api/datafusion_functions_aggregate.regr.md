# `datafusion_functions_aggregate::regr`

Crate `datafusion-functions-aggregate` · 21 public items · structured records in [`model/datafusion_functions_aggregate.regr.json`](../model/datafusion_functions_aggregate.regr.json)

## RegrType

`enum` · `datafusion_functions_aggregate::regr::RegrType`

```rust
enum RegrType
```

**Variants**: `Slope`, `Intercept`, `Count`, `R2`, `AvgX`, `AvgY`, `SXX`, `SYY`, `SXY`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.RegrType.md).


---

## regr_avgx

`function` · `datafusion_functions_aggregate::regr::regr_avgx`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_avgx`

```rust
fn regr_avgx(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_avgx.md).


Compute a linear regression of type [RegrType::AvgX]

---

## regr_avgx_udaf

`function` · `datafusion_functions_aggregate::regr::regr_avgx_udaf`

```rust
fn regr_avgx_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_avgx_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_avgx`]

---

## regr_avgy

`function` · `datafusion_functions_aggregate::regr::regr_avgy`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_avgy`

```rust
fn regr_avgy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_avgy.md).


Compute a linear regression of type [RegrType::AvgY]

---

## regr_avgy_udaf

`function` · `datafusion_functions_aggregate::regr::regr_avgy_udaf`

```rust
fn regr_avgy_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_avgy_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_avgy`]

---

## regr_count

`function` · `datafusion_functions_aggregate::regr::regr_count`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_count`

```rust
fn regr_count(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_count.md).


Compute a linear regression of type [RegrType::Count]

---

## regr_count_udaf

`function` · `datafusion_functions_aggregate::regr::regr_count_udaf`

```rust
fn regr_count_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_count_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_count`]

---

## regr_intercept

`function` · `datafusion_functions_aggregate::regr::regr_intercept`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_intercept`

```rust
fn regr_intercept(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_intercept.md).


Compute a linear regression of type [RegrType::Intercept]

---

## regr_intercept_udaf

`function` · `datafusion_functions_aggregate::regr::regr_intercept_udaf`

```rust
fn regr_intercept_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_intercept_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_intercept`]

---

## regr_r2

`function` · `datafusion_functions_aggregate::regr::regr_r2`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_r2`

```rust
fn regr_r2(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_r2.md).


Compute a linear regression of type [RegrType::R2]

---

## regr_r2_udaf

`function` · `datafusion_functions_aggregate::regr::regr_r2_udaf`

```rust
fn regr_r2_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_r2_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_r2`]

---

## regr_slope

`function` · `datafusion_functions_aggregate::regr::regr_slope`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_slope`

```rust
fn regr_slope(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_slope.md).


Compute a linear regression of type [RegrType::Slope]

---

## regr_slope_udaf

`function` · `datafusion_functions_aggregate::regr::regr_slope_udaf`

```rust
fn regr_slope_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_slope_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_slope`]

---

## regr_sxx

`function` · `datafusion_functions_aggregate::regr::regr_sxx`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_sxx`

```rust
fn regr_sxx(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_sxx.md).


Compute a linear regression of type [RegrType::SXX]

---

## regr_sxx_udaf

`function` · `datafusion_functions_aggregate::regr::regr_sxx_udaf`

```rust
fn regr_sxx_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_sxx_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_sxx`]

---

## regr_sxy

`function` · `datafusion_functions_aggregate::regr::regr_sxy`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_sxy`

```rust
fn regr_sxy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_sxy.md).


Compute a linear regression of type [RegrType::SXY]

---

## regr_sxy_udaf

`function` · `datafusion_functions_aggregate::regr::regr_sxy_udaf`

```rust
fn regr_sxy_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_sxy_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_sxy`]

---

## regr_syy

`function` · `datafusion_functions_aggregate::regr::regr_syy`

Also reachable as `datafusion_functions_aggregate::expr_fn::regr_syy`

```rust
fn regr_syy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_syy.md).


Compute a linear regression of type [RegrType::SYY]

---

## regr_syy_udaf

`function` · `datafusion_functions_aggregate::regr::regr_syy_udaf`

```rust
fn regr_syy_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.regr_syy_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_syy`]

---

## Regr

`struct` · `datafusion_functions_aggregate::regr::Regr`

```rust
struct Regr
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(regr_type: RegrType, func_name: &'static str) -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
fn documentation(&self) -> Option<&Documentation>
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.Regr.md).


---

## RegrAccumulator

`struct` · `datafusion_functions_aggregate::regr::RegrAccumulator`

```rust
struct RegrAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(regr_type: &RegrType) -> Result<Self>
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.regr.RegrAccumulator.md).


`RegrAccumulator` is used to compute linear regression aggregate functions
by maintaining statistics needed to compute them in an online fashion.

This struct uses Welford's online algorithm for calculating variance and covariance:
<https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm>

Given the statistics, the following aggregate functions can be calculated:

- `regr_slope(y, x)`: Slope of the linear regression line, calculated as:
  cov_pop(x, y) / var_pop(x).
  It represents the expected change in Y for a one-unit change in X.

- `regr_intercept(y, x)`: Intercept of the linear regression line, calculated as:
  mean_y - (regr_slope(y, x) * mean_x).
  It represents the expected value of Y when X is 0.

- `regr_count(y, x)`: Count of the non-null(both x and y) input rows.

- `regr_r2(y, x)`: R-squared value (coefficient of determination), calculated as:
  (cov_pop(x, y) ^ 2) / (var_pop(x) * var_pop(y)).
  It provides a measure of how well the model's predictions match the observed data.

- `regr_avgx(y, x)`: Average of the independent variable X, calculated as: mean_x.

- `regr_avgy(y, x)`: Average of the dependent variable Y, calculated as: mean_y.

- `regr_sxx(y, x)`: Sum of squares of the independent variable X, calculated as:
  m2_x.

- `regr_syy(y, x)`: Sum of squares of the dependent variable Y, calculated as:
  m2_y.

- `regr_sxy(y, x)`: Sum of products of paired values, calculated as:
  algo_const.

Here's how the statistics maintained in this struct are calculated:
- `cov_pop(x, y)`: algo_const / count.
- `var_pop(x)`: m2_x / count.
- `var_pop(y)`: m2_y / count.

---
