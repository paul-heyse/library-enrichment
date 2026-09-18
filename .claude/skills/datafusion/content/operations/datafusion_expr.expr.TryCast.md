# `datafusion_expr::expr::TryCast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.TryCast.json).

<a id="op-34ab54a88cc8d90de87a596b"></a>
## TryCast

`struct` · `datafusion_expr::expr::TryCast` · datafusion-expr 55.1.0

```rust
struct TryCast
```

Source: `src/expr.rs:1012`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

TryCast Expression

<a id="op-fcd714d08756ebd0f4a2ce23"></a>
## clone

`function` · `datafusion_expr::expr::TryCast::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TryCast
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1011, 10], "end": [1011, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1011`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad0ddde7d8a82509d959da8a"></a>
## eq

`function` · `datafusion_expr::expr::TryCast::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TryCast) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1011, 17], "end": [1011, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1011`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d5552b3abdb25c8e2412b4"></a>
## expr

`struct_field` · `datafusion_expr::expr::TryCast::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:1014`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression being cast

<a id="op-baa642db888510281d6485f8"></a>
## field

`struct_field` · `datafusion_expr::expr::TryCast::field` · datafusion-expr 55.1.0

```rust
field: arrow::datatypes::FieldRef
```

Source: `src/expr.rs:1016`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The `DataType` the expression will yield

<a id="op-de3fe92175bad572a1434af7"></a>
## fmt

`function` · `datafusion_expr::expr::TryCast::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1011, 50], "end": [1011, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1011`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-555266755c5f10093d8de57a"></a>
## hash

`function` · `datafusion_expr::expr::TryCast::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1011, 44], "end": [1011, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1011`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e9a2448823b7db0fff85683"></a>
## new

`function` · `datafusion_expr::expr::TryCast::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Box<Expr>, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1019, 1], "end": [1031, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1021`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new TryCast expression

<a id="op-7b6880e95199203c01590fc8"></a>
## new_from_field

`function` · `datafusion_expr::expr::TryCast::new_from_field` · datafusion-expr 55.1.0

```rust
fn new_from_field(expr: Box<Expr>, field: FieldRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1019, 1], "end": [1031, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1028`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f503ffb9306370fc7fd9963"></a>
## partial_cmp

`function` · `datafusion_expr::expr::TryCast::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &TryCast) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::TryCast", "path": "TryCast"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1011, 32], "end": [1011, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1011`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
