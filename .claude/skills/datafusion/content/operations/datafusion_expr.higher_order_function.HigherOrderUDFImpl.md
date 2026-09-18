# `datafusion_expr::higher_order_function::HigherOrderUDFImpl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.HigherOrderUDFImpl.json).

<a id="op-12806a8147b4ece5386c451b"></a>
## HigherOrderUDFImpl

`trait` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl` · datafusion-expr 55.1.0

```rust
trait HigherOrderUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

Source: `src/higher_order_function.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait for implementing user defined higher order functions.

This trait exposes the full API for implementing user defined functions and
can be used to implement any function.

New higher order functions typically implement this trait and are then
wrapped in a [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb) for registration with DataFusion.

See [`array_transform.rs`] for a commented complete implementation

[`array_transform.rs`]: https://github.com/apache/datafusion/blob/main/datafusion/functions-nested/src/array_transform.rs

<a id="op-c48d0ee4c04baea95741a182"></a>
## aliases

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Source: `src/higher_order_function.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns any aliases (alternate names) for this function.

Aliases can be used to invoke the same function using different names.
For example in some databases `now()` and `current_timestamp()` are
aliases for the same function. This behavior can be obtained by
returning `current_timestamp` as an alias for the `now` function.

Note: `aliases` should only include names other than [`Self::name`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-1fcda9c26d1faeb16fa061e2).
Defaults to `[]` (no aliases)

<a id="op-110c2da1e7ea5e0ca28c7ae1"></a>
## clear_null_values

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::clear_null_values` · datafusion-expr 55.1.0

```rust
fn clear_null_values(&self) -> bool
```

Source: `src/higher_order_function.rs:794`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether List or LargeList arguments should have it's non-empty null
sublists cleaned with [remove_list_null_values] before invoking this function

The default implementation always returns true and should only be implemented
if you want to handle non-empty null sublists yourself

[remove_list_null_values]: datafusion_common::utils::remove_list_null_values

<a id="op-f20fe249c57b53787b057f56"></a>
## coerce_value_types

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::coerce_value_types` · datafusion-expr 55.1.0

```rust
fn coerce_value_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Source: `src/higher_order_function.rs:871`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce value arguments of a function call to types that the function can evaluate.
Note that if you need to coerce values based on the output type of lambdas, you
must use [HigherOrderUDFImpl::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea), as this function is used before
the output type of lambdas are known

See the [type coercion module](crate::type_coercion)
documentation for more details on type coercion

For example, if your function requires a contiguous list argument, but the user calls
it like `my_func(c, v -> v+2)` (i.e. with `c` as a ListView), coerce_types can return `[DataType::List(..)]`
to ensure the argument is converted to a List

# Parameters
* `arg_types`: The argument types of the value arguments of this function, excluding lambdas

# Return value
A Vec the same length as `arg_types`. DataFusion will `CAST` the function call
arguments to these specific types.

<a id="op-7b2661f5491cf7d850de20ea"></a>
## coerce_values_for_lambdas

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::coerce_values_for_lambdas` · datafusion-expr 55.1.0

```rust
fn coerce_values_for_lambdas(&self, _fields: &[ValueOrLambda<DataType, DataType>]) -> Result<Option<Vec<DataType>>>
```

Source: `src/higher_order_function.rs:750`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce value arguments of a function call to types that the function can evaluate also taking into
account the *output type of it's lambdas*. This differs from [HigherOrderUDFImpl::coerce_value_types](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-f20fe249c57b53787b057f56)
that only has access to the type of it's value arguments because it's called before the output type
of lambdas are known.

See the [type coercion module](crate::type_coercion)
documentation for more details on type coercion

# Parameters
* `fields`: The argument types of the value arguments of this function, or the output type of lambdas

