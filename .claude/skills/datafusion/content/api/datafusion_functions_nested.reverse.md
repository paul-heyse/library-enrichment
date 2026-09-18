# `datafusion_functions_nested::reverse`

Crate `datafusion-functions-nested` · 4 public items · structured records in [`model/datafusion_functions_nested.reverse.json`](../model/datafusion_functions_nested.reverse.json)

## array_reverse

`function` · `datafusion_functions_nested::reverse::array_reverse`

Also reachable as `datafusion::prelude::array_reverse`, `datafusion_functions_nested::expr_fn::array_reverse`

```rust
fn array_reverse(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.reverse.array_reverse.md).


reverses the order of elements in the array.

---

## array_reverse_inner

`function` · `datafusion_functions_nested::reverse::array_reverse_inner`

```rust
fn array_reverse_inner(arg: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.reverse.array_reverse_inner.md).


array_reverse SQL function

---

## array_reverse_udf

`function` · `datafusion_functions_nested::reverse::array_reverse_udf`

```rust
fn array_reverse_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.reverse.array_reverse_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReverse

---

## ArrayReverse

`struct` · `datafusion_functions_nested::reverse::ArrayReverse`

```rust
struct ArrayReverse
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.reverse.ArrayReverse.md).


---
