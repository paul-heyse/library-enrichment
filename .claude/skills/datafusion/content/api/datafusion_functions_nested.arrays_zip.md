# `datafusion_functions_nested::arrays_zip`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.arrays_zip.json`](../model/datafusion_functions_nested.arrays_zip.json)

## arrays_zip

`function` · `datafusion_functions_nested::arrays_zip::arrays_zip`

Also reachable as `datafusion::prelude::arrays_zip`, `datafusion_functions_nested::expr_fn::arrays_zip`

```rust
fn arrays_zip(arg: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.arrays_zip.arrays_zip.md).


combines one or multiple arrays into a single array of structs.

---

## arrays_zip_udf

`function` · `datafusion_functions_nested::arrays_zip::arrays_zip_udf`

```rust
fn arrays_zip_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.arrays_zip.arrays_zip_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraysZip

---

## ArraysZip

`struct` · `datafusion_functions_nested::arrays_zip::ArraysZip`

```rust
struct ArraysZip
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.arrays_zip.ArraysZip.md).


---
