# `datafusion_expr::logical_plan::plan::Window`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Window.json).

<a id="op-7386d6673f6d599d6f92aec3"></a>
## Window

`struct` · `datafusion_expr::logical_plan::plan::Window` · datafusion-expr 55.1.0

```rust
struct Window
```

Source: `src/logical_plan/plan.rs:2779`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Window its input based on a set of window spec and window function (e.g. SUM or RANK)

# Output Schema

The output schema is the input schema followed by the window function
expressions, in order.

For example, given the input schema `"A", "B", "C"` and the window function
`SUM(A) OVER (PARTITION BY B+1 ORDER BY C)`, the output schema will be `"A",
"B", "C", "SUM(A) OVER ..."` where `"SUM(A) OVER ..."` is the name of the
output column.

Note that the `PARTITION BY` expression "B+1" is not produced in the output
schema.

<a id="op-9bdc4c0a4d0018af510ecdaa"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Window::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Window
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2778, 17], "end": [2778, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af76dbc567b56507fb63d0f8"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Window::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Window) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2778, 24], "end": [2778, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aa021c4e790ca0c25814fe6"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Window::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2778, 10], "end": [2778, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d1701d20f04c4f3f60b77e5"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Window::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2778, 39], "end": [2778, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3199d40f7097cf8e29f3a43"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Window::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:2781`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-cf303f17fcacfadd0d8978a0"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Window::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2904, 1], "end": [2924, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2905`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16bdfe3af8acb5c38a4eb47e"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Window::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2785`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema description of the window output

<a id="op-0efb856829dc84c17aa10d19"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Window::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(window_expr: Vec<Expr>, input: Arc<LogicalPlan>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2788, 1], "end": [2901, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2790`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new window operator.

<a id="op-8b611b5b61d83b0cc9ac4e4f"></a>
## try_new_with_schema

`function` · `datafusion_expr::logical_plan::plan::Window::try_new_with_schema` · datafusion-expr 55.1.0

```rust
fn try_new_with_schema(window_expr: Vec<Expr>, input: Arc<LogicalPlan>, schema: DFSchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Window", "path": "Window"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2788, 1], "end": [2901, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2881`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new window function using the provided schema to avoid the overhead of
building the schema again when the schema is already known.

This method should only be called when you are absolutely sure that the schema being
provided is correct for the window function. If in doubt, call [try_new](Self::try_new) instead.

<a id="op-ef8e5128b5027c88c9994dec"></a>
## window_expr

`struct_field` · `datafusion_expr::logical_plan::plan::Window::window_expr` · datafusion-expr 55.1.0

```rust
window_expr: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:2783`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The window function expression
