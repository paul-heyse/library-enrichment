# `datafusion_expr::higher_order_function::HigherOrderUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.HigherOrderUDF.json).

<a id="op-67b8632773d35bc58334e2bb"></a>
## HigherOrderUDF

`struct` · `datafusion_expr::higher_order_function::HigherOrderUDF` · datafusion-expr 55.1.0

```rust
struct HigherOrderUDF
```

Source: `src/higher_order_function.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Logical representation of a Higher Order User Defined Function.

A higher order function takes one or more lambda arguments in addition to
regular value arguments. This struct contains the information DataFusion
needs to plan and invoke functions you supply such as name, type signature,
return type, and actual implementation.

<a id="op-9710cb02c74653e2bb57db5b"></a>
## aliases

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:984`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the aliases for this function.

See [`HigherOrderUDF::with_aliases`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-0020360419c49c633af73668) for more details.

<a id="op-3232cb2dde45d2319b0e1e57"></a>
## clear_null_values

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::clear_null_values` · datafusion-expr 55.1.0

```rust
fn clear_null_values(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1033`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether List or LargeList arguments should have non-empty null sublists
cleaned before invoking this function.

<a id="op-a691caa1927566a21ac430de"></a>
## clone

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> HigherOrderUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 17], "end": [893, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:893`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03b626aa952339477ad7a3e0"></a>
## coerce_value_types

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::coerce_value_types` · datafusion-expr 55.1.0

```rust
fn coerce_value_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1067`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce value arguments of a function call to types that the function can evaluate.

See [`HigherOrderUDFImpl::coerce_value_types`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-f20fe249c57b53787b057f56) for more details.

<a id="op-f8ffba8f7a22fa532b2ca277"></a>
## coerce_values_for_lambdas

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::coerce_values_for_lambdas` · datafusion-expr 55.1.0

```rust
fn coerce_values_for_lambdas(&self, fields: &[ValueOrLambda<DataType, DataType>]) -> Result<Option<Vec<DataType>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1014`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Coerce value arguments based on lambda output types.

See [`HigherOrderUDFImpl::coerce_values_for_lambdas`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-7b2661f5491cf7d850de20ea) for more details.

<a id="op-3e96d353f2515808357a9c73"></a>
## conditional_arguments

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::conditional_arguments` · datafusion-expr 55.1.0

```rust
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1057`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns which arguments are evaluated eagerly vs lazily.

See [`HigherOrderUDFImpl::conditional_arguments`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-b01e2b4715f4ee4ab52c839d) for more details.

<a id="op-7379d75da34652064171cc53"></a>
## documentation

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1072`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this function, if any.

<a id="op-f1521b3706161bbe163c3420"></a>
## eq

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [898, 1], "end": [902, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/higher_order_function.rs:899`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7daf6716dd276a4bac030e6"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [893, 10], "end": [893, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:893`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1441efdb83ad4a195f52d9e7"></a>
## from

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::from` · datafusion-expr 55.1.0

```rust
fn from(fun: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDFImpl", "path": "HigherOrderUDFImpl"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [1077, 1], "end": [1084, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/higher_order_function.rs:1081`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94882c96bfae7e859012647b"></a>
## hash

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [935, 1], "end": [939, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/higher_order_function.rs:936`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-831e22e860e1c6b687ac0d9c"></a>
## inner

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::inner` · datafusion-expr 55.1.0

```rust
fn inner(&self) -> &Arc<dyn HigherOrderUDFImpl>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:958`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the underlying [`HigherOrderUDFImpl`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-12806a8147b4ece5386c451b) trait object for this function.

<a id="op-db167b82de68fa40212261b2"></a>
## invoke_with_args

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::invoke_with_args` · datafusion-expr 55.1.0

```rust
fn invoke_with_args(&self, args: HigherOrderFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1040`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function returning the appropriate result.

See [`HigherOrderUDFImpl::invoke_with_args`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-de09d4c2a3079597bff655ff) for more details.

<a id="op-3ee355a137a9a2874a7ab718"></a>
## lambda_parameters

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::lambda_parameters` · datafusion-expr 55.1.0

```rust
fn lambda_parameters(&self, step: usize, fields: &[ValueOrLambda<FieldRef, Option<FieldRef>>]) -> Result<LambdaParametersProgress>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1003`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the parameters of all lambdas of this function for the current step.

See [`HigherOrderUDFImpl::lambda_parameters`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484) for more details.

<a id="op-c876ffb602b837f4c20f33ed"></a>
## name

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:977`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name.

See [`HigherOrderUDFImpl::name`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-1fcda9c26d1faeb16fa061e2) for more details.

<a id="op-89a765423f5f1a4825a61540"></a>
## new_from_impl

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::new_from_impl` · datafusion-expr 55.1.0

```rust
fn new_from_impl<F>(fun: F) -> HigherOrderUDF where F: HigherOrderUDFImpl + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:945`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `HigherOrderUDF` from a [`HigherOrderUDFImpl`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-12806a8147b4ece5386c451b) trait object.

Note this is the same as using the `From` impl (`HigherOrderUDF::from`).

<a id="op-50ed8c612fd179135cbaef65"></a>
## new_from_shared_impl

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::new_from_shared_impl` · datafusion-expr 55.1.0

```rust
fn new_from_shared_impl(fun: Arc<dyn HigherOrderUDFImpl>) -> HigherOrderUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:953`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `HigherOrderUDF` from a shared [`HigherOrderUDFImpl`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-12806a8147b4ece5386c451b) trait object.

<a id="op-94bf3642a6211647c5078e24"></a>
## partial_cmp

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [904, 1], "end": [931, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/higher_order_function.rs:905`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db5318ad2031e43a51b613df"></a>
## return_field_from_args

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::return_field_from_args` · datafusion-expr 55.1.0

```rust
fn return_field_from_args(&self, args: HigherOrderReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1024`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the return field of the function given its arguments.

See [`HigherOrderUDFImpl::return_field_from_args`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-4de68ce01e1d3df53fc7d4fb) for more details.

<a id="op-8b608996e1eeca23b03b3908"></a>
## schema_name

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self, args: &[Expr]) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:991`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's schema_name.

See [`HigherOrderUDFImpl::schema_name`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-ee2aa3f3caca6ec74c82874f) for more details.

<a id="op-61ee72e58eaa9d7748a735b1"></a>
## short_circuits

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::short_circuits` · datafusion-expr 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:1050`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if some of this function's subexpressions may not be evaluated.

See [`HigherOrderUDFImpl::short_circuits`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-0dbca17605dd5ba7162def50) for more details.

<a id="op-be856b373130fd9c5b3865dd"></a>
## signature

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &HigherOrderSignature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:996`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's [`HigherOrderSignature`](../operations/datafusion_expr.higher_order_function.HigherOrderSignature.md#op-57014c54db47f90a3230a79a).

<a id="op-0020360419c49c633af73668"></a>
## with_aliases

`function` · `datafusion_expr::higher_order_function::HigherOrderUDF::with_aliases` · datafusion-expr 55.1.0

```rust
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderUDF", "path": "HigherOrderUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [1075, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:967`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds additional names that can be used to invoke this function, in
addition to `name`.

If you implement [`HigherOrderUDFImpl`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-12806a8147b4ece5386c451b) directly you should return aliases
directly.
