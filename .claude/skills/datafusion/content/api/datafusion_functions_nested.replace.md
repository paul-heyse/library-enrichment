# `datafusion_functions_nested::replace`

Crate `datafusion-functions-nested` · 7 public items · structured records in [`model/datafusion_functions_nested.replace.json`](../model/datafusion_functions_nested.replace.json)

## array_replace

`function` · `datafusion_functions_nested::replace::array_replace`

Also reachable as `datafusion::prelude::array_replace`, `datafusion_functions_nested::expr_fn::array_replace`

```rust
fn array_replace(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.array_replace.md).


replaces the first occurrence of the specified element with another specified element.

---

## array_replace_all

`function` · `datafusion_functions_nested::replace::array_replace_all`

Also reachable as `datafusion::prelude::array_replace_all`, `datafusion_functions_nested::expr_fn::array_replace_all`

```rust
fn array_replace_all(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.array_replace_all.md).


replaces all occurrences of the specified element with another specified element.

---

## array_replace_all_udf

`function` · `datafusion_functions_nested::replace::array_replace_all_udf`

```rust
fn array_replace_all_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.array_replace_all_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReplaceAll

---

## array_replace_n

`function` · `datafusion_functions_nested::replace::array_replace_n`

Also reachable as `datafusion::prelude::array_replace_n`, `datafusion_functions_nested::expr_fn::array_replace_n`

```rust
fn array_replace_n(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr, max: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.array_replace_n.md).


replaces the first `max` occurrences of the specified element with another specified element.

---

## array_replace_n_udf

`function` · `datafusion_functions_nested::replace::array_replace_n_udf`

```rust
fn array_replace_n_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.array_replace_n_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReplaceN

---

## array_replace_udf

`function` · `datafusion_functions_nested::replace::array_replace_udf`

```rust
fn array_replace_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.array_replace_udf.md).


ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReplace

---

## ArrayReplace

`struct` · `datafusion_functions_nested::replace::ArrayReplace`

```rust
struct ArrayReplace
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.replace.ArrayReplace.md).


---
