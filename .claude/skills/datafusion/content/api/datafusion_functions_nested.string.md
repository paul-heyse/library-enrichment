# `datafusion_functions_nested::string`

Crate `datafusion-functions-nested` · 6 public items · structured records in [`model/datafusion_functions_nested.string.json`](../model/datafusion_functions_nested.string.json)

## array_to_string

`function` · `datafusion_functions_nested::string::array_to_string`

Also reachable as `datafusion::prelude::array_to_string`, `datafusion_functions_nested::expr_fn::array_to_string`

```rust
fn array_to_string(array: datafusion_expr::Expr, delimiter: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.string.array_to_string.md).


converts each element to its text representation.

---

## array_to_string_udf

`function` · `datafusion_functions_nested::string::array_to_string_udf`

```rust
fn array_to_string_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.string.array_to_string_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayToString

---

## string_to_array

`function` · `datafusion_functions_nested::string::string_to_array`

Also reachable as `datafusion::prelude::string_to_array`, `datafusion_functions_nested::expr_fn::string_to_array`

```rust
fn string_to_array(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, null_string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.string.string_to_array.md).


splits a `string` based on a `delimiter` and returns an array of parts. Any parts matching the optional `null_string` will be replaced with `NULL`

---

## string_to_array_udf

`function` · `datafusion_functions_nested::string::string_to_array_udf`

```rust
fn string_to_array_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.string.string_to_array_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
StringToArray

---

## ArrayToString

`struct` · `datafusion_functions_nested::string::ArrayToString`

```rust
struct ArrayToString
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.string.ArrayToString.md).


---

## StringToArray

`struct` · `datafusion_functions_nested::string::StringToArray`

```rust
struct StringToArray
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.string.StringToArray.md).


---
