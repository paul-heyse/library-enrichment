# `datafusion_functions_nested::min_max`

Crate `datafusion-functions-nested` · 5 public items · structured records in [`model/datafusion_functions_nested.min_max.json`](../model/datafusion_functions_nested.min_max.json)

## array_max

`function` · `datafusion_functions_nested::min_max::array_max`

Also reachable as `datafusion::prelude::array_max`, `datafusion_functions_nested::expr_fn::array_max`

```rust
fn array_max(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the maximum value in the array.

---

## array_max_udf

`function` · `datafusion_functions_nested::min_max::array_max_udf`

```rust
fn array_max_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayMax

---

## array_min

`function` · `datafusion_functions_nested::min_max::array_min`

Also reachable as `datafusion::prelude::array_min`, `datafusion_functions_nested::expr_fn::array_min`

```rust
fn array_min(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the minimum value in the array

---

## array_min_udf

`function` · `datafusion_functions_nested::min_max::array_min_udf`

```rust
fn array_min_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayMin

---

## ArrayMax

`struct` · `datafusion_functions_nested::min_max::ArrayMax`

```rust
struct ArrayMax
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
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
