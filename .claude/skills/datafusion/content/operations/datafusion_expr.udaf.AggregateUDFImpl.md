# `datafusion_expr::udaf::AggregateUDFImpl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.AggregateUDFImpl.json).

<a id="op-9f175a3c2e0fb1ef9e53572c"></a>
## AggregateUDFImpl

`trait` · `datafusion_expr::udaf::AggregateUDFImpl` · datafusion-expr 55.1.0

```rust
trait AggregateUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

Source: `src/udaf.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait for implementing [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379).

This trait exposes the full API for implementing user defined aggregate functions and
can be used to implement any function.

See [`advanced_udaf.rs`] for a full example with complete implementation and
[`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379) for other available options.

[`advanced_udaf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udaf.rs

# Basic Example
```
# use std::any::Any;
# use std::sync::{Arc, LazyLock};
# use arrow::datatypes::{DataType, FieldRef};
# use datafusion_common::{DataFusionError, plan_err, Result};
# use datafusion_expr::{col, ColumnarValue, Signature, Volatility, Expr, Documentation};
# use datafusion_expr::{AggregateUDFImpl, AggregateUDF, Accumulator, function::{AccumulatorArgs, StateFieldsArgs}};
# use datafusion_expr::window_doc_sections::DOC_SECTION_AGGREGATE;
# use arrow::datatypes::Schema;
# use arrow::datatypes::Field;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GeoMeanUdf {
  signature: Signature,
}

impl GeoMeanUdf {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Float64], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
        Documentation::builder(DOC_SECTION_AGGREGATE, "calculates a geometric mean", "geo_mean(2.0)")
            .with_argument("arg1", "The Float64 number for the geometric mean")
            .build()
    });

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the AggregateUDFImpl trait for GeoMeanUdf
impl AggregateUDFImpl for GeoMeanUdf {
   fn name(&self) -> &str { "geo_mean" }
   fn signature(&self) -> &Signature { &self.signature }
   fn return_type(&self, args: &[DataType]) -> Result<DataType> {
     if !matches!(args.get(0), Some(&DataType::Float64)) {
       return plan_err!("geo_mean only accepts Float64 arguments");
     }
     Ok(DataType::Float64)
   }
   // This is the accumulator factory; DataFusion uses it to create new accumulators.
   fn accumulator(&self, _acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> { unimplemented!() }
   fn state_fields(&self, args: StateFieldsArgs) -> Result<Vec<FieldRef>> {
       Ok(vec![
            Arc::new(args.return_field.as_ref().clone().with_name("value")),
            Arc::new(Field::new("ordering", DataType::UInt32, true))
       ])
   }
   fn documentation(&self) -> Option<&Documentation> {
       Some(get_doc())
   }
}

// Create a new AggregateUDF from the implementation
let geometric_mean = AggregateUDF::from(GeoMeanUdf::new());

// Call the function `geo_mean(col)`
let expr = geometric_mean.call(vec![col("a")]);
```

<a id="op-713c637b03a7271bf6533704"></a>
## accumulator

`function` · `datafusion_expr::udaf::AggregateUDFImpl::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Source: `src/udaf.rs:556`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a new [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8) that aggregates values for a specific
group during query execution.

acc_args: [`AccumulatorArgs`](../operations/datafusion_functions_aggregate_common.accumulator.AccumulatorArgs.md#op-0011bde6a704a46a9e4e009c) contains information about how the
aggregate function was called.

<a id="op-94c477324d14d121d39a1a87"></a>
## aliases

`function` · `datafusion_expr::udaf::AggregateUDFImpl::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Source: `src/udaf.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns any aliases (alternate names) for this function.

Note: `aliases` should only include names other than [`Self::name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-7844e1fa23821e46c7cabf21).
Defaults to `[]` (no aliases)

<a id="op-677367464a864c7930d31160"></a>
## coerce_types

