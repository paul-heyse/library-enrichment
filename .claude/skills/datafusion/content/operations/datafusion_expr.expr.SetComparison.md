# `datafusion_expr::expr::SetComparison`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.SetComparison.json).

<a id="op-f147f4825e6b2e2445c34c3c"></a>
## SetComparison

`struct` · `datafusion_expr::expr::SetComparison` · datafusion-expr 55.1.0

```rust
struct SetComparison
```

Source: `src/expr.rs:1332`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set comparison subquery (e.g. `= ANY`, `> ALL`)

<a id="op-e848200d866dd89a8f091a66"></a>
## clone

`function` · `datafusion_expr::expr::SetComparison::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SetComparison
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetComparison", "path": "SetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1331, 10], "end": [1331, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3d075abe383a9038953f98f"></a>
## eq

`function` · `datafusion_expr::expr::SetComparison::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SetComparison) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetComparison", "path": "SetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1331, 17], "end": [1331, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-594e68d12dc9c0add43fcafc"></a>
## expr

`struct_field` · `datafusion_expr::expr::SetComparison::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:1334`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression to compare

<a id="op-7fc1e2c7942181b35058ef3d"></a>
## fmt

`function` · `datafusion_expr::expr::SetComparison::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetComparison", "path": "SetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1331, 50], "end": [1331, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bafa2bd0041dc6eb39543a6f"></a>
## hash

`function` · `datafusion_expr::expr::SetComparison::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetComparison", "path": "SetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1331, 44], "end": [1331, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-267188fd82df5307bf5ad535"></a>
## new

`function` · `datafusion_expr::expr::SetComparison::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Box<Expr>, subquery: Subquery, op: Operator, quantifier: SetQuantifier) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetComparison", "path": "SetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 1], "end": [1358, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1345`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new set comparison expression

<a id="op-6e4f88ea15943316354a78e5"></a>
## op

`struct_field` · `datafusion_expr::expr::SetComparison::op` · datafusion-expr 55.1.0

```rust
op: Operator
```

Source: `src/expr.rs:1338`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Comparison operator (e.g. `=`, `>`, `<`)

<a id="op-a6e273f21dee5c0f921a4a12"></a>
## partial_cmp

`function` · `datafusion_expr::expr::SetComparison::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &SetComparison) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetComparison", "path": "SetComparison"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1331, 32], "end": [1331, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f6b6f76c93f281e307ef157"></a>
## quantifier

`struct_field` · `datafusion_expr::expr::SetComparison::quantifier` · datafusion-expr 55.1.0

```rust
quantifier: SetQuantifier
```

Source: `src/expr.rs:1340`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Quantifier (`ANY`/`ALL`)

<a id="op-07ab19c5290402094a822f30"></a>
## subquery

`struct_field` · `datafusion_expr::expr::SetComparison::subquery` · datafusion-expr 55.1.0

```rust
subquery: logical_plan::Subquery
```

Source: `src/expr.rs:1336`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Subquery that will produce a single column of data to compare against
