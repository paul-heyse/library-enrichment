# `datafusion_functions_aggregate::bool_and_or`

Crate `datafusion-functions-aggregate` · 6 public items · structured records in [`model/datafusion_functions_aggregate.bool_and_or.json`](../model/datafusion_functions_aggregate.bool_and_or.json)

## bool_and

`function` · `datafusion_functions_aggregate::bool_and_or::bool_and`

Also reachable as `datafusion_functions_aggregate::expr_fn::bool_and`

```rust
fn bool_and(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bool_and_or.bool_and.md).


The values to combine with `AND`

---

## bool_and_udaf

`function` · `datafusion_functions_aggregate::bool_and_or::bool_and_udaf`

```rust
fn bool_and_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bool_and_or.bool_and_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`BoolAnd`]

---

## bool_or

`function` · `datafusion_functions_aggregate::bool_and_or::bool_or`

Also reachable as `datafusion_functions_aggregate::expr_fn::bool_or`

```rust
fn bool_or(expression: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bool_and_or.bool_or.md).


The values to combine with `OR`

---

## bool_or_udaf

`function` · `datafusion_functions_aggregate::bool_and_or::bool_or_udaf`

```rust
fn bool_or_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bool_and_or.bool_or_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`BoolOr`]

---

## BoolAnd

`struct` · `datafusion_functions_aggregate::bool_and_or::BoolAnd`

```rust
struct BoolAnd
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, _: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bool_and_or.BoolAnd.md).


BOOL_AND aggregate expression

---

## BoolOr

`struct` · `datafusion_functions_aggregate::bool_and_or::BoolOr`

```rust
struct BoolOr
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, _: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn documentation(&self) -> Option<&Documentation>
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_type(&self, _: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bool_and_or.BoolOr.md).


BOOL_OR aggregate expression

---
