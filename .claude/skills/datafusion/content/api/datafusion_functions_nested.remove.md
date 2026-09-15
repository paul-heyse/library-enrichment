# `datafusion_functions_nested::remove`

Crate `datafusion-functions-nested` · 9 public items · structured records in [`model/datafusion_functions_nested.remove.json`](../model/datafusion_functions_nested.remove.json)

## array_remove

`function` · `datafusion_functions_nested::remove::array_remove`

Also reachable as `datafusion::prelude::array_remove`, `datafusion_functions_nested::expr_fn::array_remove`

```rust
fn array_remove(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

removes the first element from the array equal to the given value. NULL elements already in the array are preserved when removing a non-NULL value. If `element` evaluates to NULL, the result is NULL rather than removing NULL entries.

---

## array_remove_all

`function` · `datafusion_functions_nested::remove::array_remove_all`

Also reachable as `datafusion::prelude::array_remove_all`, `datafusion_functions_nested::expr_fn::array_remove_all`

```rust
fn array_remove_all(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

removes all elements from the array equal to the given value. NULL elements already in the array are preserved when removing a non-NULL value. If `element` evaluates to NULL, the result is NULL rather than removing NULL entries.

---

## array_remove_all_udf

`function` · `datafusion_functions_nested::remove::array_remove_all_udf`

```rust
fn array_remove_all_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRemoveAll

---

## array_remove_n

`function` · `datafusion_functions_nested::remove::array_remove_n`

Also reachable as `datafusion::prelude::array_remove_n`, `datafusion_functions_nested::expr_fn::array_remove_n`

```rust
fn array_remove_n(array: datafusion_expr::Expr, element: datafusion_expr::Expr, max: datafusion_expr::Expr) -> datafusion_expr::Expr
```

removes the first `max` elements from the array equal to the given value. NULL elements already in the array are preserved when removing a non-NULL value. If `element` evaluates to NULL, the result is NULL rather than removing NULL entries.

---

## array_remove_n_udf

`function` · `datafusion_functions_nested::remove::array_remove_n_udf`

```rust
fn array_remove_n_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRemoveN

---

## array_remove_udf

`function` · `datafusion_functions_nested::remove::array_remove_udf`

```rust
fn array_remove_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRemove

---

## ArrayRemove

`struct` · `datafusion_functions_nested::remove::ArrayRemove`

```rust
struct ArrayRemove
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
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---

## ArrayRemoveAll

`struct` · `datafusion_functions_nested::remove::ArrayRemoveAll`

```rust
struct ArrayRemoveAll
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
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---

## ArrayRemoveN

`struct` · `datafusion_functions_nested::remove::ArrayRemoveN`

```rust
struct ArrayRemoveN
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
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
