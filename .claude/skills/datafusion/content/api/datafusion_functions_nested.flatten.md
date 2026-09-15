# `datafusion_functions_nested::flatten`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.flatten.json`](../model/datafusion_functions_nested.flatten.json)

## flatten

`function` · `datafusion_functions_nested::flatten::flatten`

Also reachable as `datafusion::prelude::flatten`, `datafusion_functions_nested::expr_fn::flatten`

```rust
fn flatten(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

flattens an array of arrays into a single array.

---

## flatten_udf

`function` · `datafusion_functions_nested::flatten::flatten_udf`

```rust
fn flatten_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
Flatten

---

## Flatten

`struct` · `datafusion_functions_nested::flatten::Flatten`

```rust
struct Flatten
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
