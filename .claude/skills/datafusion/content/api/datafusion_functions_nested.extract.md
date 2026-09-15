# `datafusion_functions_nested::extract`

Crate `datafusion-functions-nested` · 11 public items · structured records in [`model/datafusion_functions_nested.extract.json`](../model/datafusion_functions_nested.extract.json)

## array_any_value

`function` · `datafusion_functions_nested::extract::array_any_value`

Also reachable as `datafusion::prelude::array_any_value`, `datafusion_functions_nested::expr_fn::array_any_value`

```rust
fn array_any_value(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the first non-null element in the array.

---

## array_any_value_udf

`function` · `datafusion_functions_nested::extract::array_any_value_udf`

```rust
fn array_any_value_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayAnyValue

---

## array_element

`function` · `datafusion_functions_nested::extract::array_element`

Also reachable as `datafusion::prelude::array_element`, `datafusion_functions_nested::expr_fn::array_element`

```rust
fn array_element(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

extracts the element with the index n from the array.

---

## array_element_udf

`function` · `datafusion_functions_nested::extract::array_element_udf`

```rust
fn array_element_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayElement

---

## array_pop_back

`function` · `datafusion_functions_nested::extract::array_pop_back`

Also reachable as `datafusion::prelude::array_pop_back`, `datafusion_functions_nested::expr_fn::array_pop_back`

```rust
fn array_pop_back(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the array without the last element.

---

## array_pop_back_udf

`function` · `datafusion_functions_nested::extract::array_pop_back_udf`

```rust
fn array_pop_back_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPopBack

---

## array_pop_front

`function` · `datafusion_functions_nested::extract::array_pop_front`

Also reachable as `datafusion::prelude::array_pop_front`, `datafusion_functions_nested::expr_fn::array_pop_front`

```rust
fn array_pop_front(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the array without the first element.

---

## array_pop_front_udf

`function` · `datafusion_functions_nested::extract::array_pop_front_udf`

```rust
fn array_pop_front_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPopFront

---

## array_slice

`function` · `datafusion_functions_nested::extract::array_slice`

Also reachable as `datafusion::prelude::array_slice`, `datafusion_functions_nested::expr_fn::array_slice`

```rust
fn array_slice(array: datafusion_expr::Expr, begin: datafusion_expr::Expr, end: datafusion_expr::Expr, stride: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

returns a slice of the array.

---

## array_slice_udf

`function` · `datafusion_functions_nested::extract::array_slice_udf`

```rust
fn array_slice_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraySlice

---

## ArrayElement

`struct` · `datafusion_functions_nested::extract::ArrayElement`

```rust
struct ArrayElement
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
fn display_name(&self, args: &[Expr]) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn signature(&self) -> &Signature
```

---
