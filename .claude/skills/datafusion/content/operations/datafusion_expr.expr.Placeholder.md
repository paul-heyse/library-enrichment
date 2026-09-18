# `datafusion_expr::expr::Placeholder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Placeholder.json).

<a id="op-3028b83eb4b587de23f87f69"></a>
## Placeholder

`struct` · `datafusion_expr::expr::Placeholder` · datafusion-expr 55.1.0

```rust
struct Placeholder
```

Source: `src/expr.rs:1409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Placeholder, representing bind parameter values such as `$1` or `$name`.

The type of these parameters is inferred using [`Expr::infer_placeholder_types`](../operations/datafusion_expr.expr.Expr.md#op-db4bc32e974f4349ede47fce)
or can be specified directly using `PREPARE` statements.

<a id="op-da306dfc0d8653bea869353c"></a>
## clone

`function` · `datafusion_expr::expr::Placeholder::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Placeholder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 10], "end": [1408, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9bc5a2080bca82c5c925367"></a>
## eq

`function` · `datafusion_expr::expr::Placeholder::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Placeholder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 17], "end": [1408, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d2d4a8e73d144ef6656442"></a>
## field

`struct_field` · `datafusion_expr::expr::Placeholder::field` · datafusion-expr 55.1.0

```rust
field: Option<arrow::datatypes::FieldRef>
```

Source: `src/expr.rs:1413`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The type the parameter will be filled in with

<a id="op-632264271f63550a2ccdcad6"></a>
## fmt

`function` · `datafusion_expr::expr::Placeholder::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 50], "end": [1408, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4577bdda89c731753673dbab"></a>
## hash

`function` · `datafusion_expr::expr::Placeholder::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 44], "end": [1408, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da1bdf89e3bf3874b8dfcc7d"></a>
## id

`struct_field` · `datafusion_expr::expr::Placeholder::id` · datafusion-expr 55.1.0

```rust
id: String
```

Source: `src/expr.rs:1411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The identifier of the parameter, including the leading `$` (e.g, `"$1"` or `"$foo"`)

<a id="op-dbd24d200a719e9c35377a83"></a>
## new

`function` · `datafusion_expr::expr::Placeholder::new` · datafusion-expr 55.1.0

```rust
fn new(id: String, data_type: Option<DataType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1416, 1], "end": [1430, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1419`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Placeholder expression

<a id="op-abe770629c474741e61651a4"></a>
## new_with_field

`function` · `datafusion_expr::expr::Placeholder::new_with_field` · datafusion-expr 55.1.0

```rust
fn new_with_field(id: String, field: Option<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1416, 1], "end": [1430, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1427`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Placeholder expression from a Field

<a id="op-fcfb20a78d2fcbb697171a6b"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Placeholder::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Placeholder) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Placeholder", "path": "Placeholder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 32], "end": [1408, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
