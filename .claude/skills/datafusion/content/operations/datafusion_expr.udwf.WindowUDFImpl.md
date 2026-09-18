# `datafusion_expr::udwf::WindowUDFImpl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udwf.WindowUDFImpl.json).

<a id="op-e2b205ded2fa04ff480d51c9"></a>
## WindowUDFImpl

`trait` · `datafusion_expr::udwf::WindowUDFImpl` · datafusion-expr 55.1.0

```rust
trait WindowUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

Source: `src/udwf.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Trait for implementing [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65).

This trait exposes the full API for implementing user defined window functions and
can be used to implement any function.

While the trait depends on [`DynEq`](../operations/datafusion_expr_common.dyn_eq.DynEq.md#op-ee2f7a30a4c03e9d5680ea5a) and [`DynHash`](../operations/datafusion_expr_common.dyn_eq.DynHash.md#op-b1cd12827f1210014f9301ab) traits, these should not be
implemented directly. Instead, implement [`Eq`] and [`Hash`] and leverage the
blanket implementations of [`DynEq`](../operations/datafusion_expr_common.dyn_eq.DynEq.md#op-ee2f7a30a4c03e9d5680ea5a) and [`DynHash`](../operations/datafusion_expr_common.dyn_eq.DynHash.md#op-b1cd12827f1210014f9301ab).

See [`advanced_udwf.rs`] for a full example with complete implementation and
[`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65) for other available options.


[`advanced_udwf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udwf.rs
# Basic Example
```
# use std::sync::LazyLock;
# use arrow::datatypes::{DataType, Field, FieldRef};
# use datafusion_common::{DataFusionError, plan_err, Result};
# use datafusion_expr::{col, Signature, Volatility, PartitionEvaluator, WindowFrame, ExprFunctionExt, Documentation, LimitEffect};
# use datafusion_expr::{WindowUDFImpl, WindowUDF};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
# use datafusion_expr::window_doc_sections::DOC_SECTION_ANALYTICAL;
# use datafusion_physical_expr_common::physical_expr;
# use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SmoothIt {
  signature: Signature,
}

impl SmoothIt {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Int32], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
    Documentation::builder(DOC_SECTION_ANALYTICAL, "smooths the windows", "smooth_it(2)")
        .with_argument("arg1", "The int32 number to smooth by")
        .build()
});

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the WindowUDFImpl trait for SmoothIt
impl WindowUDFImpl for SmoothIt {
   fn name(&self) -> &str { "smooth_it" }
   fn signature(&self) -> &Signature { &self.signature }
   // The actual implementation would smooth the window
   fn partition_evaluator(
       &self,
       _partition_evaluator_args: PartitionEvaluatorArgs,
   ) -> Result<Box<dyn PartitionEvaluator>> {
       unimplemented!()
   }
   fn field(&self, field_args: WindowUDFFieldArgs) -> Result<FieldRef> {
     if let Some(DataType::Int32) = field_args.get_input_field(0).map(|f| f.data_type().clone()) {
       Ok(Field::new(field_args.name(), DataType::Int32, false).into())
     } else {
       plan_err!("smooth_it only accepts Int32 arguments")
     }
   }
   fn documentation(&self) -> Option<&Documentation> {
     Some(get_doc())
   }
    fn limit_effect(&self, _args: &[Arc<dyn physical_expr::PhysicalExpr>]) -> LimitEffect {
        LimitEffect::Unknown
    }
}

// Create a new WindowUDF from the implementation
let smooth_it = WindowUDF::from(SmoothIt::new());

// Call the function `add_one(col)`
// smooth_it(speed) OVER (PARTITION BY car ORDER BY time ASC)
let expr = smooth_it.call(vec![col("speed")])
    .partition_by(vec![col("car")])
    .order_by(vec![col("time").sort(true, true)])
    .window_frame(WindowFrame::new(None))
    .build()
    .unwrap();
```

Unresolved upstream links (retained, not inferred): ``Hash``, ``Eq``.

<a id="op-b7c1c067ea1bc71f9338cbc0"></a>
## aliases

`function` · `datafusion_expr::udwf::WindowUDFImpl::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Source: `src/udwf.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns any aliases (alternate names) for this function.

Note: `aliases` should only include names other than [`Self::name`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-f86aece97efcb21e917ad2c3).
Defaults to `[]` (no aliases)

<a id="op-3341d214a6980a9b1a692bb2"></a>
## coerce_types

`function` · `datafusion_expr::udwf::WindowUDFImpl::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Source: `src/udwf.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce arguments of a function call to types that the function can evaluate.

This function is only called if [`WindowUDFImpl::signature`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e6be77ee117b0a6e4428320a) returns [`crate::TypeSignature::UserDefined`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-9a54792c384e19dbdaacb169). Most
UDWFs should return one of the other variants of `TypeSignature` which handle common
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

<a id="op-35018f0efcf158c90bc26345"></a>
## documentation

`function` · `datafusion_expr::udwf::WindowUDFImpl::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Source: `src/udwf.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this Window UDF.

Documentation can be accessed programmatically as well as
generating publicly facing documentation.

<a id="op-0205929e53cdea9de9084167"></a>
## expressions

`function` · `datafusion_expr::udwf::WindowUDFImpl::expressions` · datafusion-expr 55.1.0

```rust
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
```

Source: `src/udwf.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the expressions that are passed to the [`PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2).

<a id="op-86a60fcc397a3ec0b61ba3fa"></a>
## field

`function` · `datafusion_expr::udwf::WindowUDFImpl::field` · datafusion-expr 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Source: `src/udwf.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) of the final result of evaluating this window function.

Call `field_args.name()` to get the fully qualified name for defining
the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542). For a complete example see the implementation in the
[Basic Example](WindowUDFImpl#basic-example) section.

<a id="op-957cd43c1fe93d411e201c0d"></a>
## limit_effect

`function` · `datafusion_expr::udwf::WindowUDFImpl::limit_effect` · datafusion-expr 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Source: `src/udwf.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If not causal, returns the effect this function will have on the window

<a id="op-f86aece97efcb21e917ad2c3"></a>
## name

`function` · `datafusion_expr::udwf::WindowUDFImpl::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/udwf.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name

<a id="op-7f0d4a74641bee694f2223f1"></a>
## partition_evaluator

`function` · `datafusion_expr::udwf::WindowUDFImpl::partition_evaluator` · datafusion-expr 55.1.0

```rust
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Source: `src/udwf.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function, returning the [`PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) instance

<a id="op-8cac07a32e8db1c7df4ea370"></a>
## reverse_expr

`function` · `datafusion_expr::udwf::WindowUDFImpl::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDWF
```

Source: `src/udwf.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Allows customizing the behavior of the user-defined window
function when it is evaluated in reverse order.

<a id="op-e6be77ee117b0a6e4428320a"></a>
## signature

`function` · `datafusion_expr::udwf::WindowUDFImpl::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Source: `src/udwf.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the function's [`Signature`](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for information about what input
types are accepted and the function's Volatility.

<a id="op-c1739b18874ddfb371d89fe5"></a>
## simplify

`function` · `datafusion_expr::udwf::WindowUDFImpl::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self) -> Option<WindowFunctionSimplification>
```

Source: `src/udwf.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns an optional hook for simplifying this user-defined window
function.

