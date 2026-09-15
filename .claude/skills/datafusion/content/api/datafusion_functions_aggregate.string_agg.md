# `datafusion_functions_aggregate::string_agg`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.string_agg.json`](../model/datafusion_functions_aggregate.string_agg.json)

## string_agg

`function` · `datafusion_functions_aggregate::string_agg::string_agg`

```rust
fn string_agg(expr: datafusion_expr::Expr, delimiter: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Concatenates the values of string expressions and places separator values between them

---

## string_agg_udaf

`function` · `datafusion_functions_aggregate::string_agg::string_agg_udaf`

```rust
fn string_agg_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`StringAgg`]

---

## StringAgg

`struct` · `datafusion_functions_aggregate::string_agg::StringAgg`

```rust
struct StringAgg
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
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

STRING_AGG aggregate expression

---
