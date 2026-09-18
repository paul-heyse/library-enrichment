# `datafusion_expr::udf::ScalarUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udf.ScalarUDF.json).

<a id="op-12eb8b815a2294fbbbd085a0"></a>
## ScalarUDF

`struct` · `datafusion_expr::udf::ScalarUDF` · datafusion-expr 55.1.0

```rust
struct ScalarUDF
```

Source: `src/udf.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Logical representation of a Scalar User Defined Function.

A scalar function produces a single row output for each row of input. This
struct contains the information DataFusion needs to plan and invoke
functions you supply such as name, type signature, return type, and actual
implementation.

1. For simple use cases, use [`create_udf`] (examples in [`simple_udf.rs`]).

2. For advanced use cases, use [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) which provides full API
   access (examples in  [`advanced_udf.rs`]).

See [`Self::call`](../operations/datafusion_expr.udf.ScalarUDF.md#op-5bee7387f4bfbc22e08e3489) to create an `Expr` which invokes a `ScalarUDF` with arguments.

# API Note

This is a separate struct from [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) to maintain backwards
compatibility with the older API.

[`create_udf`]: crate::expr_fn::create_udf
[`simple_udf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/simple_udf.rs
[`advanced_udf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udf.rs

<a id="op-7543f91e7e27917901af33b7"></a>
## aliases

`function` · `datafusion_expr::udf::ScalarUDF::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the aliases for this function.

See [`ScalarUDF::with_aliases`](../operations/datafusion_expr.udf.ScalarUDF.md#op-6b3763add92cc647eeabbf53) for more details

<a id="op-7ff8ca396cc21fdd8ce42366"></a>
## as_async

`function` · `datafusion_expr::udf::ScalarUDF::as_async` · datafusion-expr 55.1.0

```rust
fn as_async(&self) -> Option<&AsyncScalarUDF>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return true if this function is an async function

<a id="op-5bee7387f4bfbc22e08e3489"></a>
## call

`function` · `datafusion_expr::udf::ScalarUDF::call` · datafusion-expr 55.1.0

```rust
fn call(&self, args: Vec<Expr>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) logical expression to call this UDF with specified
arguments.

This utility allows easily calling UDFs

# Example
```no_run
use datafusion_expr::{col, lit, ScalarUDF};
# fn my_udf() -> ScalarUDF { unimplemented!() }
let my_func: ScalarUDF = my_udf();
// Create an expr for `my_func(a, 12.3)`
let expr = my_func.call(vec![col("a"), lit(12.3)]);
```

<a id="op-7f4983233231d493471f25fb"></a>
## clone

`function` · `datafusion_expr::udf::ScalarUDF::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ScalarUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 17], "end": [82, 22], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udf.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-375dff5a7c82c6790615845a"></a>
## coerce_types

`function` · `datafusion_expr::udf::ScalarUDF::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`ScalarUDFImpl::coerce_types`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-2418bef30f1b8de36e5ac4ec) for more details.

<a id="op-f45e25b609daa6799752c3a9"></a>
## coerce_types

`function` · `datafusion_expr::udf::ScalarUDF::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "crate::ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [73, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-246c9d2ccb4e2fec5a0f5e78"></a>
## conditional_arguments

`function` · `datafusion_expr::udf::ScalarUDF::conditional_arguments` · datafusion-expr 55.1.0

```rust
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Determines which of the arguments passed to this function are evaluated eagerly
and which may be evaluated lazily.

See [ScalarUDFImpl::conditional_arguments](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-609aba8577614f2ef39b411c) for more information.

<a id="op-4994261aae512680743de86b"></a>
## display_name

`function` · `datafusion_expr::udf::ScalarUDF::display_name` · datafusion-expr 55.1.0

```rust
fn display_name(&self, args: &[Expr]) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's display_name.

See [`ScalarUDFImpl::display_name`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-11c9162f06a7f8f322590763) for more details

<a id="op-555fe08be62d5f0ba1fc0cb2"></a>
## documentation

`function` · `datafusion_expr::udf::ScalarUDF::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this Scalar UDF.

Documentation can be accessed programmatically as well as
generating publicly facing documentation.

<a id="op-e4a0cb42e3001e8a3c14a8ec"></a>
## eq

`function` · `datafusion_expr::udf::ScalarUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [91, 2], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udf.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ed165e375c0d4640c35571c"></a>
## evaluate_bounds

`function` · `datafusion_expr::udf::ScalarUDF::evaluate_bounds` · datafusion-expr 55.1.0

```rust
fn evaluate_bounds(&self, inputs: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Computes the output interval for a [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0), given the input
intervals.

# Parameters

* `inputs` are the intervals for the inputs (children) of this function.

# Example

If the function is `ABS(a)`, and the input interval is `a: [-3, 2]`,
then the output interval would be `[0, 3]`.

<a id="op-c2d401589c11440f118f9374"></a>
## fmt

`function` · `datafusion_expr::udf::ScalarUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 10], "end": [82, 15], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-563e02ef7ce75bb362701565"></a>
## from

`function` · `datafusion_expr::udf::ScalarUDF::from` · datafusion-expr 55.1.0

```rust
fn from(fun: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [414, 1], "end": [421, 2], "filename": "src/udf.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udf.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c54bbcee0aacc4bf7df9d24"></a>
## hash

`function` · `datafusion_expr::udf::ScalarUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [128, 2], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udf.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47b2f92b0b24954d9b11d045"></a>
## inner

`function` · `datafusion_expr::udf::ScalarUDF::inner` · datafusion-expr 55.1.0

```rust
fn inner(&self) -> &Arc<dyn ScalarUDFImpl>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the underlying [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) trait object for this function

<a id="op-d214a8f4ac5b0c24f216472f"></a>
## invoke_with_args

`function` · `datafusion_expr::udf::ScalarUDF::invoke_with_args` · datafusion-expr 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function on `args`, returning the appropriate result.

See [`ScalarUDFImpl::invoke_with_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-412be3cb5d7fb7f10c3f7a58) for details.

<a id="op-00d9e990a12b7d7da1f9ca3a"></a>
## is_nullable

`function` · `datafusion_expr::udf::ScalarUDF::is_nullable` · datafusion-expr 55.1.0

```rust
fn is_nullable(&self, args: &[Expr], schema: &dyn ExprSchema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6a7a0edf4b6621d7e81b3a7"></a>
## is_strict

`function` · `datafusion_expr::udf::ScalarUDF::is_strict` · datafusion-expr 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if this function always returns NULL when any argument is
NULL.

See [`ScalarUDFImpl::is_strict`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-cabf9d40d1fe8712d7e55eef) for more details.

<a id="op-0c5a8076ed0bf9d187adf9da"></a>
## name

`function` · `datafusion_expr::udf::ScalarUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "crate::ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [73, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9636aad62898c4abb7c9c555"></a>
## name

`function` · `datafusion_expr::udf::ScalarUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name.

See [`ScalarUDFImpl::name`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-1b6686681755a5ff0c22a2d8) for more details.

<a id="op-63ffe21d049a03e423d22420"></a>
## new_from_impl

`function` · `datafusion_expr::udf::ScalarUDF::new_from_impl` · datafusion-expr 55.1.0

```rust
fn new_from_impl<F>(fun: F) -> ScalarUDF where F: ScalarUDFImpl + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `ScalarUDF` from a `[ScalarUDFImpl]` trait object

Note this is the same as using the `From` impl (`ScalarUDF::from`)

<a id="op-2cc7cb6deb4068d1ece57b54"></a>
## new_from_shared_impl

`function` · `datafusion_expr::udf::ScalarUDF::new_from_shared_impl` · datafusion-expr 55.1.0

```rust
fn new_from_shared_impl(fun: Arc<dyn ScalarUDFImpl>) -> ScalarUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `ScalarUDF` from a `[ScalarUDFImpl]` trait object

<a id="op-cdbe7be5f63d64c21b34ffbe"></a>
## output_ordering

`function` · `datafusion_expr::udf::ScalarUDF::output_ordering` · datafusion-expr 55.1.0

```rust
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Calculates the [`SortProperties`](../operations/datafusion_expr_common.sort_properties.SortProperties.md#op-a357ca132b9df8290c59f5bf) of this function based on its
children's properties.

<a id="op-c6260230a81c632fc461d571"></a>
## partial_cmp

`function` · `datafusion_expr::udf::ScalarUDF::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [120, 2], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/udf.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0189b4dacd6239e60a218cbd"></a>
## placement

`function` · `datafusion_expr::udf::ScalarUDF::placement` · datafusion-expr 55.1.0

```rust
fn placement(&self, args: &[ExpressionPlacement]) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns placement information for this function.

See [`ScalarUDFImpl::placement`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-6e36a0acc77db21760b4a45e) for more details.

<a id="op-3fc8894f5665693680a26858"></a>
## preimage

`function` · `datafusion_expr::udf::ScalarUDF::preimage` · datafusion-expr 55.1.0

```rust
fn preimage(&self, args: &[Expr], lit_expr: &Expr, info: &SimplifyContext) -> Result<PreimageResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a preimage

See [`ScalarUDFImpl::preimage`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-0f3b7cc7d7d1203c14fd2c9f) for more details.

<a id="op-dfae9490edd3f0e26b4f028a"></a>
## preserves_lex_ordering

`function` · `datafusion_expr::udf::ScalarUDF::preserves_lex_ordering` · datafusion-expr 55.1.0

```rust
fn preserves_lex_ordering(&self, inputs: &[ExprProperties]) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8603fdf655643ce82314f960"></a>
## propagate_constraints

`function` · `datafusion_expr::udf::ScalarUDF::propagate_constraints` · datafusion-expr 55.1.0

```rust
fn propagate_constraints(&self, interval: &Interval, inputs: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Updates bounds for child expressions, given a known interval for this
function. This is used to propagate constraints down through an expression
tree.

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

<a id="op-e49de3c74acf317cbf69578b"></a>
## return_field_from_args

`function` · `datafusion_expr::udf::ScalarUDF::return_field_from_args` · datafusion-expr 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the datatype this function returns given the input argument types.

See [`ScalarUDFImpl::return_field_from_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-efa24f895df9cedbd3cf8c2a) for more details.

<a id="op-5fe0f85a284f7719d64c3803"></a>
## return_type

`function` · `datafusion_expr::udf::ScalarUDF::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The datatype this function returns given the input argument types.
This function is used when the input arguments are [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)s.

 # Notes

If a function implement [`ScalarUDFImpl::return_field_from_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-efa24f895df9cedbd3cf8c2a),
its [`ScalarUDFImpl::return_type`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-656507bbb7a195e311356b01) should raise an error.

See [`ScalarUDFImpl::return_type`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-656507bbb7a195e311356b01) for more details.

<a id="op-15e98878710c4f0ac12eb4c1"></a>
## schema_name

`function` · `datafusion_expr::udf::ScalarUDF::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self, args: &[Expr]) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's schema_name.

See [`ScalarUDFImpl::schema_name`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-2594946d62d840b6b88d094f) for more details

<a id="op-d5be2797273054a8fb92acc6"></a>
## short_circuits

`function` · `datafusion_expr::udf::ScalarUDF::short_circuits` · datafusion-expr 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if some of this `exprs` subexpressions may not be evaluated
and thus any side effects (like divide by zero) may not be encountered.

See [ScalarUDFImpl::short_circuits](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-d442484f46ac4203797da7d0) for more information.

<a id="op-8dbfcec855c40d5284decffa"></a>
## signature

`function` · `datafusion_expr::udf::ScalarUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "crate::ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [73, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a26d546cca40671f7d56d593"></a>
## signature

`function` · `datafusion_expr::udf::ScalarUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's [`Signature`](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) (what input types are accepted).

See [`ScalarUDFImpl::signature`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-0b3a009ba3a69eab52444d72) for more details.

<a id="op-462b00f50d2670a83b6901cd"></a>
## simplify

`function` · `datafusion_expr::udf::ScalarUDF::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this scalar function's simplification result.

See [`ScalarUDFImpl::simplify`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-5b09f606d091d093ed8a0aec) for more details.

<a id="op-1cb529bbe4c8c719e5f02e89"></a>
## strictly_order_preserving

`function` · `datafusion_expr::udf::ScalarUDF::strictly_order_preserving` · datafusion-expr 55.1.0

```rust
fn strictly_order_preserving(&self, inputs: &[ExprProperties]) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`ScalarUDFImpl::strictly_order_preserving`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-aa3db0a55467ecf3dc3cfcab) for more details.

<a id="op-40541d2a1f7e4faaec6d9a21"></a>
## struct_field_mapping

`function` · `datafusion_expr::udf::ScalarUDF::struct_field_mapping` · datafusion-expr 55.1.0

```rust
fn struct_field_mapping(&self, literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`ScalarUDFImpl::struct_field_mapping`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-046bec345ba5869e7f1b1c29) for more details.

<a id="op-6b3763add92cc647eeabbf53"></a>
## with_aliases

`function` · `datafusion_expr::udf::ScalarUDF::with_aliases` · datafusion-expr 55.1.0

```rust
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [412, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds additional names that can be used to invoke this function, in
addition to `name`

If you implement [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) directly you should return aliases directly.
