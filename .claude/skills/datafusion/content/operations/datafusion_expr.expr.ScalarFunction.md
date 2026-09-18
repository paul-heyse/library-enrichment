# `datafusion_expr::expr::ScalarFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.ScalarFunction.json).

<a id="op-98cb91546035ded90253119b"></a>
## ScalarFunction

`struct` · `datafusion_expr::expr::ScalarFunction` · datafusion-expr 55.1.0

```rust
struct ScalarFunction
```

Source: `src/expr.rs:949`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke a [`ScalarUDF`] with a set of arguments

[`ScalarUDF`]: crate::ScalarUDF

<a id="op-ee66b57c1eea8f21031b64b5"></a>
## args

`struct_field` · `datafusion_expr::expr::ScalarFunction::args` · datafusion-expr 55.1.0

```rust
args: Vec<Expr>
```

Source: `src/expr.rs:953`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List of expressions to feed to the functions as arguments

<a id="op-5b66b76bea8ad2d027c7456a"></a>
## clone

`function` · `datafusion_expr::expr::ScalarFunction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ScalarFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [948, 10], "end": [948, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:948`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56f8db6984748a7d943f4b8b"></a>
## eq

`function` · `datafusion_expr::expr::ScalarFunction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &ScalarFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [948, 17], "end": [948, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:948`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f144a96c6f482674dc66adbf"></a>
## fmt

`function` · `datafusion_expr::expr::ScalarFunction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [948, 50], "end": [948, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:948`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f357304fd3818417372dca82"></a>
## func

`struct_field` · `datafusion_expr::expr::ScalarFunction::func` · datafusion-expr 55.1.0

```rust
func: std::sync::Arc<ScalarUDF>
```

Source: `src/expr.rs:951`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The function

<a id="op-ff94549956cfa4821da98252"></a>
## hash

`function` · `datafusion_expr::expr::ScalarFunction::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [948, 44], "end": [948, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:948`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e6f038ed164f3b3d1ba4f9"></a>
## name

`function` · `datafusion_expr::expr::ScalarFunction::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [961, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:958`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-935918ea8235ef97652c0694"></a>
## new_udf

`function` · `datafusion_expr::expr::ScalarFunction::new_udf` · datafusion-expr 55.1.0

```rust
fn new_udf(udf: Arc<ScalarUDF>, args: Vec<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [963, 1], "end": [970, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:967`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `ScalarFunction` from a [`ScalarUDF`]

[`ScalarUDF`]: crate::ScalarUDF

<a id="op-9b3a9c915357dfacfd826cb9"></a>
## partial_cmp

`function` · `datafusion_expr::expr::ScalarFunction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &ScalarFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::ScalarFunction", "path": "ScalarFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [948, 32], "end": [948, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:948`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