`function` · `datafusion_expr::udaf::AggregateUDFImpl::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Source: `src/udaf.rs:798`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce arguments of a function call to types that the function can evaluate.

This function is only called if [`AggregateUDFImpl::signature`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-130a8046da41201cb1a17c63) returns [`crate::TypeSignature::UserDefined`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-9a54792c384e19dbdaacb169). Most
UDAFs should return one of the other variants of `TypeSignature` which handle common
cases

See the [type coercion module](crate::type_coercion)
documentation for more details on type coercion

For example, if your function requires a floating point arguments, but the user calls
it like `my_func(1::int)` (aka with `1` as an integer), coerce_types could return `[DataType::Float64]`
to ensure the argument was cast to `1::double`

# Parameters
* `arg_types`: The argument types of the arguments  this function with

# Return value
A Vec the same length as `arg_types`. DataFusion will `CAST` the function call
arguments to these specific types.

<a id="op-b55923cc32b806eb805e9ffb"></a>
## create_groups_accumulator

`function` · `datafusion_expr::udaf::AggregateUDFImpl::create_groups_accumulator` · datafusion-expr 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Source: `src/udaf.rs:613`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a specialized [`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) that manages state
for all groups.

For maximum performance, a [`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) should be
implemented in addition to [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8).

<a id="op-e676ad202f44dac4ce6cd314"></a>
## create_sliding_accumulator

`function` · `datafusion_expr::udaf::AggregateUDFImpl::create_sliding_accumulator` · datafusion-expr 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Source: `src/udaf.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Sliding accumulator is an alternative accumulator that can be used for
window functions. It has retract method to revert the previous update.

See [retract_batch] for more details.

[retract_batch]: Accumulator::retract_batch

<a id="op-5ae1688167b50fd77316df64"></a>
## default_value

`function` · `datafusion_expr::udaf::AggregateUDFImpl::default_value` · datafusion-expr 55.1.0

```rust
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
```

Source: `src/udaf.rs:828`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns default value of the function given the input is all `null`.

Most of the aggregate function return Null if input is Null,
while `count` returns 0 if input is Null

<a id="op-1bb6dfbde26e11e415def938"></a>
## display_name

`function` · `datafusion_expr::udaf::AggregateUDFImpl::display_name` · datafusion-expr 55.1.0

```rust
fn display_name(&self, params: &AggregateFunctionParams) -> Result<String>
```

Source: `src/udaf.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the user-defined display name of function, given the arguments

This can be used to customize the output column name generated by this
function.

Defaults to `function_name([DISTINCT] column1, column2, ..) [null_treatment] [filter] [order_by [..]]`

<a id="op-bd4ba2258fea832fc9bac999"></a>
## documentation

`function` · `datafusion_expr::udaf::AggregateUDFImpl::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Source: `src/udaf.rs:899`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this Aggregate UDF.

Documentation can be accessed programmatically as well as
generating publicly facing documentation.

<a id="op-fbcc41eac02cc7099e2ddac5"></a>
## groups_accumulator_supported

`function` · `datafusion_expr::udaf::AggregateUDFImpl::groups_accumulator_supported` · datafusion-expr 55.1.0

```rust
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
```

Source: `src/udaf.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If the aggregate expression has a specialized
[`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) implementation. If this returns true,
`[Self::create_groups_accumulator]` will be called.

# Notes

Even if this function returns true, DataFusion will still use
[`Self::accumulator`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-713c637b03a7271bf6533704) for certain queries, such as when this aggregate is
used as a window function or when there no GROUP BY columns in the
query.

<a id="op-7058483a5bb0c7b0f33d0444"></a>
## human_display

`function` · `datafusion_expr::udaf::AggregateUDFImpl::human_display` · datafusion-expr 55.1.0

```rust
fn human_display(&self, params: &AggregateFunctionParams) -> Result<String>
```

