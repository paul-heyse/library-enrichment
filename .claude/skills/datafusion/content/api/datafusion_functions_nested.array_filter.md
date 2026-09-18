# `datafusion_functions_nested::array_filter`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_filter.json`](../model/datafusion_functions_nested.array_filter.json)

## array_filter

`function` · `datafusion_functions_nested::array_filter::array_filter`

Also reachable as `datafusion::prelude::array_filter`, `datafusion_functions_nested::expr_fn::array_filter`

```rust
fn array_filter(array: datafusion_expr::Expr, lambda: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_filter.array_filter.md).


filters the values of an array using a boolean lambda

---

## array_filter_higher_order_function

`function` · `datafusion_functions_nested::array_filter::array_filter_higher_order_function`

```rust
fn array_filter_higher_order_function() -> std::sync::Arc<datafusion_expr::HigherOrderUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_filter.array_filter_higher_order_function.md).


HigherOrderFunction that returns a [`HigherOrderUDF`](datafusion_expr::HigherOrderUDF) for 
ArrayFilter

---

## ArrayFilter

`struct` · `datafusion_functions_nested::array_filter::ArrayFilter`

```rust
struct ArrayFilter
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
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs<'_>) -> Result<Arc<Field>>
fn signature(&self) -> &HigherOrderSignature
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_filter.ArrayFilter.md).


---
