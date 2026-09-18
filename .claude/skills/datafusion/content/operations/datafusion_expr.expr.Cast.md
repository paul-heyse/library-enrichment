# `datafusion_expr::expr::Cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Cast.json).

<a id="op-6c25ed2624858ad5a2193778"></a>
## Cast

`struct` · `datafusion_expr::expr::Cast` · datafusion-expr 55.1.0

```rust
struct Cast
```

Source: `src/expr.rs:989`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Cast expression

<a id="op-417ccaf087247bde2d745ca8"></a>
## clone

`function` · `datafusion_expr::expr::Cast::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Cast
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [988, 10], "end": [988, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:988`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27eef39d5be29b5add3d05cb"></a>
## eq

`function` · `datafusion_expr::expr::Cast::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Cast) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [988, 17], "end": [988, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:988`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-461db6064ef0a44705527b5a"></a>
## expr

`struct_field` · `datafusion_expr::expr::Cast::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:991`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression being cast

<a id="op-304b91da93593454f86317f0"></a>
## field

`struct_field` · `datafusion_expr::expr::Cast::field` · datafusion-expr 55.1.0

```rust
field: arrow::datatypes::FieldRef
```

Source: `src/expr.rs:993`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The `DataType` the expression will yield

<a id="op-7879b1a1918ffcef4367668c"></a>
## fmt

`function` · `datafusion_expr::expr::Cast::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [988, 50], "end": [988, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:988`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cc182d467e23a9fa310ee5b"></a>
## hash

`function` · `datafusion_expr::expr::Cast::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [988, 44], "end": [988, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:988`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d8e82fdd368a066131e4b07"></a>
## new

`function` · `datafusion_expr::expr::Cast::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Box<Expr>, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [996, 1], "end": [1008, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:998`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Cast expression

<a id="op-efa8a70f68e1e5ebc727cd86"></a>
## new_from_field

`function` · `datafusion_expr::expr::Cast::new_from_field` · datafusion-expr 55.1.0

```rust
fn new_from_field(expr: Box<Expr>, field: FieldRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [996, 1], "end": [1008, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1005`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e49db2d3759983383672069"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Cast::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Cast) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Cast", "path": "Cast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [988, 32], "end": [988, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:988`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
