# `datafusion_expr::udf::ReturnFieldArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udf.ReturnFieldArgs.json).

<a id="op-45e9203bcbe458f8338d7ffd"></a>
## ReturnFieldArgs

`struct` · `datafusion_expr::udf::ReturnFieldArgs` · datafusion-expr 55.1.0

```rust
struct ReturnFieldArgs<'a>
```

Source: `src/udf.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Information about arguments passed to the function

This structure contains metadata about how the function was called
such as the type of the arguments, any scalar arguments and if the
arguments can (ever) be null

See [`ScalarUDFImpl::return_field_from_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-efa24f895df9cedbd3cf8c2a) for more information

<a id="op-d0e5f601bbff7eeea8d04205"></a>
## arg_fields

`struct_field` · `datafusion_expr::udf::ReturnFieldArgs::arg_fields` · datafusion-expr 55.1.0

```rust
arg_fields: &'a [arrow::datatypes::FieldRef]
```

Source: `src/udf.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The data types of the arguments to the function

<a id="op-35415f16a99e5bfb180bae76"></a>
## fmt

`function` · `datafusion_expr::udf::ReturnFieldArgs::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::udf::ReturnFieldArgs", "path": "ReturnFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 10], "end": [456, 15], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd3c5c8fadd24d8ca1efcc42"></a>
## scalar_arguments

`struct_field` · `datafusion_expr::udf::ReturnFieldArgs::scalar_arguments` · datafusion-expr 55.1.0

```rust
scalar_arguments: &'a [Option<&'a datafusion_common::ScalarValue>]
```

Source: `src/udf.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Is argument `i` to the function a scalar (constant)?

If the argument `i` is not a scalar, it will be None

For example, if a function is called like `my_function(column_a, 5)`
this field will be `[None, Some(ScalarValue::Int32(Some(5)))]`
