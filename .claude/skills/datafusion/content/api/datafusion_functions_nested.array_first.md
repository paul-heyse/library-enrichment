# `datafusion_functions_nested::array_first`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_first.json`](../model/datafusion_functions_nested.array_first.json)

## array_first

`function` · `datafusion_functions_nested::array_first::array_first`

Also reachable as `datafusion::prelude::array_first`, `datafusion_functions_nested::expr_fn::array_first`

```rust
fn array_first(array: datafusion_expr::Expr, lambda: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_first.array_first.md).


returns the first element of an array that satisfies the predicate

---

## array_first_higher_order_function

`function` · `datafusion_functions_nested::array_first::array_first_higher_order_function`

```rust
fn array_first_higher_order_function() -> std::sync::Arc<datafusion_expr::HigherOrderUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_first.array_first_higher_order_function.md).


HigherOrderFunction that returns a [`HigherOrderUDF`](datafusion_expr::HigherOrderUDF) for 
ArrayFirst

---

## ArrayFirst

`struct` · `datafusion_functions_nested::array_first::ArrayFirst`

```rust
struct ArrayFirst
```

**Implements**: `datafusion_expr::higher_order_function::HigherOrderUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::higher_order_function::HigherOrderUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn coerce_value_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: HigherOrderFunctionArgs) -> Result<ColumnarValue>
fn lambda_parameters(&self, _step: usize, fields: &[ValueOrLambda<FieldRef, Option<FieldRef>>]) -> Result<LambdaParametersProgress>
fn name(&self) -> &str
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs<'_>) -> Result<FieldRef>
fn signature(&self) -> &HigherOrderSignature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_first.ArrayFirst.md).


---
