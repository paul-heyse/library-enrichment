# `datafusion_expr::higher_order_function`

Crate `datafusion-expr` · 9 public items · structured records in [`model/datafusion_expr.higher_order_function.json`](../model/datafusion_expr.higher_order_function.json)

## HigherOrderTypeSignature

`enum` · `datafusion_expr::higher_order_function::HigherOrderTypeSignature`

Also reachable as `datafusion::logical_expr::HigherOrderTypeSignature`, `datafusion_expr::HigherOrderTypeSignature`

```rust
enum HigherOrderTypeSignature
```

**Variants**: `UserDefined`, `VariadicAny`, `Any`, `Exact`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.HigherOrderTypeSignature.md).


The types of arguments for which a function has implementations.

[`HigherOrderTypeSignature`] **DOES NOT** define the types that a user query could call the
function with. DataFusion will automatically coerce (cast) argument types to
one of the supported function signatures, if possible.

# Overview
Functions typically provide implementations for a small number of different
argument [`DataType`]s, rather than all possible combinations. If a user
calls a function with arguments that do not match any of the declared types,
DataFusion will attempt to automatically coerce (add casts to) function
arguments so they match the [`HigherOrderTypeSignature`]. See the [`type_coercion`] module
for more details

[`type_coercion`]: crate::type_coercion

---

## LambdaParametersProgress

`enum` · `datafusion_expr::higher_order_function::LambdaParametersProgress`

Also reachable as `datafusion::logical_expr::LambdaParametersProgress`, `datafusion_expr::LambdaParametersProgress`

```rust
enum LambdaParametersProgress
```

**Variants**: `Partial`, `Complete`

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md).


Represents a step during the resolution of the parameters of all lambdas of a given
higher-order function via [HigherOrderUDFImpl::lambda_parameters]. It's valid that the
fields of a given lambda changes between steps, and is up to the implementation to
provide during the function evaluation the parameters that matches the fields returned
at the [LambdaParametersProgress::Complete] step. See [HigherOrderUDFImpl::lambda_parameters]
docs for more details

---

## ValueOrLambda

`enum` · `datafusion_expr::higher_order_function::ValueOrLambda`

Also reachable as `datafusion::logical_expr::ValueOrLambda`, `datafusion_expr::ValueOrLambda`

```rust
enum ValueOrLambda<V, L>
```

**Variants**: `Value`, `Lambda`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.ValueOrLambda.md).


An argument to a higher order function

---

## HigherOrderFunctionArgs

`struct` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs`

Also reachable as `datafusion::logical_expr::HigherOrderFunctionArgs`, `datafusion_expr::HigherOrderFunctionArgs`

```rust
struct HigherOrderFunctionArgs
```

**Fields**: `args`, `arg_fields`, `number_rows`, `return_field`, `config_options`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn return_type(&self) -> &DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.HigherOrderFunctionArgs.md).


Arguments passed to [`HigherOrderUDFImpl::invoke_with_args`] when invoking a
higher order function.

---

## HigherOrderReturnFieldArgs

`struct` · `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs`

Also reachable as `datafusion::logical_expr::HigherOrderReturnFieldArgs`, `datafusion_expr::HigherOrderReturnFieldArgs`

```rust
struct HigherOrderReturnFieldArgs<'a>
```

**Fields**: `arg_fields`, `scalar_arguments`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.HigherOrderReturnFieldArgs.md).


Information about arguments passed to the function

This structure contains metadata about how the function was called
such as the type of the arguments, any scalar arguments and if the
arguments can (ever) be null

See [`HigherOrderUDFImpl::return_field_from_args`] for more information

---

## HigherOrderSignature

`struct` · `datafusion_expr::higher_order_function::HigherOrderSignature`

Also reachable as `datafusion::logical_expr::HigherOrderSignature`, `datafusion_expr::HigherOrderSignature`

```rust
struct HigherOrderSignature
```

**Fields**: `type_signature`, `volatility`, `lambda_parameters_max_iterations`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn any(arg_count: usize, volatility: Volatility) -> Self
fn exact(args: Vec<ValueOrLambda<(), ()>>, volatility: Volatility) -> Self
fn new(type_signature: HigherOrderTypeSignature, volatility: Volatility) -> Self
fn user_defined(volatility: Volatility) -> Self
fn variadic_any(volatility: Volatility) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.HigherOrderSignature.md).


Provides information necessary for calling a higher order function.

- [`HigherOrderTypeSignature`] defines the argument types that a function has implementations
  for.

- [`Volatility`] defines how the output of the function changes with the input.

---

## HigherOrderUDF

`struct` · `datafusion_expr::higher_order_function::HigherOrderUDF`

Also reachable as `datafusion::logical_expr::HigherOrderUDF`, `datafusion_expr::HigherOrderUDF`

```rust
struct HigherOrderUDF
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (17)