Source: `src/udaf.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a human readable expression.

See [`Expr::human_display`](../operations/datafusion_expr.expr.Expr.md#op-3d5b1a93f5851b5885d840ce) for details.

<a id="op-17c961a37d33cf28477ef396"></a>
## is_descending

`function` · `datafusion_expr::udaf::AggregateUDFImpl::is_descending` · datafusion-expr 55.1.0

```rust
fn is_descending(&self) -> Option<bool>
```

Source: `src/udaf.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If this function is max, return true
If the function is min, return false
Otherwise return None (the default)


Note: this is used to use special aggregate implementations in certain conditions

<a id="op-24432183afcb23ae5b355d90"></a>
## is_nullable

`function` · `datafusion_expr::udaf::AggregateUDFImpl::is_nullable` · datafusion-expr 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Source: `src/udaf.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the aggregate function is nullable.

Nullable means that the function could return `null` for any inputs.
For example, aggregate functions like `COUNT` always return a non null value
but others like `MIN` will return `NULL` if there is nullable input.
Note that if the function is declared as *not* nullable, make sure the [`AggregateUDFImpl::default_value`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-5ae1688167b50fd77316df64) is `non-null`

<a id="op-7844e1fa23821e46c7cabf21"></a>
## name

`function` · `datafusion_expr::udaf::AggregateUDFImpl::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/udaf.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name

<a id="op-3479bdd2a46b48de41f6ff9c"></a>
## order_sensitivity

