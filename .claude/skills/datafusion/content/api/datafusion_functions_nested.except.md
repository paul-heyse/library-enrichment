# `datafusion_functions_nested::except`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.except.json`](../model/datafusion_functions_nested.except.json)

## array_except

`function` · `datafusion_functions_nested::except::array_except`

Also reachable as `datafusion::prelude::array_except`, `datafusion_functions_nested::expr_fn::array_except`

```rust
fn array_except(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns an array of the elements that appear in the first array but not in the second.

---

## array_except_udf

`function` · `datafusion_functions_nested::except::array_except_udf`

```rust
fn array_except_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayExcept

---

## ArrayExcept

`struct` · `datafusion_functions_nested::except::ArrayExcept`

```rust
struct ArrayExcept
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