```rust
fn aliases(&self) -> &[String]
fn clear_null_values(&self) -> bool
fn coerce_value_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn coerce_values_for_lambdas(&self, fields: &[ValueOrLambda<DataType, DataType>]) -> Result<Option<Vec<DataType>>>
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
fn documentation(&self) -> Option<&Documentation>
fn inner(&self) -> &Arc<dyn HigherOrderUDFImpl>
fn invoke_with_args(&self, args: HigherOrderFunctionArgs) -> Result<ColumnarValue>
fn lambda_parameters(&self, step: usize, fields: &[ValueOrLambda<FieldRef, Option<FieldRef>>]) -> Result<LambdaParametersProgress>
fn name(&self) -> &str
fn new_from_impl<F>(fun: F) -> HigherOrderUDF where F: HigherOrderUDFImpl + 'static
fn new_from_shared_impl(fun: Arc<dyn HigherOrderUDFImpl>) -> HigherOrderUDF
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs<'_>) -> Result<FieldRef>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn short_circuits(&self) -> bool
fn signature(&self) -> &HigherOrderSignature
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

**via `core::convert::From`**

```rust
fn from(fun: F) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md).


Logical representation of a Higher Order User Defined Function.

A higher order function takes one or more lambda arguments in addition to
regular value arguments. This struct contains the information DataFusion
needs to plan and invoke functions you supply such as name, type signature,
return type, and actual implementation.

---

## LambdaArgument

`struct` · `datafusion_expr::higher_order_function::LambdaArgument`

Also reachable as `datafusion::logical_expr::LambdaArgument`, `datafusion_expr::LambdaArgument`

```rust
struct LambdaArgument
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn evaluate(&self, args: &[&dyn Fn() -> Result<ArrayRef>], spread_captures: impl FnOnce(&[ArrayRef]) -> Result<Vec<ArrayRef>>) -> Result<ColumnarValue>
fn new(params: Vec<FieldRef>, body: Arc<dyn PhysicalExpr>, captures: Option<RecordBatch>, used_param_indices: &[usize]) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.LambdaArgument.md).


A lambda argument to a HigherOrderFunction

---

## HigherOrderUDFImpl

`trait` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl`

Also reachable as `datafusion::logical_expr::HigherOrderUDFImpl`, `datafusion_expr::HigherOrderUDFImpl`

```rust
trait HigherOrderUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

**Implementors** (4)

- `datafusion_functions_nested::array_any_match::ArrayAnyMatch`
- `datafusion_functions_nested::array_filter::ArrayFilter`
- `datafusion_functions_nested::array_first::ArrayFirst`
- `datafusion_functions_nested::array_transform::ArrayTransform`

**Methods** (13)

```rust
fn aliases(&self) -> &[String]
fn clear_null_values(&self) -> bool
fn coerce_value_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
fn coerce_values_for_lambdas(&self, _fields: &[ValueOrLambda<DataType, DataType>]) -> Result<Option<Vec<DataType>>>
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: HigherOrderFunctionArgs) -> Result<ColumnarValue>
fn lambda_parameters(&self, step: usize, fields: &[ValueOrLambda<FieldRef, Option<FieldRef>>]) -> Result<LambdaParametersProgress>
fn name(&self) -> &str
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs<'_>) -> Result<FieldRef>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn short_circuits(&self) -> bool
fn signature(&self) -> &HigherOrderSignature
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md).


Trait for implementing user defined higher order functions.

This trait exposes the full API for implementing user defined functions and
can be used to implement any function.

New higher order functions typically implement this trait and are then
wrapped in a [`HigherOrderUDF`] for registration with DataFusion.

See [`array_transform.rs`] for a commented complete implementation

[`array_transform.rs`]: https://github.com/apache/datafusion/blob/main/datafusion/functions-nested/src/array_transform.rs

---
