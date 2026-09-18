# `datafusion_functions_window_common::expr::ExpressionArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window_common.expr.ExpressionArgs.json).

<a id="op-2c48b2a7121c7066966c54e9"></a>
## ExpressionArgs

`struct` · `datafusion_functions_window_common::expr::ExpressionArgs` · datafusion-functions-window-common 55.1.0

```rust
struct ExpressionArgs<'a>
```

Source: `src/expr.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Arguments passed to user-defined window function

<a id="op-3b67d7588714cac8c2bbb1b5"></a>
## default

`function` · `datafusion_functions_window_common::expr::ExpressionArgs::default` · datafusion-functions-window-common 55.1.0

```rust
fn default() -> ExpressionArgs<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::expr::ExpressionArgs", "path": "ExpressionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 24], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/expr.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-719235e074bab8b26d9a4b14"></a>
## fmt

`function` · `datafusion_functions_window_common::expr::ExpressionArgs::fmt` · datafusion-functions-window-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::expr::ExpressionArgs", "path": "ExpressionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49dd2ae28879dcc427e7de8e"></a>
## input_exprs

`function` · `datafusion_functions_window_common::expr::ExpressionArgs::input_exprs` · datafusion-functions-window-common 55.1.0

```rust
fn input_exprs(&self) -> &'a [Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::expr::ExpressionArgs", "path": "ExpressionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [63, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns the expressions passed as arguments to the user-defined
window function.

<a id="op-2761f7e116ad35dcd1919956"></a>
## input_fields

`function` · `datafusion_functions_window_common::expr::ExpressionArgs::input_fields` · datafusion-functions-window-common 55.1.0

```rust
fn input_fields(&self) -> &'a [FieldRef]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::expr::ExpressionArgs", "path": "ExpressionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [63, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)s corresponding to the input expressions
to the user-defined window function.

<a id="op-0c9cc0926fb6b22d57c8698b"></a>
## new

`function` · `datafusion_functions_window_common::expr::ExpressionArgs::new` · datafusion-functions-window-common 55.1.0

```rust
fn new(input_exprs: &'a [Arc<dyn PhysicalExpr>], input_fields: &'a [FieldRef]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::expr::ExpressionArgs", "path": "ExpressionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [63, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Create an instance of [`ExpressionArgs`](../operations/datafusion_functions_window_common.expr.ExpressionArgs.md#op-2c48b2a7121c7066966c54e9).

# Arguments

* `input_exprs` - The expressions passed as arguments
  to the user-defined window function.
* `input_fields` - The fields corresponding to the
  arguments to the user-defined window function.
