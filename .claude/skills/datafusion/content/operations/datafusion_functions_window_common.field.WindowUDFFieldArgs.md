# `datafusion_functions_window_common::field::WindowUDFFieldArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window_common.field.WindowUDFFieldArgs.json).

<a id="op-19074c9fba1249123677fb19"></a>
## WindowUDFFieldArgs

`struct` · `datafusion_functions_window_common::field::WindowUDFFieldArgs` · datafusion-functions-window-common 55.1.0

```rust
struct WindowUDFFieldArgs<'a>
```

Source: `src/field.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Metadata for defining the result field from evaluating a
user-defined window function.

<a id="op-563892db4681d9df18362ea6"></a>
## get_input_field

`function` · `datafusion_functions_window_common::field::WindowUDFFieldArgs::get_input_field` · datafusion-functions-window-common 55.1.0

```rust
fn get_input_field(&self, index: usize) -> Option<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::field::WindowUDFFieldArgs", "path": "WindowUDFFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [63, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns `Some(Field)` of input expression at index, otherwise
returns `None` if the index is out of bounds.

<a id="op-0d8b3048d985818ee186dd69"></a>
## input_fields

`function` · `datafusion_functions_window_common::field::WindowUDFFieldArgs::input_fields` · datafusion-functions-window-common 55.1.0

```rust
fn input_fields(&self) -> &[FieldRef]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::field::WindowUDFFieldArgs", "path": "WindowUDFFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [63, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns the field of input expressions passed as arguments
to the user-defined window function.

<a id="op-569977ba0b69b5a5c9b32f4e"></a>
## name

`function` · `datafusion_functions_window_common::field::WindowUDFFieldArgs::name` · datafusion-functions-window-common 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::field::WindowUDFFieldArgs", "path": "WindowUDFFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [63, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns the name for the field of the final result of evaluating
the user-defined window function.

<a id="op-53a51c2d33fc3b14dbd7d924"></a>
## new

`function` · `datafusion_functions_window_common::field::WindowUDFFieldArgs::new` · datafusion-functions-window-common 55.1.0

```rust
fn new(input_fields: &'a [FieldRef], display_name: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::field::WindowUDFFieldArgs", "path": "WindowUDFFieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [63, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Create an instance of [`WindowUDFFieldArgs`](../operations/datafusion_functions_window_common.field.WindowUDFFieldArgs.md#op-19074c9fba1249123677fb19).

# Arguments

* `input_fields` - The fields corresponding to the
  arguments to the user-defined window function.
* `function_name` - The qualified schema name of the
  user-defined window function expression.
