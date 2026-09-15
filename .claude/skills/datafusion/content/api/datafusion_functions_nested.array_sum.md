# `datafusion_functions_nested::array_sum`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_sum.json`](../model/datafusion_functions_nested.array_sum.json)

## array_sum

`function` · `datafusion_functions_nested::array_sum::array_sum`

Also reachable as `datafusion::prelude::array_sum`, `datafusion_functions_nested::expr_fn::array_sum`

```rust
fn array_sum(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns the sum of elements in a numeric array.

---

## array_sum_udf

`function` · `datafusion_functions_nested::array_sum::array_sum_udf`

```rust
fn array_sum_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraySum

---

## ArraySum

`struct` · `datafusion_functions_nested::array_sum::ArraySum`

```rust
struct ArraySum
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
