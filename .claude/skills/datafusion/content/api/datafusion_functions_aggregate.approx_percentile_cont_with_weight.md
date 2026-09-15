# `datafusion_functions_aggregate::approx_percentile_cont_with_weight`

Crate `datafusion-functions-aggregate` · 4 public items · structured records in [`model/datafusion_functions_aggregate.approx_percentile_cont_with_weight.json`](../model/datafusion_functions_aggregate.approx_percentile_cont_with_weight.json)

## approx_percentile_cont_with_weight

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::approx_percentile_cont_with_weight`

Also reachable as `datafusion_functions_aggregate::expr_fn::approx_percentile_cont_with_weight`

```rust
fn approx_percentile_cont_with_weight(order_by: datafusion_expr::expr::Sort, weight: datafusion_expr::Expr, percentile: datafusion_expr::Expr, centroids: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Computes the approximate percentile continuous with weight of a set of numbers

---

## approx_percentile_cont_with_weight_udaf

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::approx_percentile_cont_with_weight_udaf`

```rust
fn approx_percentile_cont_with_weight_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxPercentileContWithWeight`]

---

## ApproxPercentileContWithWeight

`struct` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight`

```rust
struct ApproxPercentileContWithWeight
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
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_within_group_clause(&self) -> bool
```

APPROX_PERCENTILE_CONT_WITH_WEIGHT aggregate expression

---

## ApproxPercentileWithWeightAccumulator

`struct` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator`

```rust
struct ApproxPercentileWithWeightAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(approx_percentile_cont_accumulator: ApproxPercentileAccumulator) -> Self
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
