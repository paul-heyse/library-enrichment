# `datafusion_functions_aggregate::grouping`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.grouping.json`](../model/datafusion_functions_aggregate.grouping.json)

## grouping

`function` · `datafusion_functions_aggregate::grouping::grouping`

Also reachable as `datafusion_functions_aggregate::expr_fn::grouping`

```rust
fn grouping(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.grouping.grouping.md).


Returns 1 if the data is aggregated across the specified column or 0 for not aggregated in the result set.

---

## grouping_udaf

`function` · `datafusion_functions_aggregate::grouping::grouping_udaf`

```rust
fn grouping_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.grouping.grouping_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Grouping`]

---

## Grouping

`struct` · `datafusion_functions_aggregate::grouping::Grouping`

```rust
struct Grouping
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.grouping.Grouping.md).


---
