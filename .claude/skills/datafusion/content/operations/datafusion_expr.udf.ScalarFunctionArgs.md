# `datafusion_expr::udf::ScalarFunctionArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udf.ScalarFunctionArgs.json).

<a id="op-363edac9d8ba73bb2e1b3696"></a>
## ScalarFunctionArgs

`struct` · `datafusion_expr::udf::ScalarFunctionArgs` · datafusion-expr 55.1.0

```rust
struct ScalarFunctionArgs
```

Source: `src/udf.rs:426`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Arguments passed to [`ScalarUDFImpl::invoke_with_args`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-412be3cb5d7fb7f10c3f7a58) when invoking a
scalar function.

<a id="op-0b5fd0d4d594d6d516d5e81c"></a>
## arg_fields

`struct_field` · `datafusion_expr::udf::ScalarFunctionArgs::arg_fields` · datafusion-expr 55.1.0

```rust
arg_fields: Vec<arrow::datatypes::FieldRef>
```

Source: `src/udf.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Field associated with each arg, if it exists

<a id="op-9fdd64c26cb70d4dce8375b1"></a>
## args

`struct_field` · `datafusion_expr::udf::ScalarFunctionArgs::args` · datafusion-expr 55.1.0

```rust
args: Vec<ColumnarValue>
```

Source: `src/udf.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The evaluated arguments to the function

<a id="op-d1834bc1cf176a26cd9f45f7"></a>
## clone

`function` · `datafusion_expr::udf::ScalarFunctionArgs::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ScalarFunctionArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarFunctionArgs", "path": "ScalarFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 17], "end": [425, 22], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udf.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fea6b68d7784d16ad60cef7d"></a>
## config_options

`struct_field` · `datafusion_expr::udf::ScalarFunctionArgs::config_options` · datafusion-expr 55.1.0

```rust
config_options: std::sync::Arc<datafusion_common::config::ConfigOptions>
```

Source: `src/udf.rs:438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The config options at execution time

<a id="op-c8a2d93af957d41073a34317"></a>
## fmt

`function` · `datafusion_expr::udf::ScalarFunctionArgs::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarFunctionArgs", "path": "ScalarFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 10], "end": [425, 15], "filename": "src/udf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da413138e4ece8062a5b684c"></a>
## number_rows

`struct_field` · `datafusion_expr::udf::ScalarFunctionArgs::number_rows` · datafusion-expr 55.1.0

```rust
number_rows: usize
```

Source: `src/udf.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The number of rows in record batch being evaluated

<a id="op-18103258f556dc5bd248bbf5"></a>
## return_field

`struct_field` · `datafusion_expr::udf::ScalarFunctionArgs::return_field` · datafusion-expr 55.1.0

```rust
return_field: arrow::datatypes::FieldRef
```

Source: `src/udf.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The return field of the scalar function returned (from `return_type`
or `return_field_from_args`) when creating the physical expression
from the logical expression

<a id="op-8b67dd4e392abccb72a0783a"></a>
## return_type

`function` · `datafusion_expr::udf::ScalarFunctionArgs::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarFunctionArgs", "path": "ScalarFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [441, 1], "end": [447, 2], "filename": "src/udf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udf.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The return type of the function. See [`Self::return_field`](../operations/datafusion_expr.udf.ScalarFunctionArgs.md#op-18103258f556dc5bd248bbf5) for more
details.
