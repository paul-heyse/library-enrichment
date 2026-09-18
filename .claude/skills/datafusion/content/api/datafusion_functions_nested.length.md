# `datafusion_functions_nested::length`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.length.json`](../model/datafusion_functions_nested.length.json)

## array_length

`function` · `datafusion_functions_nested::length::array_length`

Also reachable as `datafusion::prelude::array_length`, `datafusion_functions_nested::expr_fn::array_length`

```rust
fn array_length(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.length.array_length.md).


returns the length of the array dimension.

---

## array_length_udf

`function` · `datafusion_functions_nested::length::array_length_udf`

```rust
fn array_length_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.length.array_length_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayLength

---

## ArrayLength

`struct` · `datafusion_functions_nested::length::ArrayLength`

```rust
struct ArrayLength
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
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.length.ArrayLength.md).


---
