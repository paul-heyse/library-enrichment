# `datafusion_functions_nested::array_normalize`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_normalize.json`](../model/datafusion_functions_nested.array_normalize.json)

## array_normalize

`function` · `datafusion_functions_nested::array_normalize::array_normalize`

Also reachable as `datafusion::prelude::array_normalize`, `datafusion_functions_nested::expr_fn::array_normalize`

```rust
fn array_normalize(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the L2-normalized vector for a numeric array.

---

## array_normalize_udf

`function` · `datafusion_functions_nested::array_normalize::array_normalize_udf`

```rust
fn array_normalize_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayNormalize

---

## ArrayNormalize

`struct` · `datafusion_functions_nested::array_normalize::ArrayNormalize`

```rust
struct ArrayNormalize
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