# Return value
If `Some`, contains a Vec with the same number of [ValueOrLambda::Value](../operations/datafusion_expr.higher_order_function.ValueOrLambda.md#op-a231926a173790a29ccf9c5b) in `fields`.
DataFusion will `CAST` the function call arguments to these specific types. If `None`, no
coercion will be applied beyond the one defined by the function signature.

For example, a flexible array_reduce implementation (see [Self::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484) docs), when working
with the expression below, may want to coerce it's initial value argument, the *integer* `0`,
to match the output of it's merge function, which is a *float*:

`array_reduce([1.2, 2.1], 0, (acc, v) -> acc + v + 1.5, v -> v > 2.0)`

<a id="op-b01e2b4715f4ee4ab52c839d"></a>
## conditional_arguments

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::conditional_arguments` · datafusion-expr 55.1.0

```rust
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
```

Source: `src/higher_order_function.rs:842`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determines which of the arguments passed to *this higher-order function*
are evaluated eagerly and which may be evaluated lazily. Note that this
does *not* applies to the arguments that *lambda functions* pass to it's
body expression

If this function returns `None`, all arguments are eagerly evaluated.
Returning `None` is a micro optimization that saves a needless `Vec`
allocation.

If the function returns `Some`, returns (`eager`, `lazy`) where `eager`
are the arguments that are always evaluated, and `lazy` are the
arguments that may be evaluated lazily (i.e. may not be evaluated at all
in some cases).

Implementations must ensure that the two returned `Vec`s are disjunct,
and that each argument from `args` is present in one the two `Vec`s.

When overriding this function, [HigherOrderUDFImpl::short_circuits](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-0dbca17605dd5ba7162def50) must
be overridden to return `true`.

<a id="op-5dbcc21442d085ac4d50e7f7"></a>
## documentation

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Source: `src/higher_order_function.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this function.

Documentation can be accessed programmatically as well as generating
publicly facing documentation.

<a id="op-de09d4c2a3079597bff655ff"></a>
## invoke_with_args

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::invoke_with_args` · datafusion-expr 55.1.0

```rust
fn invoke_with_args(&self, args: HigherOrderFunctionArgs) -> Result<ColumnarValue>
```

Source: `src/higher_order_function.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function returning the appropriate result.

# Performance

For the best performance, the implementations should handle the common case
when one or more of their arguments are constant values (aka
[`ColumnarValue::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d)).

[`ColumnarValue::values_to_arrays`] can be used to convert the arguments
to arrays, which will likely be simpler code, but be slower.

Unresolved upstream links (retained, not inferred): ``ColumnarValue::values_to_arrays``.

<a id="op-8a89b1f3117f6cf763ede484"></a>
## lambda_parameters

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::lambda_parameters` · datafusion-expr 55.1.0

```rust
fn lambda_parameters(&self, step: usize, fields: &[ValueOrLambda<FieldRef, Option<FieldRef>>]) -> Result<LambdaParametersProgress>
```

Source: `src/higher_order_function.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the field of all the parameters supported by the lambdas in `fields`.
If a lambda support multiple parameters, all should be returned, regardless of
whether they are used or not on a particular invocation

Tip: If you have a [`HigherOrderFunction`] invocation, you can call the helper
[`HigherOrderFunction::lambda_parameters`] instead of this method directly

The name of the returned fields are ignored.

This function is repeatedelly called until [LambdaParametersProgress::Complete](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md#op-56b7f43783579d5d25c47969) is returned, with
`step` increased by one at each invocation, starting at 0.

For functions which all lambda parameters depend only on the field of it's value arguments,
this can return [LambdaParametersProgress::Complete](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md#op-56b7f43783579d5d25c47969) at step 0. Taking as an example a strict
array_reduce with the signature `(arr: [V], initial_value: I, (I, V) -> I, (I) -> O) -> O`, which
requires it's initial value to be the exact same type of it's merge output, which is also the
parameter of it's finish lambda, the expression

`array_reduce([1.2, 2.1], 0.0, (acc, v) -> acc + v + 1.5, v -> v > 5.1)`

 would result in this function being called as the following:

```ignore
let lambda_parameters = array_reduce.lambda_parameters(
    0,
    &[
        // the Field of the literal `[1.2, 2.1]`, the array being reduced
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::new_list(DataType::Float32, true), true))),
        // the Field of the literal `0.0`, the initial value
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::Float32, true))),
        // the Field of the output of the merge lambda, which is unknown at this point because it depends
        // on the return of this call
        ValueOrLambda::Lambda(None),
        // the Field of the output of the finish lambda, unknown for the same reason as above
        ValueOrLambda::Lambda(None),
])?;

