# `datafusion_expr::expr::InList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.InList.json).

<a id="op-030eee80e5d2a8ff2e0071a6"></a>
## InList

`struct` · `datafusion_expr::expr::InList` · datafusion-expr 55.1.0

```rust
struct InList
```

Source: `src/expr.rs:1362`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

InList expression

<a id="op-0593bfc6c86eb6163cab9bee"></a>
## clone

`function` · `datafusion_expr::expr::InList::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> InList
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InList", "path": "InList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 10], "end": [1361, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3a22e2c00feb2a6ada719ab"></a>
## eq

`function` · `datafusion_expr::expr::InList::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &InList) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InList", "path": "InList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 17], "end": [1361, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ea7fd583793e14e823b6e09"></a>
## expr

`struct_field` · `datafusion_expr::expr::InList::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:1364`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression to compare

<a id="op-a4c1222d5d7ea42cf7d4df27"></a>
## fmt

`function` · `datafusion_expr::expr::InList::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InList", "path": "InList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 50], "end": [1361, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e2391db67639e5d521e0de"></a>
## hash

`function` · `datafusion_expr::expr::InList::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InList", "path": "InList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 44], "end": [1361, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fc261854f739856aa592968"></a>
## list

`struct_field` · `datafusion_expr::expr::InList::list` · datafusion-expr 55.1.0

```rust
list: Vec<Expr>
```

Source: `src/expr.rs:1366`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The list of values to compare against

<a id="op-1058aed3775fed55960a9871"></a>
## negated

`struct_field` · `datafusion_expr::expr::InList::negated` · datafusion-expr 55.1.0

```rust
negated: bool
```

Source: `src/expr.rs:1368`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the expression is negated

<a id="op-b6020e6005865f1c4589990e"></a>
## new

`function` · `datafusion_expr::expr::InList::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Box<Expr>, list: Vec<Expr>, negated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InList", "path": "InList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1371, 1], "end": [1380, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1373`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new InList expression

<a id="op-3cffc4436ae7513b97fdc983"></a>
## partial_cmp

`function` · `datafusion_expr::expr::InList::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &InList) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InList", "path": "InList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 32], "end": [1361, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1361`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
