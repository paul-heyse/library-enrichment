# `datafusion_expr::expr::Exists`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Exists.json).

<a id="op-328ba38f274d44243e325547"></a>
## Exists

`struct` · `datafusion_expr::expr::Exists` · datafusion-expr 55.1.0

```rust
struct Exists
```

Source: `src/expr.rs:1298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

EXISTS expression

<a id="op-6396a4893aab2d176341e1b2"></a>
## clone

`function` · `datafusion_expr::expr::Exists::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Exists
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Exists", "path": "Exists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1297, 10], "end": [1297, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-243e44abd0f38ed1a055e3fc"></a>
## eq

`function` · `datafusion_expr::expr::Exists::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Exists) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Exists", "path": "Exists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1297, 17], "end": [1297, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7a5b3b32a4ae742634d2a47"></a>
## fmt

`function` · `datafusion_expr::expr::Exists::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Exists", "path": "Exists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1297, 50], "end": [1297, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-891cd1989ad2d23e5bc1e842"></a>
## hash

`function` · `datafusion_expr::expr::Exists::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Exists", "path": "Exists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1297, 44], "end": [1297, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66f91d04f9550fe05b6089a9"></a>
## negated

`struct_field` · `datafusion_expr::expr::Exists::negated` · datafusion-expr 55.1.0

```rust
negated: bool
```

Source: `src/expr.rs:1302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the expression is negated

<a id="op-0fb097e0fb46675574fbd3e0"></a>
## new

`function` · `datafusion_expr::expr::Exists::new` · datafusion-expr 55.1.0

```rust
fn new(subquery: Subquery, negated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Exists", "path": "Exists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1305, 1], "end": [1310, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-954200a1430d91a83f6164f6"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Exists::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Exists) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Exists", "path": "Exists"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1297, 32], "end": [1297, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d87d67c718a9efcb385cc038"></a>
## subquery

`struct_field` · `datafusion_expr::expr::Exists::subquery` · datafusion-expr 55.1.0

```rust
subquery: logical_plan::Subquery
```

Source: `src/expr.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Subquery that will produce a single column of data
