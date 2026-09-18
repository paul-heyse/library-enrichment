# `datafusion_expr::expr::Between`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Between.json).

<a id="op-dd9e21677b867f196a8aed69"></a>
## Between

`struct` · `datafusion_expr::expr::Between` · datafusion-expr 55.1.0

```rust
struct Between
```

Source: `src/expr.rs:922`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

BETWEEN expression

<a id="op-113cccd23aa95c022c8fba96"></a>
## clone

`function` · `datafusion_expr::expr::Between::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Between
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Between", "path": "Between"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 10], "end": [921, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e70e276852481f5933e275b"></a>
## eq

`function` · `datafusion_expr::expr::Between::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Between) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Between", "path": "Between"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 17], "end": [921, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cd4f2e18f44caa181a7d695"></a>
## expr

`struct_field` · `datafusion_expr::expr::Between::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:924`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The value to compare

<a id="op-bc9328471159e2a91bee93de"></a>
## fmt

`function` · `datafusion_expr::expr::Between::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Between", "path": "Between"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 50], "end": [921, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e86d482a5cd846c564cbc78"></a>
## hash

`function` · `datafusion_expr::expr::Between::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Between", "path": "Between"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 44], "end": [921, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c81f0a343b21e405e9a336bf"></a>
## high

`struct_field` · `datafusion_expr::expr::Between::high` · datafusion-expr 55.1.0

```rust
high: Box<Expr>
```

Source: `src/expr.rs:930`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The high end of the range

<a id="op-812bdce56a1bb3a3b42521bb"></a>
## low

`struct_field` · `datafusion_expr::expr::Between::low` · datafusion-expr 55.1.0

```rust
low: Box<Expr>
```

Source: `src/expr.rs:928`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The low end of the range

<a id="op-63d864d77d3e1e663a616572"></a>
## negated

`struct_field` · `datafusion_expr::expr::Between::negated` · datafusion-expr 55.1.0

```rust
negated: bool
```

Source: `src/expr.rs:926`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the expression is negated

<a id="op-4c2007f8201492b5c9028774"></a>
## new

`function` · `datafusion_expr::expr::Between::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Box<Expr>, negated: bool, low: Box<Expr>, high: Box<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Between", "path": "Between"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [933, 1], "end": [943, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:935`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Between expression

<a id="op-849514b8c581fa1d3fa33773"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Between::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Between) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Between", "path": "Between"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 32], "end": [921, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
