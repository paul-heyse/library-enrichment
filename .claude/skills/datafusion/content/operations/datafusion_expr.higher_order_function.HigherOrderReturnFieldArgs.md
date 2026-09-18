# `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.HigherOrderReturnFieldArgs.json).

<a id="op-360bed75d27dccd9cef87957"></a>
## HigherOrderReturnFieldArgs

`struct` · `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs` · datafusion-expr 55.1.0

```rust
struct HigherOrderReturnFieldArgs<'a>
```

Source: `src/higher_order_function.rs:461`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Information about arguments passed to the function

This structure contains metadata about how the function was called
such as the type of the arguments, any scalar arguments and if the
arguments can (ever) be null

See [`HigherOrderUDFImpl::return_field_from_args`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-4de68ce01e1d3df53fc7d4fb) for more information

<a id="op-f997050803e6a5706faea1c2"></a>
## arg_fields

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs::arg_fields` · datafusion-expr 55.1.0

```rust
arg_fields: &'a [ValueOrLambda<arrow::datatypes::FieldRef, arrow::datatypes::FieldRef>]
```

Source: `src/higher_order_function.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The data types of the arguments to the function

If argument `i` to the function is a lambda, it will be the field of the result of the
lambda if evaluated with the parameters returned from [`HigherOrderUDFImpl::lambda_parameters`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484)

For example, with `array_transform([1], v -> v == 5)`
this field will be
```ignore
[
    ValueOrLambda::Value(Field::new("", DataType::new_list(DataType::Int32, true), true)),
    ValueOrLambda::Lambda(Field::new("", DataType::Boolean, true))
]
```

<a id="op-ca912706622f632251a5b949"></a>
## clone

`function` · `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> HigherOrderReturnFieldArgs<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs", "path": "HigherOrderReturnFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 10], "end": [460, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e73c079557cd45cb6beb984"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs", "path": "HigherOrderReturnFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 17], "end": [460, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0df448fc008aca75c3c5b6fa"></a>
## scalar_arguments

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderReturnFieldArgs::scalar_arguments` · datafusion-expr 55.1.0

```rust
scalar_arguments: &'a [Option<&'a datafusion_common::ScalarValue>]
```

Source: `src/higher_order_function.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Is argument `i` to the function a scalar (constant)?

If the argument `i` is not a scalar, it will be None

For example, if a function is called like `array_transform([1], v -> v == 5)`
this field will be `[Some(ScalarValue::List(...), None]`
