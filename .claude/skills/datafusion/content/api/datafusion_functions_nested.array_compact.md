# `datafusion_functions_nested::array_compact`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_compact.json`](../model/datafusion_functions_nested.array_compact.json)

## array_compact

`function` · `datafusion_functions_nested::array_compact::array_compact`

Also reachable as `datafusion::prelude::array_compact`, `datafusion_functions_nested::expr_fn::array_compact`

```rust
fn array_compact(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

removes null values from the array.

---

## array_compact_udf

`function` · `datafusion_functions_nested::array_compact::array_compact_udf`

```rust
fn array_compact_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayCompact

---

## ArrayCompact

`struct` · `datafusion_functions_nested::array_compact::ArrayCompact`

```rust
struct ArrayCompact
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
