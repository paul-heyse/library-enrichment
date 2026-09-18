# `datafusion_expr::udf::ScalarUDFImpl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udf.ScalarUDFImpl.json).

<a id="op-7caee2ff206563b536e983e0"></a>
## ScalarUDFImpl

`trait` · `datafusion_expr::udf::ScalarUDFImpl` · datafusion-expr 55.1.0

```rust
trait ScalarUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

Source: `src/udf.rs:537`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait for implementing user defined scalar functions.

This trait exposes the full API for implementing user defined functions and
can be used to implement any function.

See [`advanced_udf.rs`] for a full example with complete implementation and
[`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0) for other available options.

[`advanced_udf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udf.rs

# Basic Example
```
# use std::any::Any;
# use std::sync::LazyLock;
# use arrow::datatypes::DataType;
# use datafusion_common::{DataFusionError, plan_err, Result};
# use datafusion_expr::{col, ColumnarValue, Documentation, ScalarFunctionArgs, Signature, Volatility};
# use datafusion_expr::{ScalarUDFImpl, ScalarUDF};
# use datafusion_expr::scalar_doc_sections::DOC_SECTION_MATH;
/// This struct for a simple UDF that adds one to an int32
#[derive(Debug, PartialEq, Eq, Hash)]
struct AddOne {
  signature: Signature,
}

impl AddOne {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Int32], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
        Documentation::builder(DOC_SECTION_MATH, "Add one to an int32", "add_one(2)")
            .with_argument("arg1", "The int32 number to add one to")
            .build()
    });

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the ScalarUDFImpl trait for AddOne
impl ScalarUDFImpl for AddOne {
   fn name(&self) -> &str { "add_one" }
   fn signature(&self) -> &Signature { &self.signature }
   fn return_type(&self, args: &[DataType]) -> Result<DataType> {
     if !matches!(args.get(0), Some(&DataType::Int32)) {
       return plan_err!("add_one only accepts Int32 arguments");
     }
     Ok(DataType::Int32)
   }
   // The actual implementation would add one to the argument
   fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        unimplemented!()
   }
   fn documentation(&self) -> Option<&Documentation> {
        Some(get_doc())
    }
}

// Create a new ScalarUDF from the implementation
let add_one = ScalarUDF::from(AddOne::new());

// Call the function `add_one(col)`
let expr = add_one.call(vec![col("a")]);
```

<a id="op-7271600ea359275a77293c23"></a>
## aliases

`function` · `datafusion_expr::udf::ScalarUDFImpl::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Source: `src/udf.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns any aliases (alternate names) for this function.

Aliases can be used to invoke the same function using different names.
For example in some databases `now()` and `current_timestamp()` are
aliases for the same function. This behavior can be obtained by
returning `current_timestamp` as an alias for the `now` function.

Note: `aliases` should only include names other than [`Self::name`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-1b6686681755a5ff0c22a2d8).
Defaults to `[]` (no aliases)

<a id="op-2418bef30f1b8de36e5ac4ec"></a>
## coerce_types

`function` · `datafusion_expr::udf::ScalarUDFImpl::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Source: `src/udf.rs:1022`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce arguments of a function call to types that the function can evaluate.

This function is only called if [`ScalarUDFImpl::signature`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-0b3a009ba3a69eab52444d72) returns
[`crate::TypeSignature::UserDefined`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-9a54792c384e19dbdaacb169). Most UDFs should return one of
the other variants of [`TypeSignature`] which handle common cases.

See the [type coercion module](crate::type_coercion)
documentation for more details on type coercion

[`TypeSignature`]: crate::TypeSignature

For example, if your function requires a floating point arguments, but the user calls
it like `my_func(1::int)` (i.e. with `1` as an integer), coerce_types can return `[DataType::Float64]`
to ensure the argument is converted to `1::double`

# Parameters
* `arg_types`: The argument types of the arguments  this function with

# Return value
A Vec the same length as `arg_types`. DataFusion will `CAST` the function call
arguments to these specific types.

<a id="op-609aba8577614f2ef39b411c"></a>
## conditional_arguments

`function` · `datafusion_expr::udf::ScalarUDFImpl::conditional_arguments` · datafusion-expr 55.1.0

```rust
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
```

Source: `src/udf.rs:905`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determines which of the arguments passed to this function are evaluated eagerly
and which may be evaluated lazily.

If this function returns `None`, all arguments are eagerly evaluated.
Returning `None` is a micro optimization that saves a needless `Vec`
allocation.

