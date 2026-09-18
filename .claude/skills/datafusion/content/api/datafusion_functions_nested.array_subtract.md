# `datafusion_functions_nested::array_subtract`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_subtract.json`](../model/datafusion_functions_nested.array_subtract.json)

## array_subtract

`function` · `datafusion_functions_nested::array_subtract::array_subtract`

Also reachable as `datafusion::prelude::array_subtract`, `datafusion_functions_nested::expr_fn::array_subtract`

```rust
fn array_subtract(array1: datafusion_expr::Expr, array2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_subtract.array_subtract.md).


returns the element-wise difference of two numeric arrays.

---

## array_subtract_udf

`function` · `datafusion_functions_nested::array_subtract::array_subtract_udf`

```rust
fn array_subtract_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_subtract.array_subtract_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraySubtract

---

## ArraySubtract

`struct` · `datafusion_functions_nested::array_subtract::ArraySubtract`

```rust
struct ArraySubtract
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_subtract.ArraySubtract.md).


---
