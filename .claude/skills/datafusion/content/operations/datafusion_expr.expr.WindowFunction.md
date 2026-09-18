# `datafusion_expr::expr::WindowFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.WindowFunction.json).

<a id="op-8f5e8e8a659430c3e7448f3a"></a>
## WindowFunction

`struct` · `datafusion_expr::expr::WindowFunction` · datafusion-expr 55.1.0

```rust
struct WindowFunction
```

Source: `src/expr.rs:1246`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Window function

Holds the actual function to call [`WindowFunction`](../operations/datafusion_expr.expr.WindowFunction.md#op-8f5e8e8a659430c3e7448f3a) as well as its
arguments (`args`) and the contents of the `OVER` clause:

1. `PARTITION BY`
2. `ORDER BY`
3. Window frame (e.g. `ROWS 1 PRECEDING AND 1 FOLLOWING`)

See [`ExprFunctionExt`] for examples of how to create a `WindowFunction`.

[`ExprFunctionExt`]: crate::ExprFunctionExt

<a id="op-675cee2a417953894b8bbe10"></a>
## clone

`function` · `datafusion_expr::expr::WindowFunction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1245, 10], "end": [1245, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a10cf2ef8437063c0d51e5c"></a>
## eq

`function` · `datafusion_expr::expr::WindowFunction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WindowFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1245, 17], "end": [1245, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ebab04a2f4e1374b07e6d7d"></a>
## fmt

`function` · `datafusion_expr::expr::WindowFunction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1245, 50], "end": [1245, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4352a78fc0cd9d7a7e111ae6"></a>
## fun

`struct_field` · `datafusion_expr::expr::WindowFunction::fun` · datafusion-expr 55.1.0

```rust
fun: WindowFunctionDefinition
```

Source: `src/expr.rs:1248`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Name of the function

<a id="op-cbd37f5a88f4dcc5589dff84"></a>
## hash

`function` · `datafusion_expr::expr::WindowFunction::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1245, 44], "end": [1245, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e717d81a7e368791d9da6c37"></a>
## new

`function` · `datafusion_expr::expr::WindowFunction::new` · datafusion-expr 55.1.0

```rust
fn new(fun: impl Into<WindowFunctionDefinition>, args: Vec<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1270, 1], "end": [1294, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Window expression with the specified argument an
empty `OVER` clause

<a id="op-8aa6ece9642d820e647a9137"></a>
## params

`struct_field` · `datafusion_expr::expr::WindowFunction::params` · datafusion-expr 55.1.0

```rust
params: WindowFunctionParams
```

Source: `src/expr.rs:1249`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-627de0d46e625c1a7e7f3418"></a>
## partial_cmp

`function` · `datafusion_expr::expr::WindowFunction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1245, 32], "end": [1245, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adfac8a034a17512224e5202"></a>
## simplify

`function` · `datafusion_expr::expr::WindowFunction::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self) -> Option<WindowFunctionSimplification>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunction", "path": "WindowFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1270, 1], "end": [1294, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1291`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this window function's simplification hook, if any.

See [`WindowFunctionSimplification`](../operations/datafusion_expr.function.WindowFunctionSimplification.md#op-4e3825e9eb717b05a23f2e0d) for more information