Use this hook to apply function-specific rewrites during optimization.
The default implementation returns `None`.

DataFusion already simplifies arguments and performs constant folding
(for example, `my_add(1, 2) -> 3`), so there is usually no need to
implement those optimizations manually for specific UDFs.

Example:
`advanced_udwf.rs`: <https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udwf.rs>

# Returns
`None` if simplify is not defined.

Or, a closure ([`WindowFunctionSimplification`](../operations/datafusion_expr.function.WindowFunctionSimplification.md#op-4e3825e9eb717b05a23f2e0d)) invoked with:
* `window_function`: [WindowFunction](../operations/datafusion_expr.expr.WindowFunction.md#op-8f5e8e8a659430c3e7448f3a) with already simplified
  arguments
* `info`: [crate::simplify::SimplifyContext](../operations/datafusion_expr.simplify.SimplifyContext.md#op-7b5998cee70553815ef2a10a)

The closure returns a simplified [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) or an error.

# Notes
The returned expression must have the same schema as the original
expression, including both the data type and nullability. For example,
if the original expression is nullable, the returned expression must
also be nullable, otherwise it may lead to schema verification errors
later in query planning.

<a id="op-d88f2961b6232b5741dffeaf"></a>
## sort_options

`function` · `datafusion_expr::udwf::WindowUDFImpl::sort_options` · datafusion-expr 55.1.0

```rust
fn sort_options(&self) -> Option<SortOptions>
```

Source: `src/udwf.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Allows the window UDF to define a custom result ordering.

By default, a window UDF doesn't introduce an ordering.
But when specified by a window UDF this is used to update
ordering equivalences.
