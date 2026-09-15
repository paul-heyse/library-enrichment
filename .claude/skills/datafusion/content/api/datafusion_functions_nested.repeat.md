# `datafusion_functions_nested::repeat`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.repeat.json`](../model/datafusion_functions_nested.repeat.json)

## array_repeat

`function` · `datafusion_functions_nested::repeat::array_repeat`

Also reachable as `datafusion::prelude::array_repeat`, `datafusion_functions_nested::expr_fn::array_repeat`

```rust
fn array_repeat(element: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr
```

returns an array containing element `count` times.

---

## array_repeat_udf

`function` · `datafusion_functions_nested::repeat::array_repeat_udf`

```rust
fn array_repeat_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRepeat

---

## ArrayRepeat

`struct` · `datafusion_functions_nested::repeat::ArrayRepeat`

```rust
struct ArrayRepeat
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
