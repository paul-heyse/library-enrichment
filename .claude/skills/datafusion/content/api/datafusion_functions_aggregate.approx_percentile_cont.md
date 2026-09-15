# `datafusion_functions_aggregate::approx_percentile_cont`

Crate `datafusion-functions-aggregate` · 4 public items · structured records in [`model/datafusion_functions_aggregate.approx_percentile_cont.json`](../model/datafusion_functions_aggregate.approx_percentile_cont.json)

## approx_percentile_cont

`function` · `datafusion_functions_aggregate::approx_percentile_cont::approx_percentile_cont`

Also reachable as `datafusion_functions_aggregate::expr_fn::approx_percentile_cont`

```rust
fn approx_percentile_cont(order_by: datafusion_expr::expr::Sort, percentile: datafusion_expr::Expr, centroids: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Computes the approximate percentile continuous of a set of numbers

---

## approx_percentile_cont_udaf

`function` · `datafusion_functions_aggregate::approx_percentile_cont::approx_percentile_cont_udaf`

```rust
fn approx_percentile_cont_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxPercentileCont`]

---

## ApproxPercentileAccumulator

`struct` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator`

```rust
struct ApproxPercentileAccumulator
```

**Implements**: `datafusion_expr_common::accumulator::Accumulator`

**Derives**: Debug

**Methods** (2)

```rust
fn new(percentile: f64, return_type: DataType) -> Self
fn new_with_max_size(percentile: f64, return_type: DataType, max_size: usize) -> Self
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

## ApproxPercentileCont

`struct` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileCont`

```rust
struct ApproxPercentileCont
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

---