assert_eq!(
     lambda_parameters,
     LambdaParametersProgress::Complete(vec![
        // the finish lambda supported parameters, regardless of how many are actually used
        vec![
            // the accumulator which is the field of the initial value
            Arc::new(Field::new("ignored_name", DataType::Float32, true)),
            // the array values being reduced
            Arc::new(Field::new("", DataType::Float32, true)),
        ],
        // the merge lambda supported parameters
        vec![
            // the reduced value which is the field of the initial value
            Arc::new(Field::new("ignored_name", DataType::Float32, true)),
        ],
     ])
);
```

For functions which lambda parameters depends on the output of other lambdas, or on their own lambda,
this can return [LambdaParametersProgress::Partial](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md#op-f0b9f54af5c1839b89e16765) until all dependencies are met. Note that for
lambda with cyclic dependencies, you likely want to use [HigherOrderUDFImpl::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea) too.
Take as an example a flexible array_reduce with the signature `(arr: [V], initial_value: I, (ACC, V) -> ACC, (ACC) -> O) -> O`.
It has a cyclic dependency in the merge lambda, and a dependency of the finish lambda in the merge lambda,
and only requires the initial value to be *coercible* to the output of the merge lambda, which is defined by
it's [HigherOrderUDFImpl::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea) implementation. The expression

`array_reduce([1.2, 2.1], 0, (acc, v) -> acc + v + 1.5, v -> v > 5.1)`

would result in this function being called as the following:

```ignore
let lambda_parameters = array_reduce.lambda_parameters(
    0,
    &[
        // the Field of the literal `[1.2, 2.1]`, the array being reduced
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::new_list(DataType::Float32, true), true))),
        // the Field of the literal `0`, the initial value
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::Int32, true))),
        // the Field of the output of the merge lambda, which is unknown at this point because it depends on
        // the return this call
        ValueOrLambda::Lambda(None),
        // the Field of the output of the finish lambda, unknown for the same reason as above
        ValueOrLambda::Lambda(None),
])?;

assert_eq!(
     lambda_parameters,
     LambdaParametersProgress::Partial(vec![
        // the finish lambda supported parameters, regardless of how many are actually used
        Some(vec![
            // at step 0, use the field of the initial value
            Arc::new(Field::new("ignored_name", DataType::Int32, true)),
            // the array values being reduced
            Arc::new(Field::new("", DataType::Float32, true)),
        ]),
        // the merge lambda supported parameters, unknown at this point due to dependency on the merge output
        None,
     ])
);

let lambda_parameters = array_reduce.lambda_parameters(
    1,
    &[
        // the Field of the literal `[1.2, 2.1]`, the array being reduced
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::new_list(DataType::Float32, true), true))),
        // the Field of the literal `0`, the initial value
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::Int32, true))),
        // the Field of the output of the merge lambda, which could be inferred to be a Float32 based on the
        // returned values of the previous step
        ValueOrLambda::Value(Arc::new(Field::new("", DataType::Float32, true))),
        // the Field of the output of the finish lambda, which is unknown at this point because it depends
        // on the return of this call
        ValueOrLambda::Lambda(None),
])?;

assert_eq!(
     lambda_parameters,
     LambdaParametersProgress::Complete(vec![
        // the finish lambda supported parameters, regardless of how many are actually used
        vec![
            // the finish lambda own output now used as it's accumulator
            Arc::new(Field::new("ignored_name", DataType::Float32, true)),
            // the array values being reduced
            Arc::new(Field::new("", DataType::Float32, true)),
        ],
        // the merge lambda supported parameters, which is the output of the merge lambda,
        vec![
            // the output of the merge lambda
            Arc::new(Field::new("", DataType::Float32, true)),
        ],
     ])
);

