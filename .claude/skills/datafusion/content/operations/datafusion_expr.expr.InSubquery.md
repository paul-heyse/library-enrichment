# `datafusion_expr::expr::InSubquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.InSubquery.json).

<a id="op-da1137b6b71e95cd22f3b6f4"></a>
## InSubquery

`struct` · `datafusion_expr::expr::InSubquery` · datafusion-expr 55.1.0

```rust
struct InSubquery
```

Source: `src/expr.rs:1384`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

IN subquery

<a id="op-2b60bae8d3bb475b694423cd"></a>
## clone

`function` · `datafusion_expr::expr::InSubquery::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> InSubquery
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InSubquery", "path": "InSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1383, 10], "end": [1383, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-156fa37539915927ad4eb5bf"></a>
## eq

`function` · `datafusion_expr::expr::InSubquery::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &InSubquery) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InSubquery", "path": "InSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1383, 17], "end": [1383, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd338268a51ec22fba2db5b3"></a>
## expr

`struct_field` · `datafusion_expr::expr::InSubquery::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:1386`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression to compare

<a id="op-eca3c0587b7117b16496b97f"></a>
## fmt

`function` · `datafusion_expr::expr::InSubquery::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InSubquery", "path": "InSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1383, 50], "end": [1383, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3841123957ffafb5a5290b5a"></a>
## hash

`function` · `datafusion_expr::expr::InSubquery::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InSubquery", "path": "InSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1383, 44], "end": [1383, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f36ab5cc776e671971221bc"></a>
## negated

`struct_field` · `datafusion_expr::expr::InSubquery::negated` · datafusion-expr 55.1.0

```rust
negated: bool
```

Source: `src/expr.rs:1390`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the expression is negated

<a id="op-0cd482c5a9ccb8b3fcaa71b6"></a>
## new

`function` · `datafusion_expr::expr::InSubquery::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Box<Expr>, subquery: Subquery, negated: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InSubquery", "path": "InSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1393, 1], "end": [1402, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1395`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new InSubquery expression

<a id="op-3a275ff931ad7df1af214a00"></a>
## partial_cmp

`function` · `datafusion_expr::expr::InSubquery::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &InSubquery) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::InSubquery", "path": "InSubquery"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1383, 32], "end": [1383, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ab7237fdf657cf9eee36ee8"></a>
## subquery

`struct_field` · `datafusion_expr::expr::InSubquery::subquery` · datafusion-expr 55.1.0

```rust
subquery: logical_plan::Subquery
```

Source: `src/expr.rs:1388`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Subquery that will produce a single column of data to compare against
