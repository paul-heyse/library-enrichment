# `datafusion_functions_nested::position`

Crate `datafusion-functions-nested` · 6 public items · structured records in [`model/datafusion_functions_nested.position.json`](../model/datafusion_functions_nested.position.json)

## array_position

`function` · `datafusion_functions_nested::position::array_position`

Also reachable as `datafusion::prelude::array_position`, `datafusion_functions_nested::expr_fn::array_position`

```rust
fn array_position(array: datafusion_expr::Expr, element: datafusion_expr::Expr, index: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.position.array_position.md).


searches for an element in the array, returns first occurrence.

---

## array_position_udf

`function` · `datafusion_functions_nested::position::array_position_udf`

```rust
fn array_position_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.position.array_position_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPosition

---

## array_positions

`function` · `datafusion_functions_nested::position::array_positions`

Also reachable as `datafusion::prelude::array_positions`, `datafusion_functions_nested::expr_fn::array_positions`

```rust
fn array_positions(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.position.array_positions.md).


searches for an element in the array, returns all occurrences.

---

## array_positions_udf

`function` · `datafusion_functions_nested::position::array_positions_udf`

```rust
fn array_positions_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.position.array_positions_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPositions

---

## ArrayPosition

`struct` · `datafusion_functions_nested::position::ArrayPosition`

```rust
struct ArrayPosition
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.position.ArrayPosition.md).


---

## ArrayPositions

`struct` · `datafusion_functions_nested::position::ArrayPositions`

```rust
struct ArrayPositions
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.position.ArrayPositions.md).


---
