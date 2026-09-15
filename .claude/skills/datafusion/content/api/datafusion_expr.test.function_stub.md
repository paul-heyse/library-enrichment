# `datafusion_expr::test::function_stub`

Crate `datafusion-expr` · 15 public items · structured records in [`model/datafusion_expr.test.function_stub.json`](../model/datafusion_expr.test.function_stub.json)

## avg

`function` · `datafusion_expr::test::function_stub::avg`

```rust
fn avg(expr: Expr) -> Expr
```

---

## avg_udaf

`function` · `datafusion_expr::test::function_stub::avg_udaf`

```rust
fn avg_udaf() -> std::sync::Arc<AggregateUDF>
```

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Avg`]

---

## count

`function` · `datafusion_expr::test::function_stub::count`

```rust
fn count(expr: Expr) -> Expr
```

---

## count_udaf

`function` · `datafusion_expr::test::function_stub::count_udaf`

```rust
fn count_udaf() -> std::sync::Arc<AggregateUDF>
```

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Count`]

---

## max

`function` · `datafusion_expr::test::function_stub::max`

```rust
fn max(expr: Expr) -> Expr
```

---

## max_udaf

`function` · `datafusion_expr::test::function_stub::max_udaf`

```rust
fn max_udaf() -> std::sync::Arc<AggregateUDF>
```

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Max`]

---

## min

`function` · `datafusion_expr::test::function_stub::min`

```rust
fn min(expr: Expr) -> Expr
```

---

## min_udaf

`function` · `datafusion_expr::test::function_stub::min_udaf`

```rust
fn min_udaf() -> std::sync::Arc<AggregateUDF>
```

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Min`]

---

## sum

`function` · `datafusion_expr::test::function_stub::sum`

```rust
fn sum(expr: Expr) -> Expr
```

---

## sum_udaf

`function` · `datafusion_expr::test::function_stub::sum_udaf`

```rust
fn sum_udaf() -> std::sync::Arc<AggregateUDF>
```

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Sum`]

---

## Avg

`struct` · `datafusion_expr::test::function_stub::Avg`

```rust
struct Avg
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
fn aliases(&self) -> &[String]
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Testing stub implementation of avg aggregate

---

## Count

`struct` · `datafusion_expr::test::function_stub::Count`

```rust
struct Count
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
fn aliases(&self) -> &[String]
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Testing stub implementation of COUNT aggregate

---

## Max

`struct` · `datafusion_expr::test::function_stub::Max`

```rust
struct Max
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
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn is_descending(&self) -> Option<bool>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Testing stub implementation of MAX aggregate

---

## Min

`struct` · `datafusion_expr::test::function_stub::Min`

```rust
struct Min
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
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn is_descending(&self) -> Option<bool>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Testing stub implementation of Min aggregate

---

## Sum

`struct` · `datafusion_expr::test::function_stub::Sum`

```rust
struct Sum
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn reverse_expr(&self) -> ReversedUDAF
fn signature(&self) -> &Signature
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Stub `sum` used for optimizer testing

---
