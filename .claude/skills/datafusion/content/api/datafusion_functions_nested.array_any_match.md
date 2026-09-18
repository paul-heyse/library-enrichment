# `datafusion_functions_nested::array_any_match`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.array_any_match.json`](../model/datafusion_functions_nested.array_any_match.json)

## array_any_match

`function` · `datafusion_functions_nested::array_any_match::array_any_match`

Also reachable as `datafusion::prelude::array_any_match`, `datafusion_functions_nested::expr_fn::array_any_match`

```rust
fn array_any_match(array: datafusion_expr::Expr, lambda: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_any_match.array_any_match.md).


returns true if any element in the array satisfies the predicate

---

## array_any_match_higher_order_function

`function` · `datafusion_functions_nested::array_any_match::array_any_match_higher_order_function`

```rust
fn array_any_match_higher_order_function() -> std::sync::Arc<datafusion_expr::HigherOrderUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_any_match.array_any_match_higher_order_function.md).


HigherOrderFunction that returns a [`HigherOrderUDF`](datafusion_expr::HigherOrderUDF) for 
ArrayAnyMatch

---

## ArrayAnyMatch

`struct` · `datafusion_functions_nested::array_any_match::ArrayAnyMatch`

```rust
struct ArrayAnyMatch
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_nested.array_any_match.ArrayAnyMatch.md).


---
