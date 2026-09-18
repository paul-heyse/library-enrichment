# `datafusion_expr::higher_order_function::HigherOrderFunctionArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.HigherOrderFunctionArgs.json).

<a id="op-ac2ad906a649c2914d7a103e"></a>
## HigherOrderFunctionArgs

`struct` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs` · datafusion-expr 55.1.0

```rust
struct HigherOrderFunctionArgs
```

Source: `src/higher_order_function.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Arguments passed to [`HigherOrderUDFImpl::invoke_with_args`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-de09d4c2a3079597bff655ff) when invoking a
higher order function.

<a id="op-2785a2e012fa5ea65626765b"></a>
## arg_fields

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::arg_fields` · datafusion-expr 55.1.0

```rust
arg_fields: Vec<ValueOrLambda<arrow::datatypes::FieldRef, arrow::datatypes::FieldRef>>
```

Source: `src/higher_order_function.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Field associated with each arg, if it exists
For lambdas, it will be the field of the result of
the lambda if evaluated with the parameters
returned from [`HigherOrderUDFImpl::lambda_parameters`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484)

<a id="op-f5145eca119c81b04d2733db"></a>
## args

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::args` · datafusion-expr 55.1.0

```rust
args: Vec<ValueOrLambda<ColumnarValue, LambdaArgument>>
```

Source: `src/higher_order_function.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The evaluated arguments and lambdas to the function

<a id="op-7c0960037cfa52294b5821a4"></a>
## clone

`function` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> HigherOrderFunctionArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderFunctionArgs", "path": "HigherOrderFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 17], "end": [207, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6be44245343215c4259a0b00"></a>
## config_options

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::config_options` · datafusion-expr 55.1.0

```rust
config_options: std::sync::Arc<datafusion_common::config::ConfigOptions>
```

Source: `src/higher_order_function.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The config options at execution time

<a id="op-d363e3dce8a4a3a326bf8019"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderFunctionArgs", "path": "HigherOrderFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 10], "end": [207, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80bc7ec808b13191feecf25c"></a>
## number_rows

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::number_rows` · datafusion-expr 55.1.0

```rust
number_rows: usize
```

Source: `src/higher_order_function.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The number of rows in record batch being evaluated

<a id="op-3a983da9f68c617ff4c51b27"></a>
## return_field

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::return_field` · datafusion-expr 55.1.0

```rust
return_field: arrow::datatypes::FieldRef
```

Source: `src/higher_order_function.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The return field of the higher order function returned
(from `return_field_from_args`) when creating the
physical expression from the logical expression

<a id="op-4665eb90fff0a305c24dc07e"></a>
## return_type

`function` · `datafusion_expr::higher_order_function::HigherOrderFunctionArgs::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderFunctionArgs", "path": "HigherOrderFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 1], "end": [232, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The return type of the function. See [`Self::return_field`](../operations/datafusion_expr.higher_order_function.HigherOrderFunctionArgs.md#op-3a983da9f68c617ff4c51b27) for more
details.
