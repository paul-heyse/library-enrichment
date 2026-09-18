# `datafusion_expr::udwf::WindowUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udwf.WindowUDF.json).

<a id="op-42e8216e20a1c833692a7e65"></a>
## WindowUDF

`struct` · `datafusion_expr::udwf::WindowUDF` · datafusion-expr 55.1.0

```rust
struct WindowUDF
```

Source: `src/udwf.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Logical representation of a user-defined window function (UDWF).

A Window Function is called via the SQL `OVER` clause:

```sql
SELECT first_value(col) OVER (PARTITION BY a, b ORDER BY c) FROM foo;
```

A UDWF is different from a user defined function (UDF) in that it is
stateful across batches.

See the documentation on [`PartitionEvaluator`] for more details

1. For simple use cases, use [`create_udwf`] (examples in
   [`simple_udwf.rs`]).

2. For advanced use cases, use [`WindowUDFImpl`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e2b205ded2fa04ff480d51c9) which provides full API
   access (examples in [`advanced_udwf.rs`]).

# API Note
This is a separate struct from `WindowUDFImpl` to maintain backwards
compatibility with the older API.

[`PartitionEvaluator`]: crate::PartitionEvaluator
[`create_udwf`]: crate::expr_fn::create_udwf
[`simple_udwf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/simple_udwf.rs
[`advanced_udwf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udwf.rs

<a id="op-376bc69a1b4d5b212c04b2bc"></a>
## aliases

`function` · `datafusion_expr::udwf::WindowUDF::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the aliases for this function.

<a id="op-13d46c2d97ba792c252df8ef"></a>
## call

`function` · `datafusion_expr::udwf::WindowUDF::call` · datafusion-expr 55.1.0

```rust
fn call(&self, args: Vec<Expr>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

creates a [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) that calls the window function with default
values for `order_by`, `partition_by`, `window_frame`.

See [`ExprFunctionExt`] for details on setting these values.

This utility allows using a user defined window function without
requiring access to the registry, such as with the DataFrame API.

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

<a id="op-083272b22584d12ab5d28f87"></a>
## clone

`function` · `datafusion_expr::udwf::WindowUDF::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 17], "end": [71, 22], "filename": "src/udwf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udwf.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-101f79c460b30f0971589100"></a>
## coerce_types

`function` · `datafusion_expr::udwf::WindowUDF::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`WindowUDFImpl::coerce_types`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-3341d214a6980a9b1a692bb2) for more details.

<a id="op-7544c4016c4fa6fa5771a964"></a>
## coerce_types

`function` · `datafusion_expr::udwf::WindowUDF::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "crate::WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [101, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2468acb9644930963d05b0a7"></a>
## documentation

`function` · `datafusion_expr::udwf::WindowUDF::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this Window UDF.

Documentation can be accessed programmatically as well as
generating publicly facing documentation.

<a id="op-826dc5edba613677cd24e155"></a>
## eq

`function` · `datafusion_expr::udwf::WindowUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [87, 2], "filename": "src/udwf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udwf.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c44da072e1c6d56b013d3bc"></a>
## expressions

`function` · `datafusion_expr::udwf::WindowUDF::expressions` · datafusion-expr 55.1.0

```rust
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Expressions that are passed to the [`PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2).

See [`WindowUDFImpl::expressions`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-0205929e53cdea9de9084167) for more details.

<a id="op-21ac43b02d4278a14bb521a9"></a>
## field

`function` · `datafusion_expr::udwf::WindowUDF::field` · datafusion-expr 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the field of the final result of evaluating this window function.

See [`WindowUDFImpl::field`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-86a60fcc397a3ec0b61ba3fa) for more details.

<a id="op-3de17df1c9ab8364c4b8d67b"></a>
## fmt

`function` · `datafusion_expr::udwf::WindowUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/udwf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udwf.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998617147379fc977ed54e55"></a>
## fmt

`function` · `datafusion_expr::udwf::WindowUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [81, 2], "filename": "src/udwf.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/udwf.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aad426365d0c32c78520591b"></a>
## from

`function` · `datafusion_expr::udwf::WindowUDF::from` · datafusion-expr 55.1.0