If the function returns `Some`, returns (`eager`, `lazy`) where `eager`
are the arguments that are always evaluated, and `lazy` are the
arguments that may be evaluated lazily (i.e. may not be evaluated at all
in some cases).

Implementations must ensure that the two returned `Vec`s are disjunct,
and that each argument from `args` is present in one the two `Vec`s.

When overriding this function, [ScalarUDFImpl::short_circuits](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-d442484f46ac4203797da7d0) must
be overridden to return `true`.

<a id="op-11c9162f06a7f8f322590763"></a>
## display_name

`function` · `datafusion_expr::udf::ScalarUDFImpl::display_name` · datafusion-expr 55.1.0

```rust
fn display_name(&self, args: &[Expr]) -> Result<String>
```

Source: `src/udf.rs:564`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the user-defined display name of function, given the arguments

This can be used to customize the output column name generated by this
function.

Defaults to `name(args[0], args[1], ...)`

<a id="op-4cf61edb626d125aba9ebc72"></a>
## documentation

`function` · `datafusion_expr::udf::ScalarUDFImpl::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Source: `src/udf.rs:1049`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this Scalar UDF.

Documentation can be accessed programmatically as well as generating
publicly facing documentation.

<a id="op-1774e326b604d7d35c5cceb3"></a>
## evaluate_bounds

`function` · `datafusion_expr::udf::ScalarUDFImpl::evaluate_bounds` · datafusion-expr 55.1.0

```rust
fn evaluate_bounds(&self, _input: &[&Interval]) -> Result<Interval>
```

Source: `src/udf.rs:927`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Computes the output [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) for a [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0), given the input
intervals.

# Parameters

* `children` are the intervals for the children (inputs) of this function.

# Example

If the function is `ABS(a)`, and the input interval is `a: [-3, 2]`,
then the output interval would be `[0, 3]`.

<a id="op-412be3cb5d7fb7f10c3f7a58"></a>
## invoke_with_args

`function` · `datafusion_expr::udf::ScalarUDFImpl::invoke_with_args` · datafusion-expr 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Source: `src/udf.rs:733`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function returning the appropriate result.

# Performance

For the best performance, the implementations should handle the common case
when one or more of their arguments are constant values (aka
[`ColumnarValue::Scalar`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d6c26dcb617515197f47859d)).

[`ColumnarValue::values_to_arrays`] can be used to convert the arguments
to arrays, which will likely be simpler code, but be slower.

Unresolved upstream links (retained, not inferred): ``ColumnarValue::values_to_arrays``.

<a id="op-94910b8050ec4aedc3543b75"></a>
## is_nullable

`function` · `datafusion_expr::udf::ScalarUDFImpl::is_nullable` · datafusion-expr 55.1.0

```rust
fn is_nullable(&self, _args: &[Expr], _schema: &dyn ExprSchema) -> bool
```

Source: `src/udf.rs:705`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cabf9d40d1fe8712d7e55eef"></a>
## is_strict

`function` · `datafusion_expr::udf::ScalarUDFImpl::is_strict` · datafusion-expr 55.1.0

```rust
fn is_strict(&self) -> bool
```

Source: `src/udf.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if this function always returns NULL when any argument is
NULL.

Strict functions are NULL-propagating: if any argument evaluates to
NULL, the function result is guaranteed to be NULL. Optimizer rules can
use this property when reasoning about expression nullability and
null-rejecting filters.

Defaults to `false` because user-defined functions may choose to accept
NULL inputs and produce non-NULL results.

<a id="op-1b6686681755a5ff0c22a2d8"></a>
## name