let coerce_to = array_reduce.coerce_values_for_lambdas(&[
    // the literal `[1.2, 2.1]` data type, the array being reduced
    ValueOrLambda::Value(DataType::new_list(DataType::Float32, true)),
    // the literal `0` data type, the initial value
    ValueOrLambda::Value(DataType::Int32),
    // the output data type of the merge lambda
    ValueOrLambda::Lambda(DataType::Float32),
    // the output data type of the finish lambda
    ValueOrLambda::Lambda(DataType::Boolean),
])?;

assert_eq!(
    coerce_to,
    Some(vec![
        // return the same type for the array being reduced
        DataType::new_list(DataType::Float32, true),
        // coerce the initial value to the output of the merge lambda
        DataType::Float32,
    ])
);

```

Note this may also be called at step 0 with all lambda outputs already set, and in that case,
[LambdaParametersProgress::Complete](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md#op-56b7f43783579d5d25c47969) must be returned

The implementation can assume that some other part of the code has coerced
the actual argument types to match [`Self::signature`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-e41232c846652d057104e4de), except the coercion defined by
[Self::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea).

[`HigherOrderFunction`]: crate::expr::HigherOrderFunction
[`HigherOrderFunction::lambda_parameters`]: crate::expr::HigherOrderFunction::lambda_parameters

<a id="op-1fcda9c26d1faeb16fa061e2"></a>
## name

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/higher_order_function.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name

<a id="op-4de68ce01e1d3df53fc7d4fb"></a>
## return_field_from_args

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::return_field_from_args` · datafusion-expr 55.1.0

```rust
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs<'_>) -> Result<FieldRef>
```

Source: `src/higher_order_function.rs:781`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

What type will be returned by this function, given the arguments?

The implementation can assume that some other part of the code has coerced
the actual argument types to match [`Self::signature`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-e41232c846652d057104e4de), including the coercion
defined by [Self::coerce_values_for_lambdas](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea).

# Example creating `Field`

Note the name of the `Field` is ignored, except for structured types such as
`DataType::Struct`.

```rust
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, FieldRef};
# use datafusion_common::Result;
# use datafusion_expr::HigherOrderReturnFieldArgs;
# struct Example{}
# impl Example {
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs) -> Result<FieldRef> {
    let field = Arc::new(Field::new("ignored_name", DataType::Int32, true));
    Ok(field)
}
# }
```

<a id="op-ee2aa3f3caca6ec74c82874f"></a>
## schema_name

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self, args: &[Expr]) -> Result<String>
```

Source: `src/higher_order_function.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the name of the column this expression would create

See [`Expr::schema_name`](../operations/datafusion_expr.expr.Expr.md#op-82ef8179c0d4e6c2ae471137) for details

<a id="op-0dbca17605dd5ba7162def50"></a>
## short_circuits

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::short_circuits` · datafusion-expr 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Source: `src/higher_order_function.rs:819`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if some of this `exprs` subexpressions may not be evaluated
and thus any side effects (like divide by zero) may not be encountered.

Setting this to true prevents certain optimizations such as common
subexpression elimination

When overriding this function to return `true`, [HigherOrderUDFImpl::conditional_arguments](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-b01e2b4715f4ee4ab52c839d) can also be
overridden to report more accurately which arguments are eagerly evaluated and which ones
lazily.

<a id="op-e41232c846652d057104e4de"></a>
## signature

`function` · `datafusion_expr::higher_order_function::HigherOrderUDFImpl::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &HigherOrderSignature
```

Source: `src/higher_order_function.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a [`HigherOrderSignature`](../operations/datafusion_expr.higher_order_function.HigherOrderSignature.md#op-57014c54db47f90a3230a79a) describing the argument types for which this
function has an implementation, and the function's [`Volatility`].

See [`HigherOrderSignature`](../operations/datafusion_expr.higher_order_function.HigherOrderSignature.md#op-57014c54db47f90a3230a79a) for more details on argument type handling
and [`Self::return_field_from_args`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-4de68ce01e1d3df53fc7d4fb) for computing the return type.

[`Volatility`]: datafusion_expr_common::signature::Volatility