```rust
fn from(fun: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [218, 1], "end": [225, 2], "filename": "src/udwf.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udwf.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebc915e6e00aa97450ebf837"></a>
## hash

`function` · `datafusion_expr::udwf::WindowUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [95, 2], "filename": "src/udwf.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udwf.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72daa72be64e3a8bf0ab06c9"></a>
## inner

`function` · `datafusion_expr::udwf::WindowUDF::inner` · datafusion-expr 55.1.0

```rust
fn inner(&self) -> &Arc<dyn WindowUDFImpl>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the underlying [`WindowUDFImpl`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e2b205ded2fa04ff480d51c9) trait object for this function

<a id="op-8a912b48a52ae4ecfc63e727"></a>
## name

`function` · `datafusion_expr::udwf::WindowUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name

See [`WindowUDFImpl::name`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-f86aece97efcb21e917ad2c3) for more details.

<a id="op-d19c120dbf3329640cfd9fca"></a>
## name

`function` · `datafusion_expr::udwf::WindowUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "crate::WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [101, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0ca10828595a2739119515c"></a>
## new_from_impl

`function` · `datafusion_expr::udwf::WindowUDF::new_from_impl` · datafusion-expr 55.1.0

```rust
fn new_from_impl<F>(fun: F) -> WindowUDF where F: WindowUDFImpl + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `WindowUDF` from a `[WindowUDFImpl]` trait object

Note this is the same as using the `From` impl (`WindowUDF::from`)

<a id="op-47fbeca07fe945528960526e"></a>
## new_from_shared_impl

`function` · `datafusion_expr::udwf::WindowUDF::new_from_shared_impl` · datafusion-expr 55.1.0

```rust
fn new_from_shared_impl(fun: Arc<dyn WindowUDFImpl>) -> WindowUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `WindowUDF` from a `[WindowUDFImpl]` trait object

<a id="op-86bd86e43e96668eb2daba32"></a>
## partial_cmp

`function` · `datafusion_expr::udwf::WindowUDF::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowUDF) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 24], "end": [71, 34], "filename": "src/udwf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/udwf.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50c2bf911581ac89a3456637"></a>
## partition_evaluator_factory

`function` · `datafusion_expr::udwf::WindowUDF::partition_evaluator_factory` · datafusion-expr 55.1.0

```rust
fn partition_evaluator_factory(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `PartitionEvaluator` for evaluating this window function

<a id="op-4d955d8987143d2b61aef472"></a>
## reverse_expr

`function` · `datafusion_expr::udwf::WindowUDF::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDWF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the reversed user-defined window function when the
order of evaluation is reversed.

See [`WindowUDFImpl::reverse_expr`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-8cac07a32e8db1c7df4ea370) for more details.

<a id="op-289716c487a2c967d070d9ea"></a>
## signature

`function` · `datafusion_expr::udwf::WindowUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "crate::WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [101, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e20f6387020a1ce6981d77d"></a>
## signature

`function` · `datafusion_expr::udwf::WindowUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's signature (what input types are accepted)

See [`WindowUDFImpl::signature`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e6be77ee117b0a6e4428320a) for more details.

<a id="op-c19e675082dd444f4e084e59"></a>
## simplify

`function` · `datafusion_expr::udwf::WindowUDF::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self) -> Option<WindowFunctionSimplification>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this window function's simplification hook, if any.

See [`WindowUDFImpl::simplify`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-c1739b18874ddfb371d89fe5) for more details.

<a id="op-f265d904195ac86e9ba1c37c"></a>
## sort_options

`function` · `datafusion_expr::udwf::WindowUDF::sort_options` · datafusion-expr 55.1.0

```rust
fn sort_options(&self) -> Option<SortOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns custom result ordering introduced by this window function
which is used to update ordering equivalences.

See [`WindowUDFImpl::sort_options`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-d88f2961b6232b5741dffeaf) for more details.

<a id="op-77fc701aa544772ceba91962"></a>
## with_aliases

`function` · `datafusion_expr::udwf::WindowUDF::with_aliases` · datafusion-expr 55.1.0

```rust
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [216, 2], "filename": "src/udwf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udwf.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds additional names that can be used to invoke this function, in
addition to `name`

If you implement [`WindowUDFImpl`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e2b205ded2fa04ff480d51c9) directly you should return aliases directly.
