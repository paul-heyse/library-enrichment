# `datafusion_functions_nested::concat`

Crate `datafusion-functions-nested` · 10 public items · structured records in [`model/datafusion_functions_nested.concat.json`](../model/datafusion_functions_nested.concat.json)

## array_append

`function` · `datafusion_functions_nested::concat::array_append`

Also reachable as `datafusion::prelude::array_append`, `datafusion_functions_nested::expr_fn::array_append`

```rust
fn array_append(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_append.md).


appends an element to the end of an array.

---

## array_append_udf

`function` · `datafusion_functions_nested::concat::array_append_udf`

```rust
fn array_append_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_append_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayAppend

---

## array_concat

`function` · `datafusion_functions_nested::concat::array_concat`

Also reachable as `datafusion::prelude::array_concat`, `datafusion_functions_nested::expr_fn::array_concat`

```rust
fn array_concat(arg: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_concat.md).


Concatenates arrays.

---

## array_concat_inner

`function` · `datafusion_functions_nested::concat::array_concat_inner`

```rust
fn array_concat_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_concat_inner.md).


---

## array_concat_udf

`function` · `datafusion_functions_nested::concat::array_concat_udf`

```rust
fn array_concat_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_concat_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayConcat

---

## array_prepend

`function` · `datafusion_functions_nested::concat::array_prepend`

Also reachable as `datafusion::prelude::array_prepend`, `datafusion_functions_nested::expr_fn::array_prepend`

```rust
fn array_prepend(element: datafusion_expr::Expr, array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_prepend.md).


Prepends an element to the beginning of an array.

---

## array_prepend_udf

`function` · `datafusion_functions_nested::concat::array_prepend_udf`

```rust
fn array_prepend_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.array_prepend_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPrepend

---

## ArrayAppend

`struct` · `datafusion_functions_nested::concat::ArrayAppend`

```rust
struct ArrayAppend
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
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.ArrayAppend.md).


---

## ArrayConcat

`struct` · `datafusion_functions_nested::concat::ArrayConcat`

```rust
struct ArrayConcat
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.ArrayConcat.md).


---

## ArrayPrepend

`struct` · `datafusion_functions_nested::concat::ArrayPrepend`

```rust
struct ArrayPrepend
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
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.concat.ArrayPrepend.md).


---
