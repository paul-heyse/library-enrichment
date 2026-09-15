# `datafusion_functions_aggregate::any_value`

Crate `datafusion-functions-aggregate` · 3 public items · structured records in [`model/datafusion_functions_aggregate.any_value.json`](../model/datafusion_functions_aggregate.any_value.json)

## any_value

`function` · `datafusion_functions_aggregate::any_value::any_value`

Also reachable as `datafusion_functions_aggregate::expr_fn::any_value`

```rust
fn any_value(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Returns an arbitrary non-null value

---

## any_value_udaf

`function` · `datafusion_functions_aggregate::any_value::any_value_udaf`

```rust
fn any_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`AnyValue`]

---

## AnyValue

`struct` · `datafusion_functions_aggregate::any_value::AnyValue`

```rust
struct AnyValue
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
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

---
