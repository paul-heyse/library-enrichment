# `datafusion_functions_aggregate::approx_median`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.approx_median.json`](../model/datafusion_functions_aggregate.approx_median.json)

## approx_median

`function` · `datafusion_functions_aggregate::approx_median::approx_median`

Also reachable as `datafusion_functions_aggregate::expr_fn::approx_median`

```rust
fn approx_median(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.approx_median.approx_median.md).


Computes the approximate median of a set of numbers

---

## approx_median_udaf

`function` · `datafusion_functions_aggregate::approx_median::approx_median_udaf`

```rust
fn approx_median_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.approx_median.approx_median_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxMedian`]

---

## ApproxMedian

`struct` · `datafusion_functions_aggregate::approx_median::ApproxMedian`

```rust
struct ApproxMedian
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
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.approx_median.ApproxMedian.md).


APPROX_MEDIAN aggregate expression

---
