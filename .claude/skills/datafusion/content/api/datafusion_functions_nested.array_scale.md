# `datafusion_functions_nested::array_scale`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_scale.json`](../model/datafusion_functions_nested.array_scale.json)

## array_scale

`function` · `datafusion_functions_nested::array_scale::array_scale`

Also reachable as `datafusion::prelude::array_scale`, `datafusion_functions_nested::expr_fn::array_scale`

```rust
fn array_scale(array: datafusion_expr::Expr, scalar: datafusion_expr::Expr) -> datafusion_expr::Expr
```

scales each element of a numeric array by a scalar.

---

## array_scale_udf

`function` · `datafusion_functions_nested::array_scale::array_scale_udf`

```rust
fn array_scale_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayScale

---

## ArrayScale

`struct` · `datafusion_functions_nested::array_scale::ArrayScale`

```rust
struct ArrayScale
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
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