`function` · `datafusion_expr::udf::ScalarUDFImpl::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/udf.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name

<a id="op-97f7c8a03d475292252eb927"></a>
## output_ordering

`function` · `datafusion_expr::udf::ScalarUDFImpl::output_ordering` · datafusion-expr 55.1.0

```rust
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
```

Source: `src/udf.rs:965`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Calculates the [`SortProperties`](../operations/datafusion_expr_common.sort_properties.SortProperties.md#op-a357ca132b9df8290c59f5bf) of this function based on its children's properties.

<a id="op-6e36a0acc77db21760b4a45e"></a>
## placement

`function` · `datafusion_expr::udf::ScalarUDFImpl::placement` · datafusion-expr 55.1.0

```rust
fn placement(&self, _args: &[ExpressionPlacement]) -> ExpressionPlacement
```

Source: `src/udf.rs:1063`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns placement information for this function.

This is used by optimizers to make decisions about expression placement,
such as whether to push expressions down through projections.

The default implementation returns [`ExpressionPlacement::KeepInPlace`](../operations/datafusion_expr_common.placement.ExpressionPlacement.md#op-600eef0885718f7165bfa8f5),
meaning the expression should be kept where it is in the plan.

Override this method to indicate that the function can be pushed down
closer to the data source.

<a id="op-0f3b7cc7d7d1203c14fd2c9f"></a>
## preimage

`function` · `datafusion_expr::udf::ScalarUDFImpl::preimage` · datafusion-expr 55.1.0

```rust
fn preimage(&self, _args: &[Expr], _lit_expr: &Expr, _info: &SimplifyContext) -> Result<PreimageResult>
```

Source: `src/udf.rs:866`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a single contiguous preimage for this function and the specified
scalar expression, if any.

Currently only applies to `=, !=, >, >=, <, <=, is distinct from, is not distinct from` predicates
# Return Value

Implementations should return a half-open interval: inclusive lower
bound and exclusive upper bound. This is slightly different from normal
[`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) semantics where the upper bound is closed (inclusive).
Typically this means the upper endpoint must be adjusted to the next
value not included in the preimage. See the Half-Open Intervals section
below for more details.

# Background

Inspired by the [ClickHouse Paper], a "preimage rewrite" transforms a
predicate containing a function call into a predicate containing an
equivalent set of input literal (constant) values. The resulting
predicate can often be further optimized by other rewrites (see
Examples).

From the paper:

> some functions can compute the preimage of a given function result.
> This is used to replace comparisons of constants with function calls
> on the key columns by comparing the key column value with the preimage.
> For example, `toYear(k) = 2024` can be replaced by
> `k >= 2024-01-01 && k < 2025-01-01`

For example, given an expression like
```sql
date_part('YEAR', k) = 2024
```

