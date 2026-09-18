# `datafusion_expr::expr::WindowFunctionParams`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.WindowFunctionParams.json).

<a id="op-134ec846aee751e27a9bbe18"></a>
## WindowFunctionParams

`struct` · `datafusion_expr::expr::WindowFunctionParams` · datafusion-expr 55.1.0

```rust
struct WindowFunctionParams
```

Source: `src/expr.rs:1253`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-616c8091af271b1f8abe9d2b"></a>
## args

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::args` · datafusion-expr 55.1.0

```rust
args: Vec<Expr>
```

Source: `src/expr.rs:1255`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List of expressions to feed to the functions as arguments

<a id="op-642bd7ee0cd4e388cba0cfe6"></a>
## clone

`function` · `datafusion_expr::expr::WindowFunctionParams::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFunctionParams
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionParams", "path": "WindowFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 10], "end": [1252, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c57770855d8808bb4e9497dc"></a>
## distinct

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::distinct` · datafusion-expr 55.1.0

```rust
distinct: bool
```

Source: `src/expr.rs:1267`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Distinct flag

<a id="op-79edba3276e6e4384be51ce6"></a>
## eq

`function` · `datafusion_expr::expr::WindowFunctionParams::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WindowFunctionParams) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionParams", "path": "WindowFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 17], "end": [1252, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-877faaf0f4730f097f3b3299"></a>
## filter

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::filter` · datafusion-expr 55.1.0

```rust
filter: Option<Box<Expr>>
```

Source: `src/expr.rs:1263`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional filter expression (FILTER (WHERE ...))

<a id="op-d2e8de82fd588b2541bf6e0b"></a>
## fmt

`function` · `datafusion_expr::expr::WindowFunctionParams::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionParams", "path": "WindowFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 50], "end": [1252, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dc3a2f6db1cd445536d6e67"></a>
## hash

`function` · `datafusion_expr::expr::WindowFunctionParams::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionParams", "path": "WindowFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 44], "end": [1252, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee61991ce77ad8eade209b90"></a>
## null_treatment

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::null_treatment` · datafusion-expr 55.1.0

```rust
null_treatment: Option<NullTreatment>
```

Source: `src/expr.rs:1265`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Specifies how NULL value is treated: ignore or respect

<a id="op-b6cc90d0f800bb8f7d75da2b"></a>
## order_by

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::order_by` · datafusion-expr 55.1.0

```rust
order_by: Vec<Sort>
```

Source: `src/expr.rs:1259`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List of order by expressions

<a id="op-ceed1a21af9751dc7197f9f9"></a>
## partial_cmp

`function` · `datafusion_expr::expr::WindowFunctionParams::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowFunctionParams) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionParams", "path": "WindowFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 32], "end": [1252, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c6cf39f5cffc4b3b8b45e4b"></a>
## partition_by

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::partition_by` · datafusion-expr 55.1.0

```rust
partition_by: Vec<Expr>
```

Source: `src/expr.rs:1257`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List of partition by expressions

<a id="op-606dfcc9eccef71e59f78d10"></a>
## window_frame

`struct_field` · `datafusion_expr::expr::WindowFunctionParams::window_frame` · datafusion-expr 55.1.0

```rust
window_frame: WindowFrame
```

Source: `src/expr.rs:1261`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Window frame