`function` · `datafusion_expr::udaf::AggregateUDFImpl::order_sensitivity` · datafusion-expr 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Source: `src/udaf.rs:659`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Gets the order sensitivity of the UDF. See [`AggregateOrderSensitivity`](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md#op-8b6a26410e0f5b4d59a68b96)
for possible options.

<a id="op-bc99f6392eb81fae9a622a87"></a>
## return_field

`function` · `datafusion_expr::udaf::AggregateUDFImpl::return_field` · datafusion-expr 55.1.0

```rust
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
```

Source: `src/udaf.rs:537`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

What type will be returned by this function, given the arguments?

By default, this function calls [`Self::return_type`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-02d75f6f8103226358991849) with the
types of each argument.

# Notes

Most UDFs should implement [`Self::return_type`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-02d75f6f8103226358991849) and not this
function as the output type for most functions only depends on the types
of their inputs (e.g. `sum(f64)` is always `f64`).

This function can be used for more advanced cases such as:

1. specifying nullability
2. return types based on the **values** of the arguments (rather than
   their **types**.
3. return types based on metadata within the fields of the inputs

<a id="op-02d75f6f8103226358991849"></a>
## return_type

`function` · `datafusion_expr::udaf::AggregateUDFImpl::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Source: `src/udaf.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

What [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) will be returned by this function, given the types of
the arguments

<a id="op-0ec0ace7b61b2c7e51ff1cd5"></a>
## reverse_expr

`function` · `datafusion_expr::udaf::AggregateUDFImpl::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Source: `src/udaf.rs:775`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the reverse expression of the aggregate function.

<a id="op-2730cda6b3e779ac3cd35ede"></a>
## schema_name

`function` · `datafusion_expr::udaf::AggregateUDFImpl::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self, params: &AggregateFunctionParams) -> Result<String>
```

Source: `src/udaf.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the name of the column this expression would create

See [`Expr::schema_name`](../operations/datafusion_expr.expr.Expr.md#op-82ef8179c0d4e6c2ae471137) for details

Example of schema_name: count(DISTINCT column1) FILTER (WHERE column2 > 10) ORDER BY [..]

<a id="op-f8dd0de923cb8b0cfc781632"></a>
## set_monotonicity

`function` · `datafusion_expr::udaf::AggregateUDFImpl::set_monotonicity` · datafusion-expr 55.1.0

```rust
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
```

Source: `src/udaf.rs:905`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates whether the aggregation function is monotonic as a set
function. See [`SetMonotonicity`](../operations/datafusion_expr.udaf.SetMonotonicity.md#op-a7821c42e6182e86e59e092e) for details.

<a id="op-130a8046da41201cb1a17c63"></a>
## signature

`function` · `datafusion_expr::udaf::AggregateUDFImpl::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Source: `src/udaf.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the function's [`Signature`](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for information about what input
types are accepted and the function's Volatility.

<a id="op-97048a636c94a209ef572cb4"></a>
## simplify

`function` · `datafusion_expr::udaf::AggregateUDFImpl::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self) -> Option<AggregateFunctionSimplification>
```

Source: `src/udaf.rs:702`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns an optional hook for simplifying this user-defined aggregate.

Use this hook to apply function-specific rewrites during optimization.
The default implementation returns `None`.

For example, `percentile_cont(x, 0.0)` and `percentile_cont(x, 1.0)` can
be rewritten to `MIN(x)` or `MAX(x)` depending on the `ORDER BY`
direction.

DataFusion already simplifies arguments and performs constant folding
(for example, `my_add(1, 2) -> 3`). For nested expressions, the optimizer
runs simplification in multiple passes, so arguments are typically
simplified before this hook is invoked. As a result, UDF implementations
usually do not need to handle argument simplification themselves.

See configuration `datafusion.optimizer.max_passes` for details on how many
optimization passes may be applied.

# Returns

`None` if simplify is not defined.

Or, a closure ([`AggregateFunctionSimplification`](../operations/datafusion_expr.function.AggregateFunctionSimplification.md#op-32ef850832a6ca8b0f6da43d)) invoked with:
* `aggregate_function`: [AggregateFunction](../operations/datafusion_expr.expr.AggregateFunction.md#op-2c30152da50e6f4ca7d2566e) with already simplified
  arguments
* `info`: [crate::simplify::SimplifyContext](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a)

The closure returns a simplified [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) or an error.

# Notes

The returned expression must have the same schema as the original
expression, including both the data type and nullability. For example,
if the original expression is nullable, the returned expression must
also be nullable, otherwise it may lead to schema verification errors
later in query planning.

<a id="op-491c7aa80585642342237af6"></a>
## simplify_expr_op_literal

`function` · `datafusion_expr::udaf::AggregateUDFImpl::simplify_expr_op_literal` · datafusion-expr 55.1.0

```rust
fn simplify_expr_op_literal(&self, _agg_function: &AggregateFunction, _arg: &Expr, _op: Operator, _lit: &Expr, _arg_is_left: bool) -> Result<Option<Expr>>
```

Source: `src/udaf.rs:763`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrite the aggregate to have simpler arguments

This query pattern is not common in most real workloads, and most
aggregate implementations can safely ignore it. This API is included in
DataFusion because it is important for ClickBench Q29. See backstory
on <https://github.com/apache/datafusion/issues/15524>

# Rewrite Overview

The idea is to rewrite multiple aggregates with "complex arguments" into
ones with simpler arguments that can be optimized by common subexpression
elimination (CSE). At a high level the rewrite looks like

* `Aggregate(SUM(x + 1), SUM(x + 2), ...)`

Into

* `Aggregate(SUM(x) + 1 * COUNT(x), SUM(x) + 2 * COUNT(x), ...)`

While this rewrite may seem worse (slower) than the original as it
computes *more* aggregate expressions, the common subexpression
elimination (CSE) can then reduce the number of distinct aggregates the
query actually needs to compute with a rewrite like

* `Projection(_A + 1*_B, _A + 2*_B)`
* `  Aggregate(_A = SUM(x), _B = COUNT(x))`

This optimization is extremely important for ClickBench Q29, which has 90
such expressions for some reason, and so this optimization results in
only two aggregates being needed. The DataFusion optimizer will invoke
this method when it detects multiple aggregates in a query that share
arguments of the form `<arg> <op> <literal>`.

# API

If `agg_function` supports the rewrite, it should return a semantically
equivalent expression (likely with more aggregate expressions, but
simpler arguments)

This is only called when:
1. There are no "special" aggregate params (filters, null handling, etc)
2. Aggregate functions with exactly one [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) argument
3. There are no volatile expressions

Arguments
* `agg_function`: the original aggregate function detected with complex
  arguments.
* `arg`: The common argument shared across multiple aggregates (e.g. `x`
  in the example above)
* `op`: the operator between the common argument and the literal (e.g.
  `+` in `x + 1` or `1 + x`)
* `lit`: the literal argument (e.g. `1` or `2` in the example above)
* `arg_is_left`: whether the common argument is on the left or right of
  the operator (e.g. `true` for `x + 1` and false for `1 + x`)

The default implementation returns `None`, which is what most aggregates
should do.

<a id="op-5451e9f265ed39056933ce16"></a>
## state_fields

`function` · `datafusion_expr::udaf::AggregateUDFImpl::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Source: `src/udaf.rs:579`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the fields used to store the intermediate state of this accumulator.

See [`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc) for background information.

args:  [`StateFieldsArgs`](../operations/datafusion_functions_aggregate_common.accumulator.StateFieldsArgs.md#op-e614e74caed7b2cb33bc425c) contains arguments passed to the
aggregate function's accumulator.

# Notes:

The default implementation returns a single state field named `name`
with the same type as `value_type`. This is suitable for aggregates such
as `SUM` or `MIN` where partial state can be combined by applying the
same aggregate.

For aggregates such as `AVG` where the partial state is more complex
(e.g. a COUNT and a SUM), this method is used to define the additional
fields.

The name of the fields must be unique within the query and thus should
be derived from `name`. See [`format_state_name`](../operations/datafusion_expr.utils.format_state_name.md#op-738b514bdebd66d9a4747d71) for a utility function
to generate a unique name.

<a id="op-20c2d24527663a5d2f4968cb"></a>
## supports_null_handling_clause

`function` · `datafusion_expr::udaf::AggregateUDFImpl::supports_null_handling_clause` · datafusion-expr 55.1.0

```rust
fn supports_null_handling_clause(&self) -> bool
```

Source: `src/udaf.rs:838`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If this function supports `[IGNORE NULLS | RESPECT NULLS]` SQL clause,
return `true`. Otherwise, return `false` which will cause an error to be
raised during SQL parsing if these clauses are detected for this function.

Functions which implement this as `true` are expected to handle the resulting
null handling config present in [`AccumulatorArgs`](../operations/datafusion_functions_aggregate_common.accumulator.AccumulatorArgs.md#op-0011bde6a704a46a9e4e009c), `ignore_nulls`.

<a id="op-2bf4f090182fa02ec5a2da5a"></a>
## supports_within_group_clause

`function` · `datafusion_expr::udaf::AggregateUDFImpl::supports_within_group_clause` · datafusion-expr 55.1.0

```rust
fn supports_within_group_clause(&self) -> bool
```

Source: `src/udaf.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If this function supports the `WITHIN GROUP (ORDER BY column [ASC|DESC])`
SQL syntax, return `true`. Otherwise, return `false` (default) which will
cause an error when parsing SQL where this syntax is detected for this
function.

This function should return `true` for ordered-set aggregate functions
only.

# Ordered-set aggregate functions

Ordered-set aggregate functions allow specifying a sort order that affects
how the function calculates its result, unlike other aggregate functions
like `sum` or `count`. For example, `percentile_cont` is an ordered-set
aggregate function that calculates the exact percentile value from a list
of values; the output of calculating the `0.75` percentile depends on if
you're calculating on an ascending or descending list of values.

An example of how an ordered-set aggregate function is called with the
`WITHIN GROUP` SQL syntax:

```sql
-- Ascending
SELECT percentile_cont(0.75) WITHIN GROUP (ORDER BY c1 ASC) FROM table;
-- Default ordering is ascending if not explicitly specified
SELECT percentile_cont(0.75) WITHIN GROUP (ORDER BY c1) FROM table;
-- Descending
SELECT percentile_cont(0.75) WITHIN GROUP (ORDER BY c1 DESC) FROM table;
```

This calculates the `0.75` percentile of the column `c1` from `table`,
according to the specific ordering. The column specified in the `WITHIN GROUP`
ordering clause is taken as the column to calculate values on; specifying
the `WITHIN GROUP` clause is optional so these queries are equivalent:

```sql
-- If no WITHIN GROUP is specified then default ordering is implementation
-- dependent; in this case ascending for percentile_cont
SELECT percentile_cont(c1, 0.75) FROM table;
SELECT percentile_cont(0.75) WITHIN GROUP (ORDER BY c1 ASC) FROM table;
```

Aggregate UDFs can define their default ordering if the function is called
without the `WITHIN GROUP` clause, though a default of ascending is the
standard practice.

Ordered-set aggregate function implementations are responsible for handling
the input sort order themselves (e.g. `percentile_cont` must buffer and
sort the values internally). That is, DataFusion does not introduce any
kind of sort into the plan for these functions with this syntax.

<a id="op-20017ef1e72ea9537862313e"></a>
## value_from_stats

`function` · `datafusion_expr::udaf::AggregateUDFImpl::value_from_stats` · datafusion-expr 55.1.0

```rust
fn value_from_stats(&self, _statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

Source: `src/udaf.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the value of this aggregate function if it can be determined
entirely from statistics and arguments.

Using a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) rather than a runtime computation can significantly
improving query performance.

For example, if the minimum value of column `x` is known to be `42` from
statistics, then the aggregate `MIN(x)` should return `Some(ScalarValue(42))`

<a id="op-1a19753a1d07d2da97a2dedc"></a>
## window_function_display_name

`function` · `datafusion_expr::udaf::AggregateUDFImpl::window_function_display_name` · datafusion-expr 55.1.0

```rust
fn window_function_display_name(&self, params: &WindowFunctionParams) -> Result<String>
```

Source: `src/udaf.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the user-defined display name of function, given the arguments

This can be used to customize the output column name generated by this
function.

Different from `display_name` in that it is used for window aggregate function

Defaults to `function_name([DISTINCT] column1, column2, ..) [null_treatment] [partition by [..]] [order_by [..]]`

<a id="op-2179be90b6fdd3ca93bde667"></a>
## window_function_schema_name

`function` · `datafusion_expr::udaf::AggregateUDFImpl::window_function_schema_name` · datafusion-expr 55.1.0

```rust
fn window_function_schema_name(&self, params: &WindowFunctionParams) -> Result<String>
```

Source: `src/udaf.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the name of the column this expression would create

See [`Expr::schema_name`](../operations/datafusion_expr.expr.Expr.md#op-82ef8179c0d4e6c2ae471137) for details

Different from `schema_name` in that it is used for window aggregate function

Example of schema_name: count(DISTINCT column1) FILTER (WHERE column2 > 10) [PARTITION BY [..]] [ORDER BY [..]]

<a id="op-7be7de13eb811e7ea01e8f98"></a>
## with_beneficial_ordering

`function` · `datafusion_expr::udaf::AggregateUDFImpl::with_beneficial_ordering` · datafusion-expr 55.1.0

```rust
fn with_beneficial_ordering(Arc<self>, _beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

Source: `src/udaf.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Sets the indicator whether ordering requirements of the AggregateUDFImpl is
satisfied by its input. If this is not the case, UDFs with order
sensitivity `AggregateOrderSensitivity::Beneficial` can still produce
the correct result with possibly more work internally.

# Returns

Returns `Ok(Some(updated_udf))` if the process completes successfully.
If the expression can benefit from existing input ordering, but does
not implement the method, returns an error. Order insensitive and hard
requirement aggregators return `Ok(None)`.
