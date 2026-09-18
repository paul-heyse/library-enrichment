# `datafusion_expr::expr::Unnest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Unnest.json).

<a id="op-601b2863f401501f84f1307d"></a>
## Unnest

`struct` · `datafusion_expr::expr::Unnest` · datafusion-expr 55.1.0

```rust
struct Unnest
```

Source: `src/expr.rs:680`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

UNNEST expression.

When `outer` is `true`, the unnest should preserve `NULL` and empty input
lists by emitting a single `NULL` output row for each. When `false` (the
historical default), the behavior is identical to the plain `UNNEST(col)`
SQL form: `NULL` and empty input lists are dropped from the output.

<a id="op-2be1b06098de94735e545379"></a>
## clone

`function` · `datafusion_expr::expr::Unnest::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Unnest
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 10], "end": [679, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92401a68a28b05165d06fc01"></a>
## eq

`function` · `datafusion_expr::expr::Unnest::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Unnest) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 17], "end": [679, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fbf5f76c1624300d230dd15"></a>
## expr

`struct_field` · `datafusion_expr::expr::Unnest::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:681`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dacebd1510bed1b3add8d314"></a>
## fmt

`function` · `datafusion_expr::expr::Unnest::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 50], "end": [679, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9691627ec7766d78b12cd99d"></a>
## hash

`function` · `datafusion_expr::expr::Unnest::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 44], "end": [679, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efcb97c42c274b2b2fbf011d"></a>
## new

`function` · `datafusion_expr::expr::Unnest::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Expr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [712, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Unnest expression with default (non-outer) semantics.

<a id="op-7ec7f1bda33938a355604231"></a>
## new_boxed

`function` · `datafusion_expr::expr::Unnest::new_boxed` · datafusion-expr 55.1.0

```rust
fn new_boxed(boxed: Box<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [712, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Unnest expression with default (non-outer) semantics.

<a id="op-9a7db134d6d0456421e9fe77"></a>
## new_outer

`function` · `datafusion_expr::expr::Unnest::new_outer` · datafusion-expr 55.1.0

```rust
fn new_outer(expr: Expr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [687, 1], "end": [712, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:706`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Unnest expression with outer-unnest semantics: `NULL`
and empty input lists each produce a single `NULL` output row.

<a id="op-871b0fbe7c1a40a16372d806"></a>
## outer

`struct_field` · `datafusion_expr::expr::Unnest::outer` · datafusion-expr 55.1.0

```rust
outer: bool
```

Source: `src/expr.rs:684`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Outer-unnest behavior: also expand empty input lists into a single
`NULL` output row (in addition to preserving `NULL` input rows).

<a id="op-b396652eddd7624f76f7efea"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Unnest::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Unnest) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Unnest", "path": "Unnest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 32], "end": [679, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