The interval `[2024-01-01, 2025-12-31`]` contains all possible input
values (preimage values) for which the function `date_part(YEAR, k)`
produces the output value `2024` (image value). Returning the interval
(note upper bound adjusted up) `[2024-01-01, 2025-01-01]` the expression
can be rewritten to

```sql
k >= '2024-01-01' AND k < '2025-01-01'
```

which is a simpler and a more canonical form, making it easier for other
optimizer passes to recognize and apply further transformations.

# Examples

Case 1:

Original:
```sql
date_part('YEAR', k) = 2024 AND k >= '2024-06-01'
```

After preimage rewrite:
```sql
k >= '2024-01-01' AND k < '2025-01-01' AND k >= '2024-06-01'
```

Since this form is much simpler, the optimizer can combine and simplify
sub-expressions further into:
```sql
k >= '2024-06-01' AND k < '2025-01-01'
```

Case 2:

For min/max pruning, simpler predicates such as:
```sql
k >= '2024-01-01' AND k < '2025-01-01'
```
are much easier for the pruner to reason about. See [PruningPredicate]
for the backgrounds of predicate pruning.

The trade-off with the preimage rewrite is that evaluating the rewritten
form might be slightly more expensive than evaluating the original
expression. In practice, this cost is usually outweighed by the more
aggressive optimization opportunities it enables.

# Half-Open Intervals

The preimage API uses half-open intervals, which makes the rewrite
easier to implement by avoiding calculations to adjust the upper bound.
For example, if a function returns its input unchanged and the desired
output is the single value `5`, a closed interval could be represented
as `[5, 5]`, but then the rewrite would require adjusting the upper
bound to `6` to create a proper range predicate. With a half-open
interval, the same range is represented as `[5, 6)`, which already
forms a valid predicate.

[PruningPredicate]: https://docs.rs/datafusion/latest/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html
[ClickHouse Paper]:  https://www.vldb.org/pvldb/vol17/p3731-schulze.pdf
[image]: https://en.wikipedia.org/wiki/Image_(mathematics)#Image_of_an_element
[preimage]: https://en.wikipedia.org/wiki/Image_(mathematics)#Inverse_image

<a id="op-ddd814ea6d24274264defc02"></a>
## preserves_lex_ordering

`function` · `datafusion_expr::udf::ScalarUDFImpl::preserves_lex_ordering` · datafusion-expr 55.1.0

```rust
fn preserves_lex_ordering(&self, _inputs: &[ExprProperties]) -> Result<bool>
```

Source: `src/udf.rs:989`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the function preserves lexicographical ordering based on
the input ordering.

See [`ExprProperties::preserves_lex_ordering`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-2ba98f8a96c838499ec6bd27) for more details

<a id="op-35b3ffb8113861b201fdf886"></a>
## propagate_constraints

`function` · `datafusion_expr::udf::ScalarUDFImpl::propagate_constraints` · datafusion-expr 55.1.0

```rust
fn propagate_constraints(&self, _interval: &Interval, _inputs: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Source: `src/udf.rs:956`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Updates bounds for child expressions, given a known [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e)s for this
function.

This function is used to propagate constraints down through an
expression tree.

# Parameters

* `interval` is the currently known interval for this function.
* `inputs` are the current intervals for the inputs (children) of this function.

# Returns

A `Vec` of new intervals for the children, in order.

If constraint propagation reveals an infeasibility for any child, returns
[`None`]. If none of the children intervals change as a result of
propagation, may return an empty vector instead of cloning `children`.
This is the default (and conservative) return value.

# Example

If the function is `ABS(a)`, the current `interval` is `[4, 5]` and the
input `a` is given as `[-7, 3]`, then propagation would return `[-5, 3]`.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-efa24f895df9cedbd3cf8c2a"></a>
## return_field_from_args

`function` · `datafusion_expr::udf::ScalarUDFImpl::return_field_from_args` · datafusion-expr 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Source: `src/udf.rs:690`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

What type will be returned by this function, given the arguments?

By default, this function calls [`Self::return_type`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-656507bbb7a195e311356b01) with the
types of each argument.

# Notes

For the majority of UDFs, implementing [`Self::return_type`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-656507bbb7a195e311356b01) is sufficient,
as the result type is typically a deterministic function of the input types
(e.g., `sqrt(f32)` consistently yields `f32`). Implementing this method directly
is generally unnecessary unless the return type depends on runtime values.

This function can be used for more advanced cases such as:

1. specifying nullability
2. return types based on the **values** of the arguments (rather than
   their **types**.

# Example creating `Field`

Note the name of the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) is ignored, except for structured types such as
`DataType::Struct`.

```rust
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, FieldRef};
# use datafusion_common::Result;
# use datafusion_expr::ReturnFieldArgs;
# struct Example{}
# impl Example {
fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
    // report output is only nullable if any one of the arguments are nullable
    let nullable = args.arg_fields.iter().any(|f| f.is_nullable());
    let field = Arc::new(Field::new("ignored_name", DataType::Int32, nullable));
    Ok(field)
}
# }
```

# Output Type based on Values

For example, the following two function calls get the same argument
types (something and a `Utf8` string) but return different types based
on the value of the second argument:

* `arrow_cast(x, 'Int16')` --> `Int16`
* `arrow_cast(x, 'Float32')` --> `Float32`

# Requirements

This function **must** consistently return the same type for the same
logical input even if the input is simplified (e.g. it must return the same
value for `('foo' | 'bar')` as it does for ('foobar').

<a id="op-656507bbb7a195e311356b01"></a>
## return_type

`function` · `datafusion_expr::udf::ScalarUDFImpl::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Source: `src/udf.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

[`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) returned by this function, given the types of the
arguments.

# Arguments

`arg_types` Data types of the arguments. The implementation of
`return_type` can assume that some other part of the code has coerced
the actual argument types to match [`Self::signature`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-0b3a009ba3a69eab52444d72).

# Notes

If you provide an implementation for [`Self::return_field_from_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-efa24f895df9cedbd3cf8c2a),
DataFusion will not call `return_type` (this function). While it is
valid to put [`unimplemented!()`] or [`unreachable!()`], it is
recommended to return [`DataFusionError::Internal`] instead, which
reduces the severity of symptoms if bugs occur (an error rather than a
panic).

[`DataFusionError::Internal`]: datafusion_common::DataFusionError::Internal

Unresolved upstream links (retained, not inferred): ``unimplemented!()``, ``unreachable!()``.

<a id="op-2594946d62d840b6b88d094f"></a>
## schema_name

`function` · `datafusion_expr::udf::ScalarUDFImpl::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self, args: &[Expr]) -> Result<String>
```

Source: `src/udf.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the name of the column this expression would create

See [`Expr::schema_name`](../operations/datafusion_expr.expr.Expr.md#op-82ef8179c0d4e6c2ae471137) for details

<a id="op-d442484f46ac4203797da7d0"></a>
## short_circuits

`function` · `datafusion_expr::udf::ScalarUDFImpl::short_circuits` · datafusion-expr 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Source: `src/udf.rs:884`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if some of this `exprs` subexpressions may not be evaluated
and thus any side effects (like divide by zero) may not be encountered.

Setting this to true prevents certain optimizations such as common
subexpression elimination

When overriding this function to return `true`, [ScalarUDFImpl::conditional_arguments](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-609aba8577614f2ef39b411c) can also be
overridden to report more accurately which arguments are eagerly evaluated and which ones
lazily.

<a id="op-0b3a009ba3a69eab52444d72"></a>
## signature

`function` · `datafusion_expr::udf::ScalarUDFImpl::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Source: `src/udf.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a [`Signature`](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) describing the argument types for which this
function has an implementation, and the function's [`Volatility`].

See [`Signature`](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for more details on argument type handling
and [`Self::return_type`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-656507bbb7a195e311356b01) for computing the return type.

[`Volatility`]: datafusion_expr_common::signature::Volatility

<a id="op-5b09f606d091d093ed8a0aec"></a>
## simplify

`function` · `datafusion_expr::udf::ScalarUDFImpl::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Source: `src/udf.rs:762`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optionally apply per-UDF simplification / rewrite rules.

This can be used to apply function specific simplification rules during
optimization (e.g. `arrow_cast` --> `Expr::Cast`). The default
implementation does nothing.

Note that DataFusion handles simplifying arguments and  "constant
folding" (replacing a function call with constant arguments such as
`my_add(1,2) --> 3` ). Thus, there is no need to implement such
optimizations manually for specific UDFs.

# Arguments
* `args`: The arguments of the function
* `info`: The necessary information for simplification

# Returns
[`ExprSimplifyResult`](../operations/datafusion_expr.simplify.ExprSimplifyResult.md#op-e99cc91e51c5aaf2a6e6620b) indicating the result of the simplification NOTE
if the function cannot be simplified, the arguments *MUST* be returned
unmodified

# Notes

The returned expression must have the same schema as the original
expression, including both the data type and nullability. For example,
if the original expression is nullable, the returned expression must
also be nullable, otherwise it may lead to schema verification errors
later in query planning.

<a id="op-aa3db0a55467ecf3dc3cfcab"></a>
## strictly_order_preserving

`function` · `datafusion_expr::udf::ScalarUDFImpl::strictly_order_preserving` · datafusion-expr 55.1.0

```rust
fn strictly_order_preserving(&self, _inputs: &[ExprProperties]) -> Result<bool>
```

Source: `src/udf.rs:997`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the function is strictly order-preserving with respect
to its `Ordered` inputs, i.e. `a.cmp(b) == f(a).cmp(f(b))`.

See [`ExprProperties::strictly_order_preserving`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-2ec9acd60d62113a73cd3a5f) for more details

<a id="op-046bec345ba5869e7f1b1c29"></a>
## struct_field_mapping

`function` · `datafusion_expr::udf::ScalarUDFImpl::struct_field_mapping` · datafusion-expr 55.1.0

```rust
fn struct_field_mapping(&self, _literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>
```

Source: `src/udf.rs:1038`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

For struct-producing functions, return how output fields map to input
arguments. This enables the optimizer to propagate orderings through
struct projections.

`literal_args[i]` is `Some(value)` if argument `i` is a known literal,
allowing extraction of field names from arguments like
`named_struct('field_name', value, ...)`.

For example, `named_struct('a', col1, 'b', col2)` would return a
mapping indicating that output field `'a'` (accessed via
`get_field(output, 'a')`) corresponds to input argument `col1` at
index 1, and field `'b'` corresponds to `col2` at index 3.

<a id="op-f0dc74cfc8974de0bebe2f8a"></a>
## with_updated_config

`function` · `datafusion_expr::udf::ScalarUDFImpl::with_updated_config` · datafusion-expr 55.1.0

```rust
fn with_updated_config(&self, _config: &ConfigOptions) -> Option<ScalarUDF>
```

Source: `src/udf.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new instance of this function with updated configuration.

This method is called when configuration options change at runtime
(e.g., via `SET` statements) to allow functions that depend on
configuration to update themselves accordingly.

Note the current [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) are also passed to [`Self::invoke_with_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-412be3cb5d7fb7f10c3f7a58) so
this API is not needed for functions where the values may
depend on the current options.

This API is useful for functions where the return
**type** depends on the configuration options, such as the `now()` function
which depends on the current timezone.

# Arguments

* `config` - The updated configuration options

# Returns

* `Some(ScalarUDF)` - A new instance of this function configured with the new settings
* `None` - If this function does not change with new configuration settings (the default)
