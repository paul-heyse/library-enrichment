# `datafusion_functions_aggregate::approx_distinct`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.approx_distinct.json`](../model/datafusion_functions_aggregate.approx_distinct.json)

## approx_distinct

`function` · `datafusion_functions_aggregate::approx_distinct::approx_distinct`

Also reachable as `datafusion_functions_aggregate::expr_fn::approx_distinct`

```rust
fn approx_distinct(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.approx_distinct.approx_distinct.md).


approximate number of distinct input values

---

## approx_distinct_udaf

`function` · `datafusion_functions_aggregate::approx_distinct::approx_distinct_udaf`

```rust
fn approx_distinct_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.approx_distinct.approx_distinct_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxDistinct`]

---

## ApproxDistinct

`struct` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct`

```rust
struct ApproxDistinct
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
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.approx_distinct.ApproxDistinct.md).


---
