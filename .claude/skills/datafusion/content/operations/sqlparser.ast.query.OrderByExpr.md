# `sqlparser::ast::query::OrderByExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.OrderByExpr.json).

<a id="op-9202f7394a1d5e3a01f7064a"></a>
## OrderByExpr

`struct` · `sqlparser::ast::query::OrderByExpr` · sqlparser 0.62.0

```rust
struct OrderByExpr
```

Source: `src/ast/query.rs:2928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ORDER BY` expression

<a id="op-f57526113d74fee882afbe57"></a>
## clone

`function` · `sqlparser::ast::query::OrderByExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OrderByExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2925, 17], "end": [2925, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a827a0951875ab321b3359c"></a>
## cmp

`function` · `sqlparser::ast::query::OrderByExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OrderByExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2925, 51], "end": [2925, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c4ee996033d751dc84373a7"></a>
## deserialize

`function` · `sqlparser::ast::query::OrderByExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2926, 49], "end": [2926, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09c6b78171e7c88758412724"></a>
## eq

`function` · `sqlparser::ast::query::OrderByExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OrderByExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2925, 24], "end": [2925, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd1e087f4a1efc80a5baaf79"></a>
## expr

`struct_field` · `sqlparser::ast::query::OrderByExpr::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:2930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression to order by.

<a id="op-fb2a75696455b4f2fa32619c"></a>
## fmt

`function` · `sqlparser::ast::query::OrderByExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2947, 1], "end": [2955, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc58b1f5a82ad2c98c5386b3"></a>
## fmt

`function` · `sqlparser::ast::query::OrderByExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2925, 10], "end": [2925, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caa49218a44929ad21951961"></a>
## from

`function` · `sqlparser::ast::query::OrderByExpr::from` · sqlparser 0.62.0

```rust
fn from(ident: Ident) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2937, 1], "end": [2945, 2], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/query.rs:2938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2d55c67c78549f0acade247"></a>
## hash

`function` · `sqlparser::ast::query::OrderByExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2925, 56], "end": [2925, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49a5cf8b62cca62fd158cbb5"></a>
## options

`struct_field` · `sqlparser::ast::query::OrderByExpr::options` · sqlparser 0.62.0

```rust
options: OrderByOptions
```

Source: `src/ast/query.rs:2932`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Ordering options such as `ASC`/`DESC` and `NULLS` behavior.

<a id="op-38dddd0e2f51709f84fa7678"></a>
## partial_cmp

`function` · `sqlparser::ast::query::OrderByExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OrderByExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2925, 35], "end": [2925, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a541566769f3dcb7ed95427"></a>
## serialize

`function` · `sqlparser::ast::query::OrderByExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2926, 38], "end": [2926, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af1f92c3c2848f70701eeb97"></a>
## span

`function` · `sqlparser::ast::query::OrderByExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "super::OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2122, 1], "end": [2132, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d724261dafe439947c26855a"></a>
## visit

`function` · `sqlparser::ast::query::OrderByExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2927, 40], "end": [2927, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2d666a840e8081f8c348887"></a>
## visit

`function` · `sqlparser::ast::query::OrderByExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByExpr", "path": "OrderByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2927, 47], "end": [2927, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adbfca6f78facc39df767887"></a>
## with_fill

`struct_field` · `sqlparser::ast::query::OrderByExpr::with_fill` · sqlparser 0.62.0

```rust
with_fill: Option<WithFill>
```

Source: `src/ast/query.rs:2934`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `WITH FILL` clause (ClickHouse extension) which specifies how to fill gaps.
