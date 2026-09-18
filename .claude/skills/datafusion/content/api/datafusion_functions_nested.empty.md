# `datafusion_functions_nested::empty`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.empty.json`](../model/datafusion_functions_nested.empty.json)

## array_empty

`function` · `datafusion_functions_nested::empty::array_empty`

Also reachable as `datafusion::prelude::array_empty`, `datafusion_functions_nested::expr_fn::array_empty`

```rust
fn array_empty(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.empty.array_empty.md).


returns true for an empty array or false for a non-empty array.

---

## array_empty_udf

`function` · `datafusion_functions_nested::empty::array_empty_udf`

```rust
fn array_empty_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.empty.array_empty_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayEmpty

---

## ArrayEmpty

`struct` · `datafusion_functions_nested::empty::ArrayEmpty`

```rust
struct ArrayEmpty
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.empty.ArrayEmpty.md).


---
