# `datafusion_functions_nested::resize`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.resize.json`](../model/datafusion_functions_nested.resize.json)

## array_resize

`function` · `datafusion_functions_nested::resize::array_resize`

Also reachable as `datafusion::prelude::array_resize`, `datafusion_functions_nested::expr_fn::array_resize`

```rust
fn array_resize(array: datafusion_expr::Expr, size: datafusion_expr::Expr, value: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns an array with the specified size filled with the given value.

---

## array_resize_udf

`function` · `datafusion_functions_nested::resize::array_resize_udf`

```rust
fn array_resize_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayResize

---

## ArrayResize

`struct` · `datafusion_functions_nested::resize::ArrayResize`

```rust
struct ArrayResize
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
