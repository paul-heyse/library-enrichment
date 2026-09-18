# `datafusion_functions_aggregate::percentile_cont`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.percentile_cont.json`](../model/datafusion_functions_aggregate.percentile_cont.json)

## percentile_cont

`function` · `datafusion_functions_aggregate::percentile_cont::percentile_cont`

Also reachable as `datafusion_functions_aggregate::expr_fn::percentile_cont`

```rust
fn percentile_cont(order_by: datafusion_expr::expr::Sort, percentile: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.percentile_cont.percentile_cont.md).


Computes the exact percentile continuous of a set of numbers

---

## percentile_cont_udaf

`function` · `datafusion_functions_aggregate::percentile_cont::percentile_cont_udaf`

```rust
fn percentile_cont_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.percentile_cont.percentile_cont_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`PercentileCont`]

---

## PercentileCont

`struct` · `datafusion_functions_aggregate::percentile_cont::PercentileCont`

```rust
struct PercentileCont
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn aliases(&self) -> &[String]
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self) -> Option<AggregateFunctionSimplification>
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_within_group_clause(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.percentile_cont.PercentileCont.md).


PERCENTILE_CONT aggregate expression. This uses an exact calculation and stores all values
in memory before computing the result. If an approximation is sufficient then
APPROX_PERCENTILE_CONT provides a much more efficient solution.

If using the distinct variation, the memory usage will be similarly high if the
cardinality is high as it stores all distinct values in memory before computing the
result, but if cardinality is low then memory usage will also be lower.

---
