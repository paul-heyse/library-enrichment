# `sqlparser::ast::query::LateralView`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.LateralView.json).

<a id="op-ac00e00a862be257dad1d941"></a>
## LateralView

`struct` · `sqlparser::ast::query::LateralView` · sqlparser 0.62.0

```rust
struct LateralView
```

Source: `src/ast/query.rs:664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A hive LATERAL VIEW with potential column aliases

<a id="op-0744e71ec744f9b40e5d5bc8"></a>
## clone

`function` · `sqlparser::ast::query::LateralView::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LateralView
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 17], "end": [661, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31b0720a9d532620bbc2d869"></a>
## cmp

`function` · `sqlparser::ast::query::LateralView::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LateralView) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 51], "end": [661, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27fcc632792c7c0fdf74efb9"></a>
## deserialize

`function` · `sqlparser::ast::query::LateralView::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 49], "end": [662, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a454fe36823a935ec8c10943"></a>
## eq

`function` · `sqlparser::ast::query::LateralView::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LateralView) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 24], "end": [661, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24baeed4953e2a6407af5e93"></a>
## fmt

`function` · `sqlparser::ast::query::LateralView::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 10], "end": [661, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-759052ac1518a3bd7d5d9552"></a>
## fmt

`function` · `sqlparser::ast::query::LateralView::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [693, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:676`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4483025697a748d5309294f6"></a>
## hash

`function` · `sqlparser::ast::query::LateralView::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 56], "end": [661, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c00f80470d9707c35b3a489f"></a>
## lateral_col_alias

`struct_field` · `sqlparser::ast::query::LateralView::lateral_col_alias` · sqlparser 0.62.0

```rust
lateral_col_alias: Vec<Ident>
```

Source: `src/ast/query.rs:670`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LATERAL VIEW optional column aliases

<a id="op-3297548619e71a6cee484738"></a>
## lateral_view

`struct_field` · `sqlparser::ast::query::LateralView::lateral_view` · sqlparser 0.62.0

```rust
lateral_view: Expr
```

Source: `src/ast/query.rs:666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LATERAL VIEW

<a id="op-8dec575d18a344658969f7a8"></a>
## lateral_view_name

`struct_field` · `sqlparser::ast::query::LateralView::lateral_view_name` · sqlparser 0.62.0

```rust
lateral_view_name: ObjectName
```

Source: `src/ast/query.rs:668`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LATERAL VIEW table name

<a id="op-7ee966133bebe7ddbfa30566"></a>
## outer

`struct_field` · `sqlparser::ast::query::LateralView::outer` · sqlparser 0.62.0

```rust
outer: bool
```

Source: `src/ast/query.rs:672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LATERAL VIEW OUTER

<a id="op-586ff9644c9ae1671a584c2f"></a>
## partial_cmp

`function` · `sqlparser::ast::query::LateralView::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LateralView) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [661, 35], "end": [661, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f3ffdea3499b2876192ed0"></a>
## serialize

`function` · `sqlparser::ast::query::LateralView::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [662, 38], "end": [662, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-236181bf6723ad9847b1fdcf"></a>
## span

`function` · `sqlparser::ast::query::LateralView::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "super::LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2365, 1], "end": [2380, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60dac47e3dde2b4d576ea803"></a>
## visit

`function` · `sqlparser::ast::query::LateralView::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 40], "end": [663, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7a150abd2b9c18244df6fba"></a>
## visit

`function` · `sqlparser::ast::query::LateralView::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LateralView", "path": "LateralView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [663, 47], "end": [663, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
